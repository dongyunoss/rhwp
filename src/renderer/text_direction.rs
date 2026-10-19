//! 원문 주소·전진을 유지하면서 방향별 구간의 실제 표시 원점을 구한다.
use unicode_bidi::{bidi_class, BidiClass, BidiInfo, Level};

struct VisualRun {
    start: usize,
    end: usize,
    x: f64,
    rtl: bool,
}

pub(crate) struct VisualTextPositions<'a> {
    positions: &'a [f64],
    runs: Vec<VisualRun>,
}

impl<'a> VisualTextPositions<'a> {
    pub(crate) fn new(text: &str, positions: &'a [f64]) -> Option<Self> {
        if !text.chars().any(|ch| {
            matches!(
                bidi_class(ch),
                BidiClass::R | BidiClass::AL | BidiClass::RLE | BidiClass::RLO | BidiClass::RLI
            )
        }) {
            return None;
        }
        let mut offsets: Vec<_> = text.char_indices().map(|(at, _)| at).collect();
        offsets.push(text.len());
        if offsets.len() != positions.len() {
            return None;
        }
        // 현재 수평 문단의 기본 흐름은 LTR이다. 뒤 공백은 본문 바깥으로 뒤집지 않는다.
        let bidi = BidiInfo::new(text, Some(Level::ltr()));
        let mut runs = Vec::new();
        for para in &bidi.paragraphs {
            let (levels, visual_runs) = bidi.visual_runs(para, para.range.clone());
            let start = offsets.binary_search(&para.range.start).ok()?;
            let mut x = positions[start];
            for run in visual_runs {
                let start = offsets.binary_search(&run.start).ok()?;
                let end = offsets.binary_search(&run.end).ok()?;
                runs.push(VisualRun {
                    start,
                    end,
                    x,
                    rtl: levels[run.start].is_rtl(),
                });
                x += positions[end] - positions[start];
            }
        }
        Some(Self { positions, runs })
    }

    pub(crate) fn span_origin(&self, start: usize, end: usize) -> Option<f64> {
        let run = self
            .runs
            .iter()
            .find(|run| start >= run.start && end <= run.end)?;
        Some(if run.rtl {
            run.x + self.positions[run.end] - self.positions[end]
        } else {
            run.x + self.positions[start] - self.positions[run.start]
        })
    }
}
