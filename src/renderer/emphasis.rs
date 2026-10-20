//! 한컴 Print의 강조점 글리프 역할과 표시 자소별 배치 결과.
use super::{render_tree::BoundingBox, text_direction::VisualTextPositions, TextStyle};
use unicode_segmentation::UnicodeSegmentation;

pub(crate) const FONT_FAMILY: &str = "HCR Batang";

pub(crate) fn glyph(kind: u8) -> Option<&'static str> {
    match kind {
        1 => Some("\u{0307}"),
        2 => Some("\u{030a}"),
        3 => Some("\u{030c}"),
        4 => Some("\u{0303}"),
        5 => Some("\u{302e}"),
        6 => Some("\u{302f}"),
        _ => None,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmphasisGeometry {
    pub font_family: &'static str,
    pub glyph: &'static str,
    pub font_size: f64,
    pub bold: bool,
    pub ratio: f64,
    pub skew_x: f64,
    pub baseline_offset: f64,
    pub positions: Vec<f64>,
    /// 브라우저의 방점 셰이핑에서 보조 원 삽입을 막는 잉크 없는 문맥.
    pub browser_text: String,
    pub browser_offset: f64,
}

impl EmphasisGeometry {
    /// clip/paint 범위에만 쓰는 보수적 em 상자이며 글줄·캐럿을 늘리지 않는다.
    pub fn ink_bounds(&self) -> Option<BoundingBox> {
        let left = self.positions.iter().copied().reduce(f64::min)?;
        let right = self.positions.iter().copied().reduce(f64::max)?;
        let pad = self.font_size * (self.ratio + self.skew_x.abs());
        Some(BoundingBox::new(
            left - pad,
            self.baseline_offset - self.font_size,
            right - left + pad * 1.25,
            self.font_size * 1.25,
        ))
    }
}

pub(crate) fn geometry(
    text: &str,
    positions: &[f64],
    style: &TextStyle,
    font_size: f64,
) -> Option<EmphasisGeometry> {
    let glyph = glyph(style.emphasis_dot)?;
    let ratio = style.ratio;
    if positions.len() != text.chars().count() + 1
        || positions.iter().any(|value| !value.is_finite())
        || !font_size.is_finite()
        || font_size <= 0.0
        || !ratio.is_finite()
        || ratio <= 0.0
    {
        return None;
    }
    let tone = style.emphasis_dot >= 5;
    let visual = VisualTextPositions::for_style(text, positions, style);
    let mut origins = Vec::new();
    let mut start = 0;
    for cluster in text.graphemes(true) {
        let end = start + cluster.chars().count();
        let width = positions[end] - positions[start];
        if width > 0.0
            && cluster
                .chars()
                .any(|ch| !ch.is_whitespace() && !ch.is_control() && ch != '\u{fffc}')
        {
            let left = visual
                .as_ref()
                .and_then(|v| v.span_origin(start, end))
                .unwrap_or(positions[start]);
            origins.push(
                left + if tone {
                    font_size * ratio * 0.89
                } else {
                    width / 2.0 + font_size * ratio * 0.225
                },
            );
        }
        start = end;
    }
    Some(EmphasisGeometry {
        font_family: FONT_FAMILY,
        glyph,
        font_size,
        bold: style.is_visually_bold(),
        ratio,
        skew_x: if style.italic { -0.333 } else { 0.0 },
        baseline_offset: if tone { 0.0 } else { -font_size * 0.37 },
        positions: origins,
        browser_text: if tone {
            format!("\u{115f}\u{1160}{glyph}")
        } else {
            glyph.to_owned()
        },
        browser_offset: if tone { -font_size * 0.5 } else { 0.0 },
    })
}
