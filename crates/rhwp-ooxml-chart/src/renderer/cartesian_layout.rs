//! 차트의 축 범위·글줄 예약을 측정과 표시가 같은 값으로 소비한다.

use super::*;

/// 차트의 기본 안쪽 여백은 기본 글자 크기의 물리 단위다.
/// 축 글자를 확대해도 프레임과 마지막 범주 글줄 사이의 여백은 변하지 않는다.
pub(super) const FRAME_PAD: f64 = hancom_default::LABEL_PT * PX_PER_PT;
pub(super) const TICK_GAP: f64 = FRAME_PAD * 0.25;

pub(super) fn title_height(chart: &OoxmlChart, has_title: bool) -> f64 {
    let text = if has_title {
        title_font_px(chart) * 1.2 + TICK_GAP
    } else {
        0.0
    };
    FRAME_PAD * 2.0 + text.max(FRAME_PAD * 1.2) + label_font_px(chart) * 0.5
}

pub(super) fn category_text_bottom(font: f64) -> f64 {
    font * 2.0 + TICK_GAP
}

#[derive(Clone, Copy)]
pub(super) struct ValueScale {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

impl ValueScale {
    fn from_tuple((min, max, step): (f64, f64, f64)) -> Self {
        Self { min, max, step }
    }

    pub fn tuple(self) -> (f64, f64, f64) {
        (self.min, self.max, self.step)
    }

    /// 원점 밖 값의 포함 여부와 축선 위치는 같은 범위에서 결정한다.
    pub fn zero_fraction(self) -> f64 {
        if self.max > self.min {
            (0.0_f64.clamp(self.min, self.max) - self.min) / (self.max - self.min)
        } else {
            0.0
        }
    }

