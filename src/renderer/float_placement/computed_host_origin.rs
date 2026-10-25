//! 같은 앵커의 표 하나가 전체 호스트 글줄의 원점을 인증할 수는 없다.

use crate::model::{control::Control, paragraph::Paragraph};

pub(super) fn shares_float_anchor(para: &Paragraph, char_pos: usize) -> bool {
    let positions = para.control_text_positions();
    para.controls
        .iter()
        .zip(positions)
        .filter(|(control, position)| {
            *position == char_pos
                && matches!(control, Control::Table(table)
                    if super::is_para_topbottom_float(&table.common))
        })
        .take(2)
        .count()
        > 1
}
