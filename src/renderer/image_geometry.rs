//! 그림의 외접 프레임과 회전 전 출력 크기를 구분한다.
use super::render_tree::{BoundingBox, ShapeTransform};
use crate::model::image::Picture;

/// 저장·편집 경로가 기록한 회전 전 크기와 회전 후 외접 크기의 비율.
/// 묶음의 독립 행렬이나 다른 크기 계약에는 이 비율을 추정하지 않는다.
pub(crate) fn picture_pre_rotation_ratio(picture: &Picture) -> Option<[f64; 2]> {
    let shape = &picture.shape_attr;
    let angle = shape.rotation_angle.rem_euclid(360);
    let (w, h) = (shape.current_width as f64, shape.current_height as f64);
    let (fw, fh) = (picture.common.width as f64, picture.common.height as f64);
    if angle == 0 || w <= 0.0 || h <= 0.0 || fw <= 0.0 || fh <= 0.0 {
        return None;
    }
    let (sin, cos) = (angle as f64).to_radians().sin_cos();
    let expected_w = w * cos.abs() + h * sin.abs();
    let expected_h = w * sin.abs() + h * cos.abs();
    // 저장 크기는 정수 HWPUNIT이다. 저장 시 반올림 한 단위까지의 차이만 허용한다.
    if (expected_w - fw).abs() > 1.0 || (expected_h - fh).abs() > 1.0 {
        return None;
    }
    Some([w / fw, h / fh])
}

pub(crate) fn pre_rotation_bbox(
    frame: &BoundingBox,
    transform: ShapeTransform,
    ratio: Option<[f64; 2]>,
) -> BoundingBox {
    let Some([rx, ry]) = ratio else {
        // 이전 생산자와 묶음 개체의 기존 계약은 유지한다.
        return transform.effective_image_bbox(frame);
    };
    let width = frame.width * rx;
    let height = frame.height * ry;
    BoundingBox::new(
        frame.x + (frame.width - width) / 2.0,
        frame.y + (frame.height - height) / 2.0,
        width,
        height,
    )
}
