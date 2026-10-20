//! 같은 문서에 실제 적재된 프로그램에서 확인한 PUA만 그린다.
use super::WebCanvasRenderer;
use crate::renderer::TextStyle;
use std::borrow::Cow;

impl WebCanvasRenderer {
    pub(super) fn glyph_display<'a>(&self, text: &'a str, style: &TextStyle) -> Cow<'a, str> {
        self.hft_outline_fonts
            .as_ref()
            .and_then(|fonts| fonts.pua_coverage.glyph(&style.font_family, text))
            .map_or(Cow::Borrowed(text), |ch| Cow::Owned(ch.to_string()))
    }
}
