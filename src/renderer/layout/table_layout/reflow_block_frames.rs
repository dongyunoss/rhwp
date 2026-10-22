//! 재조판 호스트가 발행한 점유와 원점을 전체/부분 배치가 함께 소비한다.

use super::{LayoutEngine, ResolvedStyleSet};
use crate::model::paragraph::Paragraph;
use crate::model::table::Table;
use crate::renderer::float_placement::ReflowBlockCellFlow;
use crate::renderer::hwpunit_to_px;
use crate::renderer::page_layout::LayoutRect;

impl LayoutEngine {
    /// 빈 재조판 호스트의 글자 높이가 아니라 실제 첫 소유 유닛의 컷 높이.
    /// 사전 이월과 부분 배치가 같은 자식 행·여백의 원장을 소비한다.
    pub(in crate::renderer) fn reflow_row_opening_cut_height(
        &self,
        table: &Table,
        row: usize,
        styles: &ResolvedStyleSet,
    ) -> Option<f64> {
        let cells = self.indexed_row_cut_cell_order(table, row);
        if cells.is_empty() {
            return None;
        }
        let mut end_cut = Vec::with_capacity(cells.len());
        for index in cells {
            let cell = &table.cells[index];
            if cell.row_span != 1 {
                return None;
            }
            self.reflow_block_cell_flow(&cell.paragraphs, styles)?;
            end_cut.push(self.cell_units(cell, table, styles).len().min(1));
        }
        Some(self.row_cut_content_height(table, row, &[], &end_cut, styles))
    }

    /// 부모 셀의 흐름 예산은 제한 없는 부동 개체의 가시 viewport가 아니다.
    /// 소스 컷이 없는 개체는 현재 용지 상자를 소비하며 부모 Cell clip은 넘는다.
    pub(in crate::renderer::layout) fn unrestricted_cell_float_area(
        &self,
        fallback: &LayoutRect,
        origin: (Option<f64>, f64),
    ) -> LayoutRect {
        let x = origin.0.unwrap_or(fallback.x);
        let y = origin.1;
        LayoutRect {
            x,
            y,
            width: (self.current_paper_width.get() - x).max(0.0),
            height: (self.current_page_height.get() - y).max(0.0),
        }
    }

    pub(in crate::renderer::layout) fn reflow_block_cell_flow(
        &self,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
    ) -> Option<ReflowBlockCellFlow> {
        if self.profile.get().hwp3_layout() {
            return None;
        }
        crate::renderer::float_placement::reflow_block_cell_flow(
            paragraphs,
            styles,
            self.dpi,
            |child| self.nested_table_body_height(child, styles),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::renderer::layout) fn reflow_block_table_origin(
        &self,
        flow: &ReflowBlockCellFlow,
        paragraph_index: usize,
        control_index: usize,
        table: &Table,
        cell_left: f64,
        cell_top: f64,
        text_left: f64,
        text_top: f64,
    ) -> Option<(Option<f64>, f64)> {
        let origin = flow
            .origins
            .get(paragraph_index)?
            .get(control_index)?
            .as_ref()?;
        let offset = hwpunit_to_px(table.common.horizontal_offset as i32, self.dpi);
        Some((
            Some(if origin.cell_relative {
                cell_left + offset
            } else {
                text_left + offset + hwpunit_to_px(i32::from(table.outer_margin_left), self.dpi)
            }),
            if origin.cell_relative {
                cell_top
            } else {
                text_top
            } + origin.top,
        ))
    }
}
