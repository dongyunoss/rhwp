//! 시험지형 HML(수학비서 등이 HWPML 2.8 로 저장)에서 쓰는 미주·자동 번호·그림·수식 캡션.

use super::parse_hml;
use crate::model::control::{AutoNumberType, Control};
use crate::model::header_footer::HeaderFooterApply;

const EXAM: &str = concat!(
    r#"<?xml version="1.0" encoding="UTF-8"?>"#,
    r#"<HWPML Style="embed" SubVersion="8.0.1.0" Version="2.8"><HEAD SecCnt="1"><MAPPINGTABLE>"#,
    r#"<BINDATALIST Count="1"><BINITEM BinData="1" Format="png" Type="Embedding"/></BINDATALIST>"#,
    r#"</MAPPINGTABLE></HEAD><BODY><SECTION Id="0"><P ParaShape="0" Style="0"><TEXT CharShape="0">"#,
    // 문제 번호 = 미주 번호, 미주 내용 = 정답
    r#"<ENDNOTE><PARALIST><P ParaShape="0" Style="0"><TEXT CharShape="0">"#,
    r#"<AUTONUM Number="3" NumberType="Endnote"><AUTONUMFORMAT SuffixChar="." Superscript="false" Type="Digit"/></AUTONUM>"#,
    r#"<CHAR> [정답] ④</CHAR></TEXT></P></PARALIST></ENDNOTE>"#,
    r#"<CHAR>가</CHAR>"#,
    // 수식 개체의 숨은 캡션 문단은 본문으로 새면 안 된다
    r#"<EQUATION BaseUnit="1000"><SHAPEOBJECT><SIZE Height="100" Width="100"/>"#,
    r#"<CAPTION><PARALIST><P ParaShape="1" Style="0"><TEXT CharShape="0"><CHAR>qLa0</CHAR></TEXT></P></PARALIST></CAPTION>"#,
    r#"</SHAPEOBJECT><SCRIPT>x</SCRIPT></EQUATION>"#,
    r#"<CHAR>나</CHAR>"#,
    r#"<PICTURE Reverse="false"><SHAPEOBJECT><SIZE Height="2000" Width="3000"/>"#,
    r#"<POSITION TreatAsChar="true" VertRelTo="Para" HorzRelTo="Column"/>"#,
    r#"<CAPTION><PARALIST><P ParaShape="1" Style="0"><TEXT CharShape="0"><CHAR>rX7h</CHAR></TEXT></P></PARALIST></CAPTION>"#,
    r#"</SHAPEOBJECT><IMAGE BinItem="1" Effect="RealPic"/></PICTURE>"#,
    r#"<CHAR>다</CHAR></TEXT></P></SECTION></BODY>"#,
    r#"<TAIL><BINDATASTORAGE><BINDATA Encoding="Base64" Id="1" Compress="false">iVBORw0K
Ggo=</BINDATA></BINDATASTORAGE></TAIL></HWPML>"#,
);

#[test]
fn hwpml_2_8_opens() {
    let result = parse_hml(EXAM.as_bytes()).expect("HWPML 2.8 은 열려야 한다");
    assert_eq!(result.metadata.hwpml_version.as_deref(), Some("2.8"));
}

#[test]
fn note_and_caption_paragraphs_stay_out_of_the_body() {
    let document = parse_hml(EXAM.as_bytes()).unwrap().document;
    let paragraphs = &document.sections[0].paragraphs;
    assert_eq!(paragraphs.len(), 1, "미주·캡션 문단이 본문 문단으로 새면 안 된다");
    assert_eq!(paragraphs[0].text, "가나다");
}

#[test]
fn controls_keep_their_text_positions() {
    let document = parse_hml(EXAM.as_bytes()).unwrap().document;
    let paragraph = &document.sections[0].paragraphs[0];
    let kinds: Vec<&str> = paragraph
        .controls
        .iter()
        .map(|control| match control {
            Control::Endnote(_) => "endnote",
            Control::Equation(_) => "equation",
            Control::Picture(_) => "picture",
            _ => "other",
        })
        .collect();
    assert_eq!(kinds, ["endnote", "equation", "picture"]);
    // 미주(0..8) 가(8) 수식(9..17) 나(17) 그림(18..26) 다(26)
    assert_eq!(paragraph.char_offsets, [8, 17, 26]);
}

#[test]
fn endnote_keeps_its_number_and_answer() {
    let document = parse_hml(EXAM.as_bytes()).unwrap().document;
    let Control::Endnote(endnote) = &document.sections[0].paragraphs[0].controls[0] else {
        panic!("첫 컨트롤은 미주여야 한다");
    };
    assert_eq!(endnote.number, 3);
    assert_eq!(endnote.after_decoration_letter, '.' as u16);
    assert_eq!(endnote.paragraphs.len(), 1);
    let inner = &endnote.paragraphs[0];
    // 자동 번호 자리표시 공백 + 본문
    assert_eq!(inner.text, "  [정답] ④");
    let Control::AutoNumber(number) = &inner.controls[0] else {
        panic!("미주 문단은 자동 번호로 시작해야 한다");
    };
    assert_eq!(number.number_type, AutoNumberType::Endnote);
    assert_eq!(number.number, 3);
    assert_eq!(number.suffix_char, '.');
}

