//! 독립 한컴 Print와 대조한 저장 TAC 원점·내용 보존·직선의 출력 프레임.
//! 전쪽 Native/fresh 90% 이상, 대상 쪽 양쪽 98% 이상과 직접 판독 증거는
//! mydocs/pr/assets/stored_tac_source_frame_20261021/README.md에 있다.
#![cfg(not(target_arch = "wasm32"))]

use rhwp::document_core::DocumentCore;
use rhwp::model::{control::Control, paragraph::Paragraph, shape::ShapeObject};
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};
use rhwp::renderer::ArrowStyle;
use serde_json::Value;
use std::path::Path;

const INPUT: &str = "samples/2025 행정업무운영 편람(최종).hwp";

fn bytes() -> Vec<u8> {
    std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT)).expect("편람 원본")
}

fn open() -> DocumentCore {
    let mut core = DocumentCore::from_bytes(&bytes()).expect("편람 로드");
    core.set_hangul2024_compat(true);
    core
}

fn evidence() -> Value {
    serde_json::from_str(include_str!(
        "../../mydocs/pr/assets/stored_tac_source_frame_20261021/independent_coordinates.json"
    ))
    .expect("독립 Print 좌표")
}

fn number(value: &Value, key: &str) -> f64 {
    value[key].as_f64().expect("독립 좌표 수치")
}

fn descendants(node: &RenderNode) -> Vec<&RenderNode> {
    let mut result = vec![node];
    for child in &node.children {
        result.extend(descendants(child));
    }
    result
}

fn text(node: &RenderNode) -> String {
    let mut result = match &node.node_type {
        RenderNodeType::TextRun(run) => run.text.clone(),
        _ => String::new(),
    };
    for child in &node.children {
        result.push_str(&text(child));
    }
    result
}

fn compact(value: &str) -> String {
    value.chars().filter(|c| !c.is_whitespace()).collect()
}

fn source_text(para: &Paragraph) -> String {
    let mut result = para.text.clone();
    for control in &para.controls {
        match control {
            Control::Table(table) => {
                for cell in &table.cells {
                    for child in &cell.paragraphs {
                        result.push_str(&source_text(child));
                    }
                }
            }
            Control::Shape(shape) => result.push_str(&shape_text(shape)),
            _ => {}
        }
    }
    result
}

fn shape_text(shape: &ShapeObject) -> String {
    let mut result = String::new();
    if let Some(tb) = shape
        .drawing()
        .and_then(|drawing| drawing.text_box.as_ref())
    {
        for para in &tb.paragraphs {
            result.push_str(&source_text(para));
        }
    }
    if let ShapeObject::Group(group) = shape {
        for child in &group.children {
            result.push_str(&shape_text(child));
        }
    }
    result
}

fn table_node(root: &RenderNode, section: usize, para: usize, control: usize) -> &RenderNode {
    let found: Vec<_> = descendants(root)
        .into_iter()
        .filter(|node| {
            matches!(&node.node_type, RenderNodeType::Table(t)
                if t.section_index == Some(section)
                    && t.para_index == Some(para)
                    && t.control_index == Some(control))
        })
        .collect();
    assert_eq!(
        found.len(),
        1,
        "표의 쪽 소속·중복: {section}.{para}/{control}"
    );
    found[0]
}

fn assert_cells(node: &RenderNode, section: usize, para: usize, control: usize) {
    let parsed = rhwp::parser::parse_document(&bytes()).expect("원본 셀 내용");
    let Control::Table(source) = &parsed.sections[section].paragraphs[para].controls[control]
    else {
        panic!("원본 표 소유");
    };
    let RenderNodeType::Table(rendered) = &node.node_type else {
        panic!("출력 표");
    };
    assert_eq!(rendered.row_count, source.row_count);
    assert_eq!(rendered.col_count, source.col_count);
    for cell in &source.cells {
        let expected = compact(&cell.paragraphs.iter().map(source_text).collect::<String>());
        let found: Vec<_> = node
            .children
            .iter()
            .filter(|child| {
                matches!(&child.node_type, RenderNodeType::TableCell(c)
                    if c.row == cell.row && c.col == cell.col)
            })
            .collect();
        assert!(found.len() <= 1, "셀 중복: {}.{}", cell.row, cell.col);
        if !expected.is_empty() {
            assert_eq!(found.len(), 1, "셀 누락: {}.{}", cell.row, cell.col);
        }
        let actual = compact(&found.iter().map(|child| text(child)).collect::<String>());
        assert_eq!(
            actual, expected,
            "셀 내용 순서·보존: {}.{}",
            cell.row, cell.col
        );
    }
}

