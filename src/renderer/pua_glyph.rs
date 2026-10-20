//! 표준 옛한글 자소의 호환 PUA 글리프를 실제 공급 프로그램의 cmap로만 선택한다.
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

const FIRST: u32 = 0xe000;
const LAST: u32 = 0xf8ff;
const WORDS: usize = ((LAST - FIRST + 1) / 64) as usize;
// 보조 cmap 비트만 보관한다. 실제 공급 catalog의 별칭·스타일도 유한하게 수용한다.
const MAX_FACES: usize = 4096;
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
    /// 미검증 스타일이 같은 family의 이전 양수 coverage를 재사용하지 않게 한다.
    /// 선택적 보조 정보의 상한 때문에 브라우저의 정상 글꼴 적재를 실패시키지 않는다.
    fn disable_family(&mut self, family: String) -> bool {
        let key = (family.clone(), 0, false);
        if self
            .faces
            .get(&key)
            .is_some_and(|mask| mask.iter().all(|word| *word == 0))
        {
            return false;
        }
        self.faces.retain(|(name, _, _), _| name != &family);
        if self.faces.len() < MAX_FACES {
            self.faces.insert(key, Arc::new([0; WORDS]));
        }
        false
    }

    pub(crate) fn register(
        &mut self,
        family: &str,
        bytes: &[u8],
        index: u32,
    ) -> Result<bool, &'static str> {
        if family.is_empty() || family.len() > 256 || family.chars().any(char::is_control) {
            return Err("invalid text font family");
        }
        let family = primary(family);
        if bytes.is_empty() {
            return Err("empty text font source");
        }
        if bytes.len() > MAX_SOURCE_BYTES {
            return Ok(self.disable_family(family));
        }
        let Ok(face) = ttf_parser::Face::parse(bytes, index) else {
            return Ok(self.disable_family(family));
        };
        let key = (family, face.weight().to_number(), face.is_italic());
        if !self.faces.contains_key(&key) && self.faces.len() >= MAX_FACES {
            return Ok(self.disable_family(key.0));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oversized_optional_program_does_not_abort_loading_or_reuse_stale_coverage() {
        let mut coverage = FontCoverage::default();
        coverage.faces.insert(
            ("source face".into(), 400, false),
            Arc::new([u64::MAX; WORDS]),
        );
        let text = "\u{1112}\u{119e}\u{11ab}";
        assert_eq!(coverage.glyph("Source Face", text), Some('\u{f53a}'));
        // 크기 상한은 보조 cmap 수집을 제한한다. 실제 FontFace 적재는 계속한다.
        let oversized = vec![0; MAX_SOURCE_BYTES + 1];
        assert_eq!(coverage.register("Source Face", &oversized, 0), Ok(false));
        assert_eq!(coverage.glyph("Source Face", text), None);
        // 다음 스타일을 알게 돼도 앞서 미검증인 스타일을 지워서 인정하지 않는다.
        coverage.faces.insert(
            ("source face".into(), 700, false),
            Arc::new([u64::MAX; WORDS]),
        );
        assert_eq!(coverage.glyph("Source Face", text), None);
    }
}
