//! 글자 장식(밑줄) 기하 — 한글 2022 실측 계약. (#5730)
//!
//! 종전에는 밑줄을 기준선 아래 **고정 2.0px** 에 그렸다. 한글 2022 COM 프로브
//! (9/12/15/18/24/36pt, 실선)로 실측하면 밑줄은 글꼴 크기에 비례해
//! **기준선 + 0.17em** 에 온다 (1.44/9=0.160, 2.04/12=0.170, 2.52/15=0.168,
//! 3.00/18=0.167, 4.07/24=0.170, 6.11/36=0.170). 고정 2.0px 은 11.8px 글꼴에서만
//! 우연히 맞고, 제목처럼 큰 글꼴에서는 밑줄이 디센더를 가로지른다
//! (156467175 실측: 24px 제목에서 한글 4.2px vs rhwp 0.8px).
//!
//! 이중/삼중선(shape 7~10)도 같은 프로브(15pt·24pt)로 선별 위치·굵기가 전부
//! em 비례임을 실측했다. 이 모듈이 그 계약의 단일 출처다 — SVG/Skia 백엔드가
//! 같은 표를 소비한다.

/// 단일선(및 파선 등 shape 0~6, 11)의 기준선 아래 오프셋 비율 (em).
pub(crate) const UNDERLINE_BASELINE_RATIO: f64 = 0.17;

/// 밑줄 다중선(shape 7~10)의 (기준선 아래 오프셋 em, 선 굵기 em) 목록.
///
/// 한글 2022 실측 (15pt/24pt 프로브, PDF 드로잉 좌표):
/// - 7 이중선: 0.160(0.032) + 0.250(0.032)
/// - 8 가는+굵은: 0.175(0.048) + 0.300(0.100)
/// - 9 굵은+가는: 0.200(0.100) + 0.325(0.048)
/// - 10 삼중선: 0.165(0.032) + 0.250(0.080) + 0.335(0.032)
///
/// 그 외 shape 는 `None` — 호출부가 단일선 경로(0.17em, 기존 선 모양 유지)로 그린다.
pub(crate) fn underline_multi_lines(shape: u8) -> Option<&'static [(f64, f64)]> {
    match shape {
        7 => Some(&[(0.160, 0.032), (0.250, 0.032)]),
        8 => Some(&[(0.175, 0.048), (0.300, 0.100)]),
        9 => Some(&[(0.200, 0.100), (0.325, 0.048)]),
        10 => Some(&[(0.165, 0.032), (0.250, 0.080), (0.335, 0.032)]),
        _ => None,
    }
}