fn assert_print_edges(node: &RenderNode, expected: &Value) {
    let tolerance = number(&evidence(), "tolerance_px");
    assert!(
        (node.bbox.y - number(expected, "top")).abs() <= tolerance,
        "독립 Print 윗변: {:?}",
        node.bbox
    );
    assert!(
        (node.bbox.y + node.bbox.height - number(expected, "bottom")).abs() <= tolerance,
        "독립 Print 아랫변: {:?}",
        node.bbox
    );
}

#[test]
fn decorated_tac_line_preserves_print_edges_and_following_table_contents() {
    let core = open();
    assert_eq!(core.page_count(), 383, "독립 Print 전체 쪽수");
    let page = core.build_page_render_tree(110).expect("111쪽");
    let expected = evidence();
    let mut previous = None;
    for row in expected["tables_111"].as_array().expect("세 표") {
        let para = row["paragraph"].as_u64().unwrap() as usize;
        let control = row["control"].as_u64().unwrap() as usize;
        let table = table_node(&page.root, 2, para, control);
        assert_print_edges(table, row);
        assert_cells(table, 2, para, control);
        if let Some(bottom) = previous {
            assert!(table.bbox.y > bottom, "앞뒤 표의 순서·겹침");
        }
        previous = Some(table.bbox.y + table.bbox.height);
    }
}

#[test]
fn saved_tac_title_preserves_print_frame_and_following_body_order() {
    let core = open();
    let page = core.build_page_render_tree(241).expect("242쪽");
    let table = table_node(&page.root, 7, 414, 0);
    assert_print_edges(table, &evidence()["table_242"]);
    assert_cells(table, 7, 414, 0);
    let content = compact(&text(&page.root));
    let title = content
        .find("정책연구관리시스템(PRISM)")
        .expect("제목 보존");
    let purpose = content.find("가.구축목적").expect("뒤 본문 소속");
    let functions = content.find("나.주요기능").expect("다음 본문 보존");
    assert!(title < purpose && purpose < functions, "제목과 본문 순서");
}

#[test]
fn stored_cell_arrow_keeps_print_direction_without_a_second_transform() {
    let core = open();
    let page = core.build_page_render_tree(110).expect("111쪽");
    let found: Vec<_> = descendants(&page.root)
        .into_iter()
        .filter_map(|node| match &node.node_type {
            RenderNodeType::Line(line)
                if line.section_index == Some(2)
                    && line.para_index == Some(841)
                    && line.control_index == Some(0)
                    && line.cell_index == Some(0)
                    && line.cell_para_index == Some(0)
                    && line.outer_table_control_index == Some(0) =>
            {
                Some(line)
            }
            _ => None,
        })
        .collect();
    assert_eq!(found.len(), 1, "실제 셀 직선의 소속·누락·중복");
    let line = found[0];
    // 완성된 종이 좌표를 다시 회전하면 PDF의 위 화살표가 오른쪽으로 바뀐다.
    assert!(
        !line.transform.has_transform(),
        "완성된 선에 남은 추가 변환"
    );
    let expected = evidence();
    let arrow = &expected["arrow_111"];
    let tolerance = number(&expected, "tolerance_px");
    for x in [line.x1, line.x2] {
        assert!((x - number(arrow, "x")).abs() <= tolerance);
    }
    assert!((line.y1 - number(arrow, "start_y")).abs() <= tolerance);
    assert!((line.y2 - number(arrow, "end_y")).abs() <= tolerance);
    assert!(line.y1 < line.y2, "시작점의 위 화살표 방향");
    assert_eq!(line.style.start_arrow, ArrowStyle::Arrow);
    assert_eq!(line.style.end_arrow, ArrowStyle::None);
    let table = table_node(&page.root, 2, 841, 0);
    assert!(line.y1 >= table.bbox.y);
    assert!(line.y2 <= table.bbox.y + table.bbox.height);
    let Control::Table(source) = &core.document().sections[2].paragraphs[841].controls[0] else {
        panic!("원본 표 소유");
    };
    let Control::Shape(shape) = &source.cells[0].paragraphs[0].controls[0] else {
        panic!("원본 직선 보존");
    };
    let ShapeObject::Line(source) = shape.as_ref() else {
        panic!("원본 직선 종류 보존");
    };
    assert_eq!(
        source.drawing.shape_attr.rotation_angle, 90,
        "편집 속성 보존"
    );
}
