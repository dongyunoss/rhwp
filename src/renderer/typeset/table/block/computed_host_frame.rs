//! 계산한 빈 글줄과 자리차지 표의 바깥 상자를 같은 원점에서 점유한다.

use super::BlockTableInput;
use crate::model::{paragraph::LineSeg, shape::VertAlign};
use crate::renderer::{
    float_placement::{
        control_leaves_table_anchor_empty, is_para_topbottom_float, signed_hwpunit,
        ParagraphFloatFlow, ParagraphFloatPlacement,
    },
    hwpunit_to_px,
};

pub(super) fn resolve(
    input: BlockTableInput<'_>,
    origin: f64,
    dpi: f64,
) -> Option<ParagraphFloatPlacement> {
    let BlockTableInput {
        para,
        table,
        ft,
        fmt,
        ..
    } = input;
    // 실제 저장 줄의 원점은 다른 소유자가 검증한다. 계산 줄을 지우거나
    // 저장 줄로 승격하지 않고, 이미 format한 줄 점유를 그대로 함께 사용한다.
    if !para.text.is_empty()
        || para.line_segs.is_empty()
        || para
            .line_segs
            .iter()
            .any(|line| line.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0)
        || !is_para_topbottom_float(&table.common)
        || !matches!(table.common.vert_align, VertAlign::Top)
        || signed_hwpunit(table.common.vertical_offset) < 0
    {
        return None;
    }
    let mut objects = para
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
    let table_top = origin
        + ft.host_spacing.before
        + hwpunit_to_px(signed_hwpunit(table.common.vertical_offset), dpi);
    let table_end = table_top + ft.effective_height + ft.host_spacing.after;
    let line_end = origin + fmt.total_height;
    if ![origin, table_top, table_end, line_end]
        .iter()
        .all(|value| value.is_finite())
    {
        return None;
    }
    Some(ParagraphFloatPlacement {
        flow: ParagraphFloatFlow::Exclusion,
        anchor_y: origin,
        stored_host_origin: None,
        stored_successor_line_origin: None,
        table_left: None,
        table_top,
        // 같은 호스트의 계산 줄과 표는 병렬 점유다. 둘의 높이를 합산하지 않는다.
        occupied_bottom: table_end.max(line_end),
    })
}
