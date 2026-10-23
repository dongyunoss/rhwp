//! 재조판 호스트가 발행한 점유와 원점을 전체/부분 배치가 함께 소비한다.

use super::{LayoutEngine, ResolvedStyleSet};
use crate::model::paragraph::Paragraph;
use crate::model::table::Table;
use crate::renderer::float_placement::{ReflowBlockCellFlow, ReflowBlockOrigin};
use crate::renderer::hwpunit_to_px;
use crate::renderer::page_layout::LayoutRect;

pub(in crate::renderer::layout) struct ReflowBlockHostScope<'a> {
    slot: &'a std::cell::Cell<Option<(f64, f64)>>,
}

impl Drop for ReflowBlockHostScope<'_> {
    fn drop(&mut self) {
        self.slot.set(None);
    }
}

impl LayoutEngine {
    /// 셀 단위 나눔의 페이지 초과 여부는 호스트 밴드와 부동 자식 프레임을 구분한다.
    /// 부동 자식까지 합친 전체 높이는 예약·배치에서 유지한다. 그 확장만으로 셀 내부
    /// 나눔을 허용하면 한 쪽에 들어가는 호스트에 붙은 자식도 부모 정책과 달리 잘린다.
    pub(in crate::renderer) fn reflow_cellbreak_row_host_band_height(
        &self,
        table: &Table,
        row: usize,
        styles: &ResolvedStyleSet,
    ) -> Option<f64> {
        if table.page_break != crate::model::table::TablePageBreak::CellBreak {
            return None;
        }
        let cells = self.indexed_row_cut_cell_order(table, row);
        if cells.is_empty() {
            return None;
        }
        let mut height = 0.0_f64;
        for index in cells {
            let cell = &table.cells[index];
            if cell.row_span != 1 {
                return None;
            }
            let flow = self.reflow_block_cell_flow(&cell.paragraphs, styles)?;
            // 제한 없는 자식의 독립 스트림은 이 부모 프레임의 원자 단위가 아니다.
            if flow
                .origins
                .iter()
                .flatten()
                .flatten()
                .any(|o| o.host_relative)
            {
                return None;
            }
            let (_, _, top, bottom) = self.resolve_stored_nested_cell_padding(cell, table, styles);
            height = height.max(flow.host_bottom + top + bottom);
        }
        Some(height)
    }

    /// 셀·투명 래퍼의 재귀 배치가 외부 문단의 원점을 덮어쓰지 않는다.
    pub(in crate::renderer::layout) fn reflow_block_host_scope(
        &self,
        root: bool,
        x: f64,
        y: f64,
    ) -> Option<ReflowBlockHostScope<'_>> {
        if !root || self.reflow_block_host_anchor.get().is_some() {
            return None;
        }
        self.reflow_block_host_anchor.set(Some((x, y)));
        Some(ReflowBlockHostScope {
            slot: &self.reflow_block_host_anchor,
        })
    }

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
    /// 원본 컷은 보존하고 가시 viewport는 물리 용지 경계를 소비한다.
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
        text_left: f64,
        text_top: f64,
    ) -> Option<(Option<f64>, f64)> {
        let origin = flow
            .origins
            .get(paragraph_index)?
            .get(control_index)?
            .as_ref()?;
        self.resolve_reflow_block_origin(*origin, table, text_left, text_top)
    }

    /// 부분 컷이 확정한 본문 원점을 유지하며 같은 참조계로 각 조각을 배치한다.
    pub(in crate::renderer::layout) fn reflow_block_fragment_table_origin(
        &self,
        flow: &ReflowBlockCellFlow,
        paragraph_index: usize,
        control_index: usize,
        table: &Table,
        text_left: f64,
        selected_flow_top: f64,
    ) -> Option<(Option<f64>, f64)> {
        let mut origin = *flow
            .origins
            .get(paragraph_index)?
            .get(control_index)?
            .as_ref()?;
        if !origin.host_relative {
            origin.top = hwpunit_to_px(i32::from(table.outer_margin_top), self.dpi);
        }
        self.resolve_reflow_block_origin(origin, table, text_left, selected_flow_top)
    }

    fn resolve_reflow_block_origin(
        &self,
        origin: ReflowBlockOrigin,
        table: &Table,
        text_left: f64,
        text_top: f64,
    ) -> Option<(Option<f64>, f64)> {
        let offset = hwpunit_to_px(table.common.horizontal_offset as i32, self.dpi);
        let margin_left = hwpunit_to_px(i32::from(table.outer_margin_left), self.dpi);
        if origin.host_relative {
            let (x, y) = self.reflow_block_host_anchor.get()?;
            return Some((
                Some(x + offset + margin_left),
                y + origin.top + hwpunit_to_px(i32::from(table.outer_margin_top), self.dpi),
            ));
        }
        Some((
            Some(text_left + offset + margin_left),
            text_top + origin.top,
        ))
    }
}