#[test]
fn picture_points_at_decoded_bin_data() {
    let document = parse_hml(EXAM.as_bytes()).unwrap().document;
    let Control::Picture(picture) = &document.sections[0].paragraphs[0].controls[2] else {
        panic!("세 번째 컨트롤은 그림이어야 한다");
    };
    assert_eq!(picture.image_attr.bin_data_id, 1);
    assert_eq!((picture.common.width, picture.common.height), (3000, 2000));
    assert!(picture.common.treat_as_char);
    assert_eq!(document.bin_data_content.len(), 1);
    let bytes = document.bin_data_content[0]
        .data
        .load_limited(1024)
        .expect("그림 바이트");
    assert_eq!(bytes, [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]);
    assert_eq!(document.bin_data_content[0].extension, "png");
}

#[test]
fn equation_size_does_not_overwrite_the_enclosing_table() {
    let xml = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?><HWPML Version="2.91"><HEAD SecCnt="1"/><BODY><SECTION Id="0">"#,
        r#"<P><TEXT><TABLE ColCount="1" RowCount="1"><SHAPEOBJECT><SIZE Height="5000" Width="40000"/></SHAPEOBJECT>"#,
        r#"<ROW><CELL ColAddr="0" RowAddr="0" Width="40000" Height="5000"><PARALIST><P><TEXT>"#,
        r#"<EQUATION><SHAPEOBJECT><SIZE Height="100" Width="700"/></SHAPEOBJECT><SCRIPT>x</SCRIPT></EQUATION>"#,
        r#"</TEXT></P></PARALIST></CELL></ROW></TABLE></TEXT></P></SECTION></BODY></HWPML>"#,
    );
    let document = parse_hml(xml.as_bytes()).unwrap().document;
    let Control::Table(table) = &document.sections[0].paragraphs[0].controls[0] else {
        panic!("표여야 한다");
    };
    assert_eq!(table.common.width, 40000);
}

#[test]
fn numbered_paragraph_shape_reads_its_numbering() {
    let xml = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?><HWPML Version="2.8"><HEAD SecCnt="1"><MAPPINGTABLE>"#,
        r#"<NUMBERINGLIST Count="1"><NUMBERING Id="1" Start="0">"#,
        r#"<PARAHEAD Alignment="Left" AutoIndent="true" Level="1" NumFormat="Digit" Start="1" TextOffset="50" TextOffsetType="percent" UseInstWidth="true" WidthAdjust="0" CharShape="0">^1.</PARAHEAD>"#,
        r#"<PARAHEAD Level="2" NumFormat="CircledDigit" Start="1">^2</PARAHEAD>"#,
        r#"</NUMBERING></NUMBERINGLIST>"#,
        r#"<PARASHAPELIST Count="1"><PARASHAPE Id="0" HeadingType="Number" Heading="1" Level="0"/></PARASHAPELIST>"#,
        r#"</MAPPINGTABLE></HEAD><BODY><SECTION Id="0"><P ParaShape="0"><TEXT><CHAR>문제</CHAR></TEXT></P></SECTION></BODY></HWPML>"#,
    );
    let document = parse_hml(xml.as_bytes()).unwrap().document;
    let shape = &document.doc_info.para_shapes[0];
    assert_eq!(shape.head_type, crate::model::style::HeadType::Number);
    assert_eq!(shape.numbering_id, 1);
    let numbering = &document.doc_info.numberings[0];
    assert_eq!(numbering.level_formats[0], "^1.");
    assert_eq!(numbering.level_formats[1], "^2");
    assert_eq!(numbering.level_start_numbers[0], 1);
    assert_eq!(numbering.heads[1].number_format, 1);
    assert_eq!(numbering.heads[0].attr & 0b1100, 0b1100, "UseInstWidth·AutoIndent");
}

