//! 실제 공급 face가 있는 경우에만 호환 PUA 글리프를 선택한다.
use super::SvgRenderer;
use crate::renderer::TextStyle;
use std::borrow::Cow;

impl SvgRenderer {
    pub(super) fn prepare_pua_glyphs(&mut self, text: &str, style: &TextStyle) {
        if !crate::renderer::contains_old_hangul_jamo(text) {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !self.source_font_coverage.has_family(&style.font_family) {
            let plan = super::plan_svg_font_file_lookup(
                &style.font_family,
                &self.font_paths,
                style.is_visually_bold(),
            );
            if let Some(file) = super::find_font_file(&plan) {
                if file.metadata().is_ok_and(|m| m.len() <= 32 * 1024 * 1024) {
                    if let Ok(bytes) = std::fs::read(file) {
                        let _ = self
                            .source_font_coverage
                            .register(&style.font_family, &bytes, 0);
                    }
                }
            }
        }
        if self.font_embed_mode != super::FontEmbedMode::None {
            for (_, cluster) in crate::renderer::layout::split_into_clusters(text) {
                if let Some(ch) = self
                    .source_font_coverage
                    .glyph(&style.font_family, &cluster)
                {
                    for family in style.font_family.split(',').map(str::trim) {
                        self.font_codepoints
                            .entry(family.to_owned())
                            .or_default()
                            .insert(ch);
                    }
                }
            }
        }
    }
}

pub(super) fn display<'a>(
    coverage: &crate::renderer::pua_glyph::FontCoverage,
    text: &'a str,
    style: &TextStyle,
) -> Cow<'a, str> {
    coverage
        .glyph(&style.font_family, text)
        .map_or(Cow::Borrowed(text), |ch| Cow::Owned(ch.to_string()))
}
