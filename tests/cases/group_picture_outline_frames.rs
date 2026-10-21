//! 독립 Print와 대조한 묶음 자식 그림의 외곽선과 주석 쌓기 순서.
//! 전383쪽 Native/fresh raw90% 이상·94쪽 양쪽98% 이상·직접 판독 후 추가했다.
//! mydocs/pr/assets/group_picture_outline_frames_20261021/README.md
#![cfg(not(target_arch = "wasm32"))]

use rhwp::{
    document_core::DocumentCore,
    model::{control::Control, shape::ShapeObject},
    renderer::render_tree::{RenderNode, RenderNodeType},
};
use serde_json::Value;
use std::path::Path;

fn descendants(node: &RenderNode) -> Vec<&RenderNode> {
    let mut nodes = vec![node];
    for child in &node.children {
        nodes.extend(descendants(child));
    }
    nodes
}

fn check(paragraph: usize) {
    let input =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("samples/2025 행정업무운영 편람(최종).hwp");
    let mut core = DocumentCore::from_bytes(&std::fs::read(input).unwrap()).unwrap();
    core.set_hangul2024_compat(true);
    let Control::Shape(shape) = &core.document().sections[2].paragraphs[paragraph].controls[0]
    else {
        panic!("독립 Print의 원문 묶음 소유자");
    };
    let ShapeObject::Group(source) = shape.as_ref() else {
        panic!("스크린샷과 주석이 같은 묶음을 소유한다");
    };
    let ShapeObject::Picture(picture) = &source.children[0] else {
        panic!("원문 순서의 첫 자식은 스크린샷이다");
    };
    assert_eq!(picture.border_attr.attr & 0x3f, 1, "원본에 저장된 실선");
    let tree = core.build_page_render_tree(93).unwrap();
    let groups: Vec<_> = descendants(&tree.root)
        .into_iter()
        .filter(|node| {
            matches!(node.node_type, RenderNodeType::Group(_))
                && node.children.iter().any(|child| {
                    matches!(&child.node_type, RenderNodeType::Image(image)
                        if image.bin_data_id == picture.image_attr.bin_data_id)
                })
        })
        .collect();
    assert_eq!(groups.len(), 1, "원본 그림의 묶음 소속·누락·중복");
    let group = groups[0];
    let images: Vec<_> = group
        .children
        .iter()
        .enumerate()
        .filter(|(_, node)| matches!(node.node_type, RenderNodeType::Image(_)))
        .collect();
    assert_eq!(images.len(), 1, "원본 스크린샷을 한 번만 그린다");
    let (image_index, image) = images[0];
    let outlines: Vec<_> = group
        .children
        .iter()
        .enumerate()
        .filter(|(_, node)| {
            matches!(&node.node_type, RenderNodeType::Rectangle(rectangle)
            if rectangle.style.stroke_color == Some(picture.border_color)
                && rectangle.style.stroke_width > 0.0
                && rectangle.style.fill_color.is_none())
        })
        .collect();
    assert_eq!(outlines.len(), 1, "독립 Print에 있는 그림 외곽선 누락·중복");
    let (outline_index, outline) = outlines[0];
    assert_eq!(
        outline_index,
        image_index + 1,
        "테두리는 자기 bitmap 바로 위"
    );
    assert_eq!(
        group.children.len(),
        source.children.len() + 1,
        "그림과 기존 주석 자식은 보존하고 외곽선 하나만 추가한다"
    );
    assert!(
        group.children[outline_index + 1..]
            .iter()
            .all(|node| matches!(node.node_type, RenderNodeType::Rectangle(_))),
        "원문 주석 상자는 그림·외곽선 뒤의 쌓기 순서를 보존한다"
    );
    assert_eq!(
        (
            outline.bbox.x,
            outline.bbox.y,
            outline.bbox.width,
            outline.bbox.height
        ),
        (
            image.bbox.x,
            image.bbox.y,
            image.bbox.width,
            image.bbox.height
        ),
        "그림과 선이 같은 최종 개체 틀을 쓴다"
    );

    let evidence: Value = serde_json::from_str(include_str!(
        "../../mydocs/pr/assets/group_picture_outline_frames_20261021/independent_coordinates.json"
    ))
    .unwrap();
    let expected = &evidence["objects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["paragraph"].as_u64() == Some(paragraph as u64))
        .expect("이 원문 소유자에 대응하는 독립 Print 테두리")["outline_bbox_css_px"];
    for (name, actual) in [
        ("x", outline.bbox.x),
        ("y", outline.bbox.y),
        ("width", outline.bbox.width),
        ("height", outline.bbox.height),
    ] {
        let print = expected[name].as_f64().unwrap();
        assert!(
            (actual - print).abs() <= 2.0,
            "94쪽 양쪽 raw98% 이상: 독립 Print {name}={print}, 실제={actual}, 허용 ±2px"
        );
    }
}

#[test]
fn upper_grouped_screenshot_keeps_outline_owner_stacking_and_print_frame() {
    check(657);
}

#[test]
fn lower_grouped_screenshot_keeps_outline_owner_stacking_and_print_frame() {
    check(661);
}
