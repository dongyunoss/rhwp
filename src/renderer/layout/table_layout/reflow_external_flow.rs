//! 같은 외부 호스트에 붙은 부모 프레임과 자식 개체의 전량 점유.

use super::{LayoutEngine, ResolvedStyleSet};
use crate::model::{shape::VertAlign, table::Table};
use crate::renderer::{
    float_placement::{is_para_topbottom_float, signed_hwpunit, ReflowExternalTableFlow},
    hwpunit_to_px,
};

impl LayoutEngine {
    /// 원본 행 유닛의 순서는 보존하고 실제 공동 앵커의 점유 끝만 별도로 발행한다.
    /// 여러 물리 행·걸침 칸의 원장은 일반 row/block 경로가 계속 소유한다.
    pub(in crate::renderer) fn reflow_unrestricted_table_flow(
        &self,
        table: &Table,
        styles: &ResolvedStyleSet,
    ) -> Option<ReflowExternalTableFlow> {
        if !is_para_topbottom_float(&table.common)
            || table.common.flow_with_text
            || table.common.vert_align != VertAlign::Top
            || table.caption.is_some()
            || table.row_count != 1
            || table.cells.is_empty()
        {
            return None;
        }
        let mut parent_frame_height = 0.0_f64;
        let mut external_bottom = None::<f64>;
        let mut child_splittable = false;
        for cell in &table.cells {
            if cell.row_span != 1 || cell.text_direction != 0 {
                return None;
            }
            // 이 원장은 칸마다 연속된 자식 컷 하나를 소비한다. 같은 칸 안의 여러
            // 독립 자식은 별도의 cursor 원장이 필요하므로 기존 경로를 유지한다.
            if cell
                .paragraphs
                .iter()
                .flat_map(|para| &para.controls)
                .filter(|control| matches!(control, crate::model::control::Control::Table(_)))
                .count()
                != 1
            {
                return None;
            }
            let flow = self.reflow_block_cell_flow(&cell.paragraphs, styles)?;
            // 포함 자식의 순차 밴드와 외부 공동 앵커가 섞인 경우 기존 원장을 유지한다.
            if flow
                .origins
                .iter()
                .flatten()
                .flatten()
                .any(|origin| !origin.host_relative)
            {
                return None;
            }
            let child_end = flow.external_bottom?;
            parent_frame_height =
                parent_frame_height.max(flow.parent_frame_height(cell, table, self.dpi)?);
            external_bottom = Some(external_bottom.map_or(child_end, |end| end.max(child_end)));
            child_splittable |= flow.external_child_splittable;
        }
        let external_bottom = external_bottom?;
        parent_frame_height += hwpunit_to_px(i32::from(table.cell_spacing), self.dpi) * 2.0;
        // Format의 host 간격은 부모 바깥여백을 이미 포함한다. 같은 root의 자식 끝을
        // 그 내부 높이 축으로 변환해 부모 여백과 오프셋을 이중 계상하지 않는다.
        let outside = hwpunit_to_px(
            i32::from(table.outer_margin_top) + i32::from(table.outer_margin_bottom),
            self.dpi,
        );
        let offset = hwpunit_to_px(signed_hwpunit(table.common.vertical_offset), self.dpi);
        let effective_height = parent_frame_height.max(external_bottom - offset - outside);
        (parent_frame_height.is_finite() && external_bottom.is_finite()).then_some(
            ReflowExternalTableFlow {
                parent_frame_height,
                external_bottom,
                effective_height,
                child_splittable,
            },
        )
    }
}
