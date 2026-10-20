//! 쪽 제한을 끈 나누지 않는 float는 본문 높이 때문에 행을 자르지 않는다.

use super::BlockTableInput;
use crate::model::{shape::VertAlign, table::TablePageBreak};
use crate::renderer::{
    float_placement::{
        control_leaves_table_anchor_empty, is_para_topbottom_float, signed_hwpunit,
        ParagraphFloatFlow, ParagraphFloatPlacement,
    },
    hwpunit_to_px,
};

pub(super) fn owns_whole_object(input: BlockTableInput<'_>) -> bool {
    input.table.page_break == TablePageBreak::None
        && is_para_topbottom_float(&input.table.common)
        && !input.table.common.flow_with_text
        && input.ft.table_footnotes.is_empty()
}

/// 다른 경로가 인증한 원점은 호출자가 우선한다. 빈 Top 앵커의 누락 상자만
/// 전체 format 결과에서 만들고 예약과 실제 배치에 같은 값을 전달한다.
pub(super) fn missing_frame(
    input: BlockTableInput<'_>,
    origin: f64,
    dpi: f64,
) -> Option<ParagraphFloatPlacement> {
    if !owns_whole_object(input)
        || !input.para.text.is_empty()
        || input.table.common.vert_align != VertAlign::Top
    {
        return None;
    }
    let mut objects = input
        .para
        .controls
        .iter()
        .filter(|control| !control_leaves_table_anchor_empty(control));
    if !matches!(
        objects.next(),
        Some(crate::model::control::Control::Table(_))
    ) || objects.next().is_some()
    {
        return None;
    }
    let offset = hwpunit_to_px(signed_hwpunit(input.table.common.vertical_offset), dpi);
    let table_top = origin + input.ft.host_spacing.before + offset;
    let occupied_bottom = origin + input.ft.total_height + offset;
    if ![origin, table_top, occupied_bottom]
        .iter()
        .all(|value| value.is_finite())
    {
        return None;
    }
    Some(ParagraphFloatPlacement {
        flow: ParagraphFloatFlow::EmptyHost,
        anchor_y: origin,
        stored_host_origin: None,
        stored_successor_line_origin: None,
        table_left: None,
        table_top,
        occupied_bottom,
    })
}
