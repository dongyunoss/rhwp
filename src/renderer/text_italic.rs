//! 독립 한컴 Print의 본문 합성 기울임. 실제 italic face에는 중복 적용하지 않는다.
pub(crate) use super::text_transform::SYNTHETIC_ITALIC_SKEW as SKEW_X;
// atan(0.333)의 도 단위 표현. CSS는 실제 italic face가 있으면 그 프로그램을 선택한다.
pub(crate) const CSS_STYLE: &str = "oblique 18.418deg";
pub(crate) fn css_prefix(italic: bool) -> &'static str {
    if italic {
        "oblique 18.418deg "
    } else {
        ""
    }
}