    pub fn label_width(
        self,
        chart: &OoxmlChart,
        group: u8,
        measure: &dyn Fn(&str, f64, &str) -> f64,
    ) -> f64 {
        let horizontal = chart.chart_type == OoxmlChartType::Bar && !chart.is_combo();
        let axis = axis_at(
            chart,
            if group == 1 {
                AxisPos::Right
            } else if horizontal {
                AxisPos::Bottom
            } else {
                AxisPos::Left
            },
        );
        let fmt = axis.format_code.as_deref().or_else(|| {
            chart
                .series
                .iter()
                .find(|s| s.axis_group == group)
                .and_then(|s| s.format_code.as_deref())
        });
        let font = label_font_px(chart);
        let family = chart.text_font_family.as_deref().unwrap_or("sans-serif");
        let steps = if self.step > 0.0 {
            ((self.max - self.min) / self.step)
                .round()
                .clamp(0.0, 2000.0) as usize
        } else {
            0
        };
        let percent = chart.grouping == BarGrouping::PercentStacked
            || (chart.chart_type == OoxmlChartType::Line
                && chart.line_grouping == BarGrouping::PercentStacked);
        (0..=steps)
            .map(|i| {
                let value = self.min + i as f64 * self.step;
                let text = if percent && fmt.is_none() {
                    format!("{}%", value.round() as i64)
                } else {
                    format_value_axis_num(value, fmt)
                };
                measure(&text, font, family)
            })
            .fold(0.0, f64::max)
    }
}

/// 라벨의 두 줄 높이를 최소 눈금 간격으로 보고, 원 데이터에는 최대 열 칸을 배정한다.
/// 10% 범위 여유는 표시 범위에 포함하며 원 데이터의 최대값을 축 끝에 붙이지 않는다.
fn automatic_scale(min: f64, max: f64, length: f64, font: f64) -> ValueScale {
    if length <= 0.0 {
        return ValueScale::from_tuple((0.0, 0.0, 1.0));
    }
    let min = min.min(0.0);
    let max = max.max(0.0);
    let span = (max - min).max(f64::EPSILON);
    let budget = (length / (font * 2.0)).floor().clamp(1.0, 10.0);
    let raw_step = span / budget;
    let magnitude = 10.0_f64.powf(raw_step.log10().floor());
    let step = [1.0, 2.0, 5.0, 10.0]
        .into_iter()
        .map(|n| n * magnitude)
        .find(|n| *n > raw_step * (1.0 + 1e-9))
        .unwrap_or(magnitude * 20.0);
    let padding = span * 0.1;
    ValueScale {
        min: if min < 0.0 {
            ((min - padding) / step).floor() * step
        } else {
            0.0
        },
        max: if max > 0.0 {
            ((max + padding) / step).ceil() * step
        } else {
            0.0
        },
        step,
    }
}

#[derive(Clone, Copy)]
pub(super) struct ChartScales {
    pub primary: ValueScale,
    pub secondary: ValueScale,
}

impl ChartScales {
    pub fn new(chart: &OoxmlChart, width: f64, height: f64) -> Self {
        let font = label_font_px(chart);
        let (min, max) = raw_value_bounds(chart.series.iter());
        let mut primary = automatic_scale(min, max, height, font);
        let secondary = if chart.is_combo() || chart.has_secondary_axis {
            let pri: Vec<_> = chart.series.iter().filter(|s| s.axis_group == 0).collect();
            let sec: Vec<_> = chart.series.iter().filter(|s| s.axis_group == 1).collect();
            if !pri.is_empty() {
                let (min, max) = raw_value_bounds(pri.into_iter());
                primary = automatic_scale(min, max, height, font);
            }
            if sec.is_empty() {
                ValueScale::from_tuple((0.0, 1.0, 0.2))
            } else {
                let (min, max) = raw_value_bounds(sec.into_iter());
                automatic_scale(min, max, height, font)
            }
        } else {
            let bars = matches!(
                chart.chart_type,
                OoxmlChartType::Column | OoxmlChartType::Bar
            );
            let grouping = if chart.chart_type == OoxmlChartType::Line {
                chart.line_grouping
            } else {
                chart.grouping
            };
            let horizontal = chart.chart_type == OoxmlChartType::Bar;
            let ticks = if horizontal {
                HORIZONTAL_AXIS_TICKS
            } else {
                VERTICAL_AXIS_TICKS
            };
            let length = if horizontal { width } else { height };
            if grouping == BarGrouping::PercentStacked {
                primary = ValueScale::from_tuple((0.0, 100.0, 20.0));
            } else if grouping == BarGrouping::Stacked {
                let count = chart
                    .series
                    .iter()
                    .map(|s| s.values.len())
                    .max()
                    .unwrap_or(0);
                let positive = (0..count)
                    .map(|ci| category_positive_sum(chart, ci))
                    .fold(0.0, f64::max);
                let negative = (0..count)
                    .map(|ci| {
                        chart
                            .series
                            .iter()
                            .map(|s| s.values.get(ci).copied().unwrap_or(0.0).min(0.0))
                            .sum::<f64>()
                    })
                    .fold(0.0, f64::min);
                primary = if chart.is_3d && bars {
                    let (min, max, step) = nice_axis(0.0, positive.max(1.0), ticks);
                    ValueScale::from_tuple((min, max + if horizontal { 0.0 } else { step }, step))
                } else {
                    automatic_scale(negative, positive.max(f64::EPSILON), length, font)
                };
            } else if chart.is_3d && bars {
                primary =
                    ValueScale::from_tuple(nice_axis_no_headroom(min, max, HORIZONTAL_AXIS_TICKS));
            } else if chart.chart_type == OoxmlChartType::Stock {
                let (min, max, step) = nice_axis_no_headroom(min, max, VERTICAL_AXIS_TICKS);
                primary = ValueScale::from_tuple((min, max + step, step));
            } else if horizontal {
                primary = automatic_scale(min, max, length, font);
            }
            // 저장 범위는 측정 뒤 paint에서 덮어쓰지 않고 이 결과에서 확정한다.
            if bars {
                let axis = axis_at(
                    chart,
                    if horizontal {
                        AxisPos::Bottom
                    } else {
                        AxisPos::Left
                    },
                );
                if let Some(min) = axis.minimum {
                    primary.min = min;
                }
                if let Some(max) = axis.maximum.filter(|max| *max > primary.min) {
                    primary.max = max;
                }
                if axis.minimum.is_some() || axis.maximum.is_some() {
                    let intervals = if axis.format_code.as_deref().is_some_and(|s| s.contains('%'))
                    {
                        5.0
                    } else {
                        ticks
                    };
                    primary.step = (primary.max - primary.min) / intervals;
                }
            }
            ValueScale::from_tuple((0.0, 1.0, 0.2))
        };
        Self { primary, secondary }
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct CategoryLabelLayout {
    pub rotate: bool,
    pub maximum_advance: f64,
    pub axis_fraction: f64,
}

impl CategoryLabelLayout {
    pub fn new(chart: &OoxmlChart, width: f64, measure: &dyn Fn(&str, f64, &str) -> f64) -> Self {
        let font = label_font_px(chart);
        let family = chart.text_font_family.as_deref().unwrap_or("sans-serif");
        let maximum_advance = chart
            .categories
            .iter()
            .map(|s| measure(s, font, family))
            .fold(0.0, f64::max);
        let count = chart.categories.len().max(
            chart
                .series
                .iter()
                .map(|s| s.values.len())
                .max()
                .unwrap_or(0),
        );
        let rotate = !chart.is_3d
            && chart.chart_type != OoxmlChartType::Bar
            && chart.chart_type != OoxmlChartType::Scatter
            && count > 0
            && maximum_advance > width / count as f64;
        Self {
            rotate,
            maximum_advance,
            axis_fraction: 0.0,
        }
    }

    pub fn bottom_reservation(self, chart: &OoxmlChart, crosses_inside: bool) -> f64 {
        let font = label_font_px(chart);
        if crosses_inside {
            FRAME_PAD + font * 0.5
        } else if self.rotate {
            FRAME_PAD + font + self.maximum_advance
        } else {
            FRAME_PAD + category_text_bottom(font)
        }
    }

    pub fn axis_y(self, y: f64, height: f64) -> f64 {
        y + height * (1.0 - self.axis_fraction)
    }
}

/// 반복 계산에서 예약한 여백과 최종 축·그리기 프레임을 함께 반환한다.
pub(super) struct CartesianLayout {
    pub left: f64,
    pub width: f64,
    pub height: f64,
    pub scales: ChartScales,
    pub category: CategoryLabelLayout,
}

impl CartesianLayout {
    pub fn new(
        chart: &OoxmlChart,
        width: f64,
        height: f64,
        title_height: f64,
        legend_height: f64,
        legend_width: f64,
        measure: &dyn Fn(&str, f64, &str) -> f64,
    ) -> Self {
        let font = label_font_px(chart);
        let horizontal = chart.chart_type == OoxmlChartType::Bar
            && !chart.is_combo()
            && !chart.has_secondary_axis;
        let pie = chart.chart_type == OoxmlChartType::Pie;
        let raw_min = raw_value_bounds(chart.series.iter()).0;
        let crosses_inside = !horizontal
            && !chart.is_3d
            && axis_at(chart, AxisPos::Left).minimum.unwrap_or(raw_min) < 0.0;
        let mut left: f64 = 0.0;
        let mut right: f64 = 0.0;
        let mut rotated = false;
        loop {
            let plot_width = (width - left - right - legend_width).max(0.0);
            let mut category = CategoryLabelLayout::new(chart, plot_width, measure);
            if category.rotate && !rotated {
                // 방향이 바뀌면 이전 방향의 숫자 라벨 예약은 다시 계산한다.
                // 이 이후의 너비 감소로 이미 세운 글줄을 수평으로 되돌리지는 않는다.
                rotated = true;
                left = 0.0;
                right = 0.0;
                continue;
            }
            category.rotate = rotated;
            let bottom = if pie {
                font * hancom_default::PIE_BOTTOM_PAD_EM
            } else {
                category.bottom_reservation(chart, crosses_inside)
            };
            let plot_height = (height - title_height - legend_height - bottom).max(0.0);
            let scales = ChartScales::new(chart, plot_width, plot_height);
            if crosses_inside || (horizontal && !chart.is_3d) {
                category.axis_fraction = scales.primary.zero_fraction();
            }
            let needed_left = if pie {
                0.0
            } else if horizontal {
                category.maximum_advance + FRAME_PAD * 2.05
            } else if chart.chart_type == OoxmlChartType::Scatter {
                estimate_axis_label_width(chart, 0)
            } else {
                scales.primary.label_width(chart, 0, measure) + FRAME_PAD * 1.8
            };
            let needed_right = if chart.has_secondary_axis {
                scales.secondary.label_width(chart, 1, measure) + FRAME_PAD * 1.8
            } else if horizontal {
                // 마지막 수치 라벨은 축 끝을 중심으로 놓인다.
                let axis = axis_at(chart, AxisPos::Bottom);
                let text = format_value_axis_num(scales.primary.max, axis.format_code.as_deref());
                let family = chart.text_font_family.as_deref().unwrap_or("sans-serif");
                measure(&text, font, family) * 0.5 + FRAME_PAD * 1.3
            } else if chart.chart_type == OoxmlChartType::Scatter {
                value_axis_max_label_width(chart) * 0.5 + FRAME_PAD * 1.3
            } else {
                FRAME_PAD
            };
            // 같은 숫자 예산과 라벨 방향에는 같은 글자들이 생긴다. 필요한 여백만
            // 단조롭게 예약하여 축 간격 변경이 앞선 측정 결과를 무효화하지 않게 한다.
            if needed_left <= left + 1e-7 && needed_right <= right + 1e-7 {
                return Self {
                    left,
                    width: plot_width,
                    height: plot_height,
                    scales,
                    category,
                };
            }
            left = left.max(needed_left);
            right = right.max(needed_right);
        }
    }
}
