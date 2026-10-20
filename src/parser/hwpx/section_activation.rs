//! 단 정의가 없는 HWPX의 구역 선언과 실제 한컴 기본 용지 해석을 구분한다.
use crate::model::{control::Control, document::Section, page::PageDef};

pub(super) fn apply_missing_column_defaults(section: &mut Section) {
    let controls = || {
        section
            .paragraphs
            .iter()
            .flat_map(|paragraph| &paragraph.controls)
    };
    if !controls().any(|control| matches!(control, Control::SectionDef(_)))
        || controls().any(|control| matches!(control, Control::ColumnDef(_)))
    {
        return;
    }
    // 한컴 Print와 동일 입력의 한컴 저장본에서 단 정의가 없으면 선언 여백 대신
    // 새 문서 기본 용지를 사용하는 것을 확인했다. 단 정의가 있는 구역은 보존한다.
    let page = PageDef::a4_default();
    section.section_def.page_def = page.clone();
    for paragraph in &mut section.paragraphs {
        for control in &mut paragraph.controls {
            if let Control::SectionDef(definition) = control {
                definition.page_def = page.clone();
            }
        }
    }
}
