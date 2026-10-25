//! 저장 줄이 없는 TAC 표의 흐름과 배치가 공유하는 실제 측정 결과.

use crate::model::table::Table;
use crate::renderer::height_measurer::{HeightMeasurer, MeasuredTable};
use crate::renderer::style_resolver::ResolvedStyleSet;

pub(crate) fn inline_table_needs_reflow_measurement(table: &Table) -> bool {
    table.common.treat_as_char
        && (super::table_cells_have_no_stored_lines(table)
            || inline_table_width_cache_is_stale(table))
}

pub(crate) fn inline_table_width_cache_is_stale(table: &Table) -> bool {
    table
        .closed_inline_grid_width_hu()
        .is_some_and(|width| width != table.common.width)
}

/// 재조판 셀의 내용·행 하한·캡션을 개체 선언 높이로 다시 덮어쓰지 않는다.
/// 저장 줄이 있는 표는 기존 저장 프레임 경로에서 측정 결과를 공급한다.
pub(crate) fn measured_reflow_inline_table(
    table: &Table,
    styles: &ResolvedStyleSet,
    dpi: f64,
) -> Option<MeasuredTable> {
    inline_table_needs_reflow_measurement(table).then(|| {
        HeightMeasurer::new(dpi)
            .with_session_edited(true)
            .measure_table(table, 0, 0, styles)
    })
}
