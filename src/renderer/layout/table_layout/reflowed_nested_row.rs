//! 중첩 RowBreak 행의 일반 글줄 컷을 상위 조각에도 보존한다.

use super::{CellUnit, LayoutEngine, NestedTableUnitCut, ResolvedStyleSet};
use crate::model::table::{Cell, Table};

impl LayoutEngine {
    pub(super) fn nested_plain_text_row_has_line_cuts(
        &self,
        row_cells: &[&Cell],
        driver_units: &[CellUnit],
    ) -> bool {
        // CELL은 선언 높이가 있어도 셀 안 글줄에서 나눌 수 있다. 부모의 앞 내용이
        // 늘어 남은 예산이 줄어드는 경우도 같으며, 자식 자신의 성장만 요구하지 않는다.
        // 개체 혼재·rowspan 경로는 기존 분할 계약을 유지한다. 호출자가 rowspan과
        // RowBreak를 확인하고, 여기서는 같은 CellUnit의 일반 글줄 경계만 승인한다.
        driver_units.len() > 1
            && driver_units
                .iter()
                .all(|unit| unit.nested_row.is_none() && unit.vis_start < unit.vis_end)
            && !row_cells.is_empty()
            && row_cells.iter().all(|cell| {
                !cell.paragraphs.is_empty()
                    && cell
                        .paragraphs
                        .iter()
                        .all(|paragraph| paragraph.controls.is_empty())
            })
    }

    /// 자식 배치에 전달하는 것과 같은 시작·종료 컷의 물리 상자를 예약한다.
    /// 마지막 유닛을 소비하면 배치도 빈 end_cut으로 행 전체를 닫는다. 이때
    /// 마지막 줄까지의 내용 높이만 쓰면 선언 상자의 남은 띠가 원장에서 사라진다.
    pub(super) fn nested_plain_row_unit_height(
        &self,
        table: &Table,
        row_frame: (usize, f64),
        frame_start: &[usize],
        fragment: &NestedTableUnitCut,
        styles: &ResolvedStyleSet,
    ) -> f64 {
        let (row, whole_height) = row_frame;
        let origin = if frame_start.iter().all(|&cut| cut == 0) {
            &[][..]
        } else {
            frame_start
        };
        let end = if fragment.terminal {
            &[][..]
        } else {
            fragment.end_cut.as_slice()
        };
        let previous = if fragment.start_cut == frame_start {
            0.0
        } else {
            self.row_cut_content_height(table, row, origin, &fragment.start_cut, styles)
        };
        let required = self.row_cut_content_height(table, row, origin, end, styles);
        let required = if origin.is_empty() && fragment.terminal {
            required.max(whole_height)
        } else {
            required
        };
        (required - previous).max(0.0)
    }
}