/// 위 밑줄의 독립 Print 계약. 좌표는 기준선에 대한 오프셋이며 길이 단위는 레이아웃 px다.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DecorationStroke {
    pub offset: f64,
    pub width: f64,
    pub dash: Vec<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DecorationDots {
    pub offset: f64,
    pub radius: f64,
    pub step: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct TopUnderlineGeometry {
    pub strokes: Vec<DecorationStroke>,
    pub dots: Option<DecorationDots>,
}

impl TopUnderlineGeometry {
    /// 기준선에 대한 잉크의 위·아래 경계. 모든 출력과 clip이 같은 기하를 사용한다.
    pub fn vertical_bounds(&self) -> (f64, f64) {
        self.strokes.iter().fold(
            self.dots
                .as_ref()
                .map_or((f64::INFINITY, f64::NEG_INFINITY), |dot| {
                    (dot.offset - dot.radius, dot.offset + dot.radius)
                }),
            |(top, bottom), stroke| {
                (
                    top.min(stroke.offset - stroke.width / 2.0),
                    bottom.max(stroke.offset + stroke.width / 2.0),
                )
            },
        )
    }
}

/// 한컴 Print의 12/24/36/48pt 대조군에서 확인한 위 밑줄이다.
/// 같은 줄의 비장식 큰 글자도 높이·굵기의 기준이며, 아래 밑줄의 표와 구분한다.
pub(crate) fn top_underline_geometry(
    style: &super::TextStyle,
    font_size: f64,
) -> Option<TopUnderlineGeometry> {
    use crate::model::style::UnderlineType;
    if style.underline != UnderlineType::Top || style.underline_shape > 10 {
        return None;
    }
    let size = style
        .top_underline_font_size
        .filter(|size| size.is_finite() && *size > 0.0)
        .unwrap_or(font_size);
    if !(size * 2.0).is_finite() || size <= 0.0 {
        return None;
    }
    let shape = style.underline_shape;
    if shape == 6 {
        return Some(TopUnderlineGeometry {
            strokes: Vec::new(),
            dots: Some(DecorationDots {
                offset: -0.875 * size,
                radius: 0.03 * size,
                step: size / 7.0,
            }),
        });
    }
    let bands: &[(f64, f64)] = match shape {
        7 => &[(0.955, 0.030), (0.865, 0.030)],
        8 => &[(1.019, 0.048), (0.900, 0.100)],
        9 => &[(0.993, 0.100), (0.874, 0.048)],
        10 => &[(1.035, 0.030), (0.950, 0.080), (0.865, 0.030)],
        _ => &[(0.870, 0.040)],
    };
    // 짧은 선·긴 선과 간격의 공통 비율. Print 장치의 600dpi 반올림은 넣지 않는다.
    let pattern: &[f64] = match shape {
        1 => &[2.0, 3.0],
        2 => &[10.0, 6.0],
        3 => &[20.0, 6.0, 2.0, 6.0],
        4 => &[20.0, 6.0, 2.0, 6.0, 2.0, 6.0],
        5 => &[20.0, 6.0],
        _ => &[],
    };
    Some(TopUnderlineGeometry {
        strokes: bands
            .iter()
            .map(|&(offset, width)| DecorationStroke {
                offset: -offset * size,
                width: width * size,
                dash: pattern.iter().map(|value| value * size / 35.0).collect(),
            })
            .collect(),
        dots: None,
    })
}

/// 글줄 상자 위로 올라가는 장식(위 밑줄·강조점)의 잉크 범위다.
///
/// 첫 줄의 이 잉크는 글줄 상자 밖, 본문 영역 위에 놓여 본문 clip에 잘린다.
/// 글자·글줄 상자를 키우면 캐럿/선택도 바뀌므로 본문 clip 계산에만 쓴다.
/// 위 밑줄은 공통 선 기하, 강조점은 현재 glyph 기하를 소비한다.
/// 회전·세로쓰기·글자겹침은 다루지 않는다.
pub(crate) fn raised_decoration_ink_bbox(
    bbox: super::render_tree::BoundingBox,
    run: &super::render_tree::TextRunNode,
) -> Option<super::render_tree::BoundingBox> {
    use crate::model::style::UnderlineType;

    let style = &run.style;
    let top_underline = style.underline == UnderlineType::Top && style.underline_shape <= 10;
    let emphasis = (1..=6).contains(&style.emphasis_dot);
    if !(top_underline || emphasis)
        || run.char_overlap.is_some()
        || run.rotation != 0.0
        || run.is_vertical
        || bbox.width <= 0.0
        || !run.display_or_text().chars().any(|ch| ch != '\u{FFFC}')
    {
        return None;
    }
    let base_size = if style.font_size > 0.0 {
        style.font_size
    } else {
        12.0
    };
    let (font_size, baseline) = style.script_draw_metrics(base_size, bbox.y + run.baseline);
    let mut top = f64::INFINITY;
    let mut bottom = f64::NEG_INFINITY;
    let geometry = top_underline_geometry(style, font_size);
    if let Some(geometry) = &geometry {
        let (ink_top, ink_bottom) = geometry.vertical_bounds();
        top = top.min(baseline + ink_top);
        bottom = bottom.max(baseline + ink_bottom);
    }
    if emphasis {
        // 강조점 글자의 em 상자(기준선 위 1em, 아래 0.25em)를 잉크 범위로 본다.
        let dot_size = font_size * 0.3;
        let dot_y = baseline - font_size * 1.05;
        top = top.min(dot_y - dot_size);
        bottom = bottom.max(dot_y + dot_size * 0.25);
    }
    // 원형 점선(6)만 round cap 이라 선 양 끝이 획 반만큼 번진다.
    let end_pad = geometry
        .as_ref()
        .and_then(|g| g.dots.as_ref())
        .map_or(0.0, |d| d.radius);
    let ink = super::render_tree::BoundingBox::new(
        bbox.x - end_pad,
        top,
        bbox.width + 2.0 * end_pad,
        bottom - top,
    );
    [ink.x, ink.y, ink.width, ink.height]
        .iter()
        .all(|v| v.is_finite())
        .then_some(ink)
}
