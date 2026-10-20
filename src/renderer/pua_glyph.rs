//! 표준 옛한글 자소의 호환 PUA 글리프를 실제 공급 프로그램의 cmap로만 선택한다.
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

const FIRST: u32 = 0xe000;
const LAST: u32 = 0xf8ff;
const WORDS: usize = ((LAST - FIRST + 1) / 64) as usize;
const MAX_FACES: usize = 256;
const MAX_SOURCE_BYTES: usize = 32 * 1024 * 1024;

/// 모호한 역매핑은 선택하지 않는다. 원문·표시 문자열·문자 주소는 변경하지 않는다.
pub(crate) fn candidate(cluster: &str) -> Option<char> {
    if !super::contains_old_hangul_jamo(cluster) {
        return None;
    }
    static REVERSE: OnceLock<HashMap<String, Option<char>>> = OnceLock::new();
    REVERSE
        .get_or_init(|| {
            let mut map = HashMap::new();
            for cp in FIRST..=LAST {
                let ch = char::from_u32(cp).unwrap();
                if let Some(chars) = super::pua_oldhangul::map_pua_old_hangul(ch) {
                    let key: String = chars.iter().collect();
                    map.entry(key)
                        .and_modify(|value| *value = None)
                        .or_insert(Some(ch));
                }
            }
            map
        })
        .get(cluster)
        .copied()
        .flatten()
}

type FaceKey = (String, u16, bool);

/// 글꼴 바이트나 윤곽선을 복제하지 않고 공급 face별 가시 PUA 비트만 보관한다.
#[derive(Clone, Default)]
pub(crate) struct FontCoverage {
    faces: HashMap<FaceKey, Arc<[u64; WORDS]>>,
}

fn primary(family: &str) -> String {
    family
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches(['\'', '"'])
        .to_lowercase()
}

impl FontCoverage {
    pub(crate) fn register(
        &mut self,
        family: &str,
        bytes: &[u8],
        index: u32,
    ) -> Result<bool, &'static str> {
        if family.is_empty() || family.len() > 256 || family.chars().any(char::is_control) {
            return Err("invalid text font family");
        }
        if bytes.is_empty() || bytes.len() > MAX_SOURCE_BYTES {
            return Err("text font source byte limit exceeded");
        }
        let family = primary(family);
        let Ok(face) = ttf_parser::Face::parse(bytes, index) else {
            let count = self.faces.len();
            self.faces.retain(|(name, _, _), _| name != &family);
            return Ok(count != self.faces.len());
        };
        let key = (family, face.weight().to_number(), face.is_italic());
        if !self.faces.contains_key(&key) && self.faces.len() >= MAX_FACES {
            return Err("text font face limit exceeded");
        }
        let mut coverage = [0u64; WORDS];
        for cp in FIRST..=LAST {
            let Some(glyph) = face.glyph_index(char::from_u32(cp).unwrap()) else {
                continue;
            };
            // 이름·glyph ID 존재만으로 빈 GSUB 대상을 가시 글리프로 인정하지 않는다.
            if glyph.0 == 0 || face.glyph_bounding_box(glyph).is_none() {
                continue;
            }
            let bit = (cp - FIRST) as usize;
            coverage[bit / 64] |= 1 << (bit % 64);
        }
        if self
            .faces
            .get(&key)
            .is_some_and(|old| old.as_ref() == &coverage)
        {
            return Ok(false);
        }
        self.faces.insert(key, Arc::new(coverage));
        Ok(true)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn has_family(&self, family: &str) -> bool {
        let family = primary(family);
        self.faces.keys().any(|(name, _, _)| name == &family)
    }

    pub(crate) fn glyph(&self, family: &str, cluster: &str) -> Option<char> {
        let glyph = candidate(cluster)?;
        let family = primary(family);
        let bit = (glyph as u32 - FIRST) as usize;
        let mut found = false;
        for ((name, _, _), coverage) in &self.faces {
            if name != &family {
                continue;
            }
            found = true;
            // 실제 공급한 스타일들이 모두 같은 호환 글리프를 가질 때만 고른다.
            if coverage[bit / 64] & (1 << (bit % 64)) == 0 {
                return None;
            }
        }
        found.then_some(glyph)
    }
}

#[derive(serde::Serialize)]
pub(crate) struct Candidate {
    pub start: usize,
    pub end: usize,
    pub glyph: char,
}

pub(crate) fn candidates(text: &str) -> Vec<Candidate> {
    super::layout::split_into_clusters(text)
        .into_iter()
        .take(crate::paint::MAX_POSITIONED_CONTROL_MARKS_PER_RUN)
        .filter_map(|(start, cluster)| {
            candidate(&cluster).map(|glyph| Candidate {
                start,
                end: start + cluster.chars().count(),
                glyph,
            })
        })
        .collect()
}
