//! 하단 범례의 실제 행 구성과 플롯 예약이 공유하는 점유 결과.

use super::legacy_presentation::{Font, Presentation, Text};

pub(super) struct LegendLayout {
    pub x: f64,
    pub y: f64,
    pub marker: f64,
    pub column_step: f64,
    pub row_step: f64,
    pub rows: usize,
    /// 저장된 범례 행 구성에서 늘거나 줄어든 플롯 예약 높이.
    pub section_top_delta: f64,
}

/// 저장 TextBlock의 점유와 실제 폰트/DPI를 기존 범례 painter와 같은 단위로 읽는다.
pub(super) fn text_size(text: &Text) -> Option<[f64; 2]> {
    let m = &text.metrics;
    if !m.cache_valid
        || m.extent.iter().any(|v| *v < 0)
        || m.margin < 0
        || m.orientation != 0
        || m.horizontal > 2
        || m.vertical != 2
    {
        return None;
    }
    let width = f64::from(m.rect[2]) - f64::from(m.rect[0]);
    let height = f64::from(m.rect[3]) - f64::from(m.rect[1]);
    (width > 0.0 && height > 0.0).then_some([width, height])
}

impl LegendLayout {
    pub fn new(
        p: &Presentation,
        font: &Font,
        width: f64,
        height: f64,
        dpi: f64,
        inset: f64,
    ) -> Option<Self> {
        let section = p.legend_section.as_ref()?;
        let unit = dpi / 1440.0;
        let sx = width / (p.extent[2] - p.extent[0]);
        let sy = height / (p.extent[3] - p.extent[1]);
        let map_y = |y: f64| (p.extent[3] - y) * sy;
        let mut marker: f64 = 0.0;
        let mut label_width: f64 = 0.0;
        let mut cached_marker: f64 = 0.0;
        for series in &p.series {
            let label = series.legend.as_ref()?;
            if label.font != *font || label.metrics.horizontal != 0 {
                return None;
            }
            let size = text_size(label)?;
            cached_marker = cached_marker.max(size[1]);
            marker = marker.max(size[1] * unit);
            label_width = label_width.max(size[0] * unit);
        }
        let column_width = marker * 1.5 + label_width;
        let gap = f64::from(section.internal) * sx;
        // 바깥여백은 글꼴과 같은 물리 twip 단위이며 저장 프레임의 배율을 적용하지 않는다.
        let available_width = width - 2.0 * f64::from(section.external) * unit;
        let columns = ((available_width + gap) / (column_width + gap)).floor() as usize;
        if columns == 0 || p.series.is_empty() {
            return None;
        }
        let columns = columns.min(p.series.len());
        let rows = p.series.len().div_ceil(columns);
        let grid_width = columns as f64 * (column_width + gap) - gap;
        let row_step = marker * 1.5;
        let grid_height = rows as f64 * row_step - marker * 0.5;
        // 아래쪽 50단위 여유와 내부 여백은 저장 Section의 root 변환을 따른다.
        let bottom = map_y(section.rect[1] + inset + 50.0);
        let y = bottom - grid_height;
        // Section 경계는 저장 조판의 예약 영역이며 마커의 실제 그리기 영역이 아니다.
        // 두 경계를 직접 맞추면 행 수가 그대로인 정상 차트까지 이동한다.
        // 저장 예약에서 행 수를 읽고 새 행 구성으로 증감한 높이만 플롯에 전달한다.
        let cached_grid_height = section.rect[3] - section.rect[1] - 2.0 * inset - 50.0;
        let cached_rows = (cached_grid_height + cached_marker * 0.5) / (cached_marker * 1.5);
        let rounded_rows = cached_rows.round();
        if rounded_rows < 1.0 || (cached_rows - rounded_rows).abs() > f64::EPSILON * 16.0 {
            return None;
        }
        Some(Self {
            x: (width - grid_width - 50.0 * sx) / 2.0,
            y,
            marker,
            column_step: column_width + gap,
            row_step,
            rows,
            section_top_delta: (rounded_rows - rows as f64) * row_step,
        })
    }

    /// 범례는 열 안에서 위에서 아래로 채운 뒤 다음 열로 이어진다.
    pub fn origin(&self, index: usize) -> [f64; 2] {
        [
            self.x + (index / self.rows) as f64 * self.column_step,
            self.y + (index % self.rows) as f64 * self.row_step,
        ]
    }
}
