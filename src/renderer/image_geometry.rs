//! 그림의 외접 프레임과 회전 전 출력 크기를 구분한다.
use super::render_tree::{BoundingBox, ShapeTransform};
use crate::model::image::Picture;

/// 저장본은 실제 합성 행렬이 그림의 방향을 정한다.
/// 편집 중에는 아직 저장 행렬이 없으므로 편집 플래그를 사용한다.
pub(crate) fn picture_transform(picture: &Picture) -> ShapeTransform {
    let shape = &picture.shape_attr;
    let mut transform = crate::renderer::layout::utils::extract_picture_transform(picture);
    if shape.raw_rendering.is_empty() || shape.group_level != 0 {
        return transform;
    }
    // imgRect는 그림의 저장 경계다. 꼭짓점 순서를 뒤집은 입력을 이미지
    // 반전으로 추정하지 않고, 실제 행렬이 변환한 두 기저축을 사용한다.
    let u = [shape.render_sx, shape.render_c];
    let v = [shape.render_b, shape.render_sy];
    let (sin, cos) = transform.rotation.to_radians().sin_cos();
    // 직교·축 정렬 계약이 성립하는 사각형만 분해한다. 전단·묶음 행렬을
    // 두 개의 flip 플래그로 근사하지 않으며 기존 별도 경로를 유지한다.
    let along_u = u[0] * cos + u[1] * sin;
    let across_u = -u[0] * sin + u[1] * cos;
    let along_v = -v[0] * sin + v[1] * cos;
    let across_v = v[0] * cos + v[1] * sin;
    if ![along_u, across_u, along_v, across_v]
        .iter()
        .all(|x| x.is_finite())
        || along_u == 0.0
        || along_v == 0.0
        || across_u.abs() > along_u.abs() * 1e-6
        || across_v.abs() > along_v.abs() * 1e-6
    {
        return transform;
    }
    transform.horz_flip = along_u < 0.0;
    transform.vert_flip = along_v < 0.0;
    transform
}

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
