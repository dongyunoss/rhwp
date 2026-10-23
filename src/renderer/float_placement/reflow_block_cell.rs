//! 저장 앵커가 없는 빈 표 호스트의 줄·개체·흐름 점유를 분리한다.

use super::{hwpunit_to_px, signed_hwpunit};
use crate::model::{
    control::Control,
    paragraph::{ColumnBreakType, LineSeg, Paragraph},
    shape::{HorzAlign, HorzRelTo, TextWrap, VertAlign, VertRelTo},
    table::Table,
};
use crate::renderer::style_resolver::ResolvedStyleSet;

#[derive(Clone, Copy)]
pub(crate) struct ReflowBlockOrigin {
    /// 제한이 꺼진 부동 표는 부모 셀과 별개인 외부 호스트 문단을 참조한다.
    pub(crate) host_relative: bool,
    /// 제한을 켜면 내용 원점 기준이며 표의 위 바깥여백까지 포함한다.
    pub(crate) top: f64,
}

pub(crate) struct ReflowBlockCellFlow {
    pub(crate) paragraph_origins: Vec<f64>,
    pub(crate) origins: Vec<Vec<Option<ReflowBlockOrigin>>>,
    /// 부동 자식의 별도 프레임을 제외한 호스트 글줄의 점유 끝.
    pub(crate) host_bottom: f64,
    pub(crate) bottom: f64,
}

/// 온전한 호스트와 부분 조각이 같은 빈 글줄의 점유 높이를 사용한다.
pub(crate) fn reflow_host_line_height(
    para: &Paragraph,
    styles: &ResolvedStyleSet,
    dpi: f64,
    last: bool,
) -> Option<f64> {
    let em = para
        .char_shapes
        .first()
        .and_then(|shape| styles.char_styles.get(shape.char_shape_id as usize))?
        .font_size;
    if em <= 0.0 {
        return None;
    }
    Some(if last {
        em
    } else {
        styles
            .para_styles
            .get(para.para_shape_id as usize)
            .map_or(em, |style| {
                crate::renderer::corrected_line_height(
                    hwpunit_to_px(400, dpi),
                    em,
                    style.line_spacing_type,
                    style.line_spacing,
                )
            })
    })
}

/// 저장된 선행 글줄 뒤의 NO_LS 호스트도 동일한 재조판 프레임을 연다.
/// 다른 개체와 혼합된 호스트·저장 위치로 돌아오는 후속 문단은 각각의 기존
/// 줄 구성/저장 프레임 계약을 유지한다. 자식 높이는 바깥여백을 제외한 실제 본체다.
pub(crate) fn reflow_block_cell_flow(
    paragraphs: &[Paragraph],
    styles: &ResolvedStyleSet,
    dpi: f64,
    mut table_body_height: impl FnMut(&Table) -> f64,
) -> Option<ReflowBlockCellFlow> {
    let mut reflow_started = false;
    let mut has_host = false;
    for para in paragraphs {
        let no_ls = crate::renderer::para_has_no_stored_line_segs(para);
        if para.column_type != ColumnBreakType::None {
            return None;
        }
        if no_ls {
            // 가시 텍스트의 줄 나눔은 실제 가용 폭에서 구성한 결과가 소유한다.
            if !para.text.is_empty() {
                return None;
            }
            reflow_started = true;
        } else if reflow_started
            || para.stored_text_partition_is_dirty()
            || para.cell_format_vpos_dirty
            || para.line_segs.iter().any(|line| {
                line.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
                    || line.vertical_pos < 0
                    || line.line_height <= 0
            })
        {
            return None;
        }
        for control in &para.controls {
            match control {
                Control::Table(table)
                    if no_ls
                        && !table.common.treat_as_char
                        && table.common.text_wrap == TextWrap::TopAndBottom
                        && table.common.vert_rel_to == VertRelTo::Para
                        && table.common.vert_align == VertAlign::Top
                        && matches!(
                            table.common.horz_rel_to,
                            HorzRelTo::Para | HorzRelTo::Column
                        )
                        && table.common.horz_align == HorzAlign::Left
                        && table.caption.is_none() =>
                {
                    has_host = true;
                }
                Control::ColumnDef(columns) if columns.column_count == 1 => {}
                _ => return None,
            }
        }
    }
    if !has_host {
        return None;
    }

    let mut flow = ReflowBlockCellFlow {
        paragraph_origins: Vec::with_capacity(paragraphs.len()),
        origins: paragraphs
            .iter()
            .map(|p| vec![None; p.controls.len()])
            .collect(),
        host_bottom: 0.0,
        bottom: 0.0,
    };
    let mut cursor = 0.0_f64;
    for (pi, para) in paragraphs.iter().enumerate() {
        let style = styles.para_styles.get(para.para_shape_id as usize);
        let last = pi + 1 == paragraphs.len();
        let before = crate::renderer::cell_paragraph_spacing::paragraph_before(
            para,
            pi,
            style.map_or(0.0, |style| style.spacing_before),
            dpi,
        )
        .occupied_before();
        let after = if last {
            0.0
        } else {
            style.map_or(0.0, |style| style.spacing_after)
        };
        cursor += before;
        flow.host_bottom += before;
        let no_ls = crate::renderer::para_has_no_stored_line_segs(para);
        if !no_ls {
            // 선행 저장 사다리의 빈 밴드는 글자 수나 합산 높이로 추측하지 않는다.
            let top = hwpunit_to_px(para.line_segs.first()?.vertical_pos, dpi);
            if top < cursor {
                return None;
            }
            cursor = top;
            flow.host_bottom = top;
            flow.paragraph_origins.push(cursor);
            for (li, line) in para.line_segs.iter().enumerate() {
                let end = hwpunit_to_px(line.vertical_pos.saturating_add(line.line_height), dpi)
                    + if !last || li + 1 < para.line_segs.len() {
                        hwpunit_to_px(line.line_spacing, dpi)
                    } else {
                        0.0
                    };
                if end < cursor {
                    return None;
                }
                cursor = end;
                flow.host_bottom = end;
            }
        } else {
            flow.paragraph_origins.push(cursor);
            let line_height = reflow_host_line_height(para, styles, dpi, last)?;
            let mut block_height = 0.0_f64;
            for group in super::nested_table_groups(para, false) {
                let mut group_height = 0.0_f64;
                for ci in group.controls {
                    let Control::Table(table) = &para.controls[ci] else {
                        continue;
                    };
                    let offset = hwpunit_to_px(signed_hwpunit(table.common.vertical_offset), dpi);
                    let restricted = table.common.flow_with_text;
                    let preceding = if group.side_by_side {
                        0.0
                    } else {
                        group_height
                    };
                    flow.origins[pi][ci] = Some(ReflowBlockOrigin {
                        host_relative: !restricted,
                        top: if restricted {
                            cursor
                                + block_height
                                + preceding
                                + offset
                                + hwpunit_to_px(i32::from(table.outer_margin_top), dpi)
                        } else {
                            offset
                        },
                    });
                    if restricted {
                        let height = offset
                            + table_body_height(table)
                            + hwpunit_to_px(
                                i32::from(table.outer_margin_top)
                                    + i32::from(table.outer_margin_bottom),
                                dpi,
                            );
                        group_height = if group.side_by_side {
                            group_height.max(height)
                        } else {
                            group_height + height
                        };
                    }
                }
                block_height += group_height;
            }
            let occupied = line_height.max(block_height);
            cursor += occupied;
            flow.host_bottom += line_height;
        }
        cursor += after;
        flow.host_bottom += after;
    }
    flow.bottom = cursor;
    Some(flow)
}