#[test]
fn master_page_and_footer_stay_out_of_the_body() {
    let xml = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?><HWPML Version="2.8"><HEAD SecCnt="1"/><BODY><SECTION Id="0">"#,
        r#"<P><TEXT><SECDEF><HIDE Header="false" Footer="false" MasterPage="true" Border="false" Fill="false" EmptyLine="false"/>"#,
        r#"<MASTERPAGE Type="Even"><PARALIST TextWidth="62648" TextHeight="85890"><P><TEXT><CHAR>짝</CHAR></TEXT></P></PARALIST></MASTERPAGE>"#,
        r#"<MASTERPAGE Type="Odd"><PARALIST TextWidth="62648" TextHeight="85890"><P><TEXT><CHAR>홀</CHAR></TEXT></P></PARALIST></MASTERPAGE>"#,
        r#"</SECDEF><FOOTER ApplyPageType="Both"><PARALIST TextWidth="62648" TextHeight="3685"><P><TEXT><CHAR>꼬리</CHAR></TEXT></P></PARALIST></FOOTER>"#,
        r#"<CHAR>본문</CHAR></TEXT></P></SECTION></BODY></HWPML>"#,
    );
    let document = parse_hml(xml.as_bytes()).unwrap().document;
    let section = &document.sections[0];
    assert_eq!(section.paragraphs.len(), 1, "바탕쪽·꼬리말 문단이 본문으로 새면 안 된다");
    assert_eq!(section.paragraphs[0].text, "본문");
    assert!(section.section_def.hide_master_page);
    assert!(!section.section_def.hide_footer);

    let master_pages = &section.section_def.master_pages;
    assert_eq!(master_pages.len(), 2);
    assert_eq!(master_pages[0].apply_to, HeaderFooterApply::Even);
    assert_eq!(master_pages[0].paragraphs[0].text, "짝");
    assert_eq!(master_pages[1].apply_to, HeaderFooterApply::Odd);
    assert_eq!(master_pages[1].paragraphs[0].text, "홀");
    assert_eq!((master_pages[1].text_width, master_pages[1].text_height), (62648, 85890));

    let footers: Vec<_> = section.paragraphs[0]
        .controls
        .iter()
        .filter_map(|control| match control {
            Control::Footer(footer) => Some(footer),
            _ => None,
        })
        .collect();
    assert_eq!(footers.len(), 1);
    assert_eq!(footers[0].apply_to, HeaderFooterApply::Both);
    assert_eq!(footers[0].paragraphs[0].text, "꼬리");
}

#[test]
fn master_page_line_becomes_a_line_shape() {
    // 시험지 바탕쪽의 가운데 세로줄: 원래 100×100 을 높이 84548 로 늘린 직선
    let xml = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?><HWPML Version="2.8"><HEAD SecCnt="1"/><BODY><SECTION Id="0">"#,
        r#"<P><TEXT><SECDEF><MASTERPAGE Type="Both"><PARALIST><P><TEXT>"#,
        r#"<LINE EndX="100" EndY="100" IsReverseHV="false" StartX="0" StartY="0">"#,
        r#"<SHAPEOBJECT TextWrap="InFrontOfText"><SIZE Height="84548" Width="1"/>"#,
        r#"<POSITION HorzOffset="36416" HorzRelTo="Paper" TreatAsChar="false" VertOffset="9967" VertRelTo="Paper"/></SHAPEOBJECT>"#,
        r#"<DRAWINGOBJECT><SHAPECOMPONENT CurHeight="84548" CurWidth="0" OriHeight="100" OriWidth="100" XPos="0" YPos="0"/>"#,
        r#"<LINESHAPE Alpha="0" EndCap="Flat" Style="Solid" Width="33"/></DRAWINGOBJECT></LINE>"#,
        r#"<CHAR>머리</CHAR></TEXT></P></PARALIST></MASTERPAGE></SECDEF><CHAR>본문</CHAR></TEXT></P></SECTION></BODY></HWPML>"#,
    );
    let document = parse_hml(xml.as_bytes()).unwrap().document;
    let master = &document.sections[0].section_def.master_pages[0];
    let paragraph = &master.paragraphs[0];
    assert_eq!(paragraph.text, "머리", "직선은 글자가 아니라 개체 자리만 차지한다");
    let line = paragraph
        .controls
        .iter()
        .find_map(|control| match control {
            Control::Shape(shape) => match shape.as_ref() {
                crate::model::shape::ShapeObject::Line(line) => Some(line),
                _ => None,
            },
            _ => None,
        })
        .expect("LINE 이 직선 개체로 들어와야 한다");
    assert_eq!((line.start.x, line.start.y, line.end.x, line.end.y), (0, 0, 100, 100));
    assert_eq!((line.common.width, line.common.height), (1, 84548));
    assert_eq!((line.common.horizontal_offset, line.common.vertical_offset), (36416, 9967));
    assert_eq!(line.drawing.shape_attr.original_height, 100);
    assert_eq!(line.drawing.border_line.width, 33);
}

#[test]
fn page_number_gets_a_placeholder_for_the_renderer() {
    let xml = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?><HWPML Version="2.8"><HEAD SecCnt="1"/><BODY><SECTION Id="0">"#,
        r#"<P><TEXT><CHAR>- </CHAR><AUTONUM Number="1" NumberType="Page"><AUTONUMFORMAT Type="Digit"/></AUTONUM>"#,
        r#"<CHAR> / </CHAR><AUTONUM Number="1" NumberType="TotalPage"><AUTONUMFORMAT Type="Digit"/></AUTONUM><CHAR></CHAR>"#,
        r#"</TEXT></P></SECTION></BODY></HWPML>"#,
    );
    let document = parse_hml(xml.as_bytes()).unwrap().document;
    let paragraph = &document.sections[0].paragraphs[0];
    // HWP5 와 같다: 번호 자리는 공백 한 글자, 그 뒤 글자는 8 칸 뒤.
    assert_eq!(paragraph.text, "-   /  ");
    assert_eq!(paragraph.char_offsets, [0, 1, 2, 10, 11, 12, 13]);
    assert_eq!(paragraph.controls.len(), 2);
}
