use crate::model::bin_data::{BinData, BinDataContent, BinDataType};
use crate::model::control::{Control, Equation, UnknownControl};
use crate::model::footnote::{Endnote, Footnote};
use crate::model::image::{ImageAttr, Picture};
use crate::model::document::{Document, FileHeader, HwpVersion, Section, SectionDef};
use crate::model::paragraph::{CharShapeRef, Paragraph};
use crate::model::shape::{CommonObjAttr, DrawingObjAttr, RectangleShape, ShapeObject, TextBox};
use crate::model::style::{BorderFill, CharShape, Font, LineSpacingType, ParaShape, Style, TabDef};
use crate::model::table::{Cell, Table, TablePageBreak, VerticalAlign};

use super::error::HmlError;
use super::reader::{
    HmlControl, HmlEquation, HmlNote, HmlNoteKind, HmlParagraph, HmlPicture, HmlRectangle,
    HmlSource, HmlTable,
};

pub(crate) fn into_document(mut source: HmlSource) -> Result<Document, HmlError> {
    let mut document = Document {
        header: FileHeader {
            version: HwpVersion {
                major: 5,
                minor: 1,
                build: 0,
                revision: 0,
            },
            ..Default::default()
        },
        provenance: crate::model::provenance::SourceProvenance {
            format: crate::model::provenance::SourceFormat::Hml,
            hwp3_lineage: false,
            hwpx_lineage: false,
            hft_ascii_halfwidth_witnessed: false,
        },
        ..Default::default()
    };
    document.doc_info.hwpml_version = Some(source.version.clone());
    move_resources(&mut source, &mut document);
    move_bin_data(&source, &mut document);
    for source_section in source.sections {
        let section_def = SectionDef {
            page_def: source_section
                .page_def
                .unwrap_or_else(crate::model::page::PageDef::a4_default),
            ..Default::default()
        };
        let paragraphs = source_section
            .paragraphs
            .into_iter()
            .map(into_paragraph)
            .collect::<Result<Vec<_>, _>>()?;
        document.sections.push(Section {
            section_def,
            paragraphs,
            raw_provenance: None,
            raw_stream: None,
        });
    }
    document.doc_properties.section_count = document.sections.len() as u16;
    Ok(document)
}

fn move_resources(source: &mut HmlSource, document: &mut Document) {
    add_default_resources(document);
    if !source.font_faces.is_empty() {
        document.doc_info.font_faces = std::mem::take(&mut source.font_faces);
    }
    if !source.border_fills.is_empty() {
        document.doc_info.border_fills = std::mem::take(&mut source.border_fills);
    }
    if !source.char_shapes.is_empty() {
        document.doc_info.char_shapes = std::mem::take(&mut source.char_shapes);
    }
    if !source.para_shapes.is_empty() {
        document.doc_info.para_shapes = std::mem::take(&mut source.para_shapes);
    }
    if !source.tab_defs.is_empty() {
        document.doc_info.tab_defs = std::mem::take(&mut source.tab_defs);
    }
    if !source.styles.is_empty() {
        document.doc_info.styles = std::mem::take(&mut source.styles);
    }
}

/// `BINITEM` 목록과 `TAIL/BINDATASTORAGE` 의 Base64 본문을 그림 데이터로 옮긴다.
///
/// 렌더러는 `bin_data_content` 를 `BinItem` 번호의 1-기반 순번으로 찾으므로
/// (`find_bin_data_index`), 번호 1..=최댓값 자리를 빠짐없이 채운다. 본문이 없거나
/// 풀지 못한 항목은 빈 바이트로 남겨 "그림 없음"으로 그려지게 한다.
fn move_bin_data(source: &HmlSource, document: &mut Document) {
    let Some(max_id) = source.bin_items.iter().map(|item| item.id).max() else {
        return;
    };
    let storage = source
        .preserved_fragments
        .iter()
        .filter(|fragment| fragment.xml_path == "/HWPML/TAIL/BINDATASTORAGE")
        .flat_map(|fragment| read_bin_data_storage(&fragment.raw_xml))
        .collect::<std::collections::HashMap<_, _>>();
    for id in 1..=max_id {
        let item = source.bin_items.iter().find(|item| item.id == id);
        let extension = item
            .map(|item| item.format.clone())
            .filter(|format| !format.is_empty())
            .unwrap_or_else(|| "bin".to_string());
        document.doc_info.bin_data_list.push(BinData {
            data_type: if item.is_none_or(|item| item.embedded) {
                BinDataType::Embedding
            } else {
                BinDataType::Link
            },
            storage_id: id,
            extension: Some(extension.clone()),
            ..Default::default()
        });
        document.bin_data_content.push(BinDataContent {
            id,
            data: storage.get(&id).cloned().unwrap_or_default().into(),
            extension,
        });
    }
}

/// `<BINDATASTORAGE><BINDATA Id="1" Encoding="Base64" Compress="true">…</BINDATA>…`
fn read_bin_data_storage(xml: &str) -> Vec<(u16, Vec<u8>)> {
    use base64::Engine;
    use quick_xml::events::Event;

    let mut reader = quick_xml::Reader::from_str(xml);
    let mut items = Vec::new();
    let mut current: Option<(u16, bool, String)> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) if element.name().as_ref().as_bytes() == b"BINDATA" => {
                let mut id = 0u16;
                let mut compressed = false;
                for attr in element.attributes().flatten() {
                    let value = String::from_utf8_lossy(attr.value.as_ref().as_bytes()).into_owned();
                    match attr.key.as_ref().as_bytes() {
                        b"Id" => id = value.trim().parse().unwrap_or(0),
                        b"Compress" => compressed = value == "true",
                        _ => {}
                    }
                }
                current = Some((id, compressed, String::new()));
            }
            Ok(Event::Text(text)) => {
                if let Some((_, _, body)) = current.as_mut() {
                    body.push_str(text.as_ref());
                }
            }
            Ok(Event::End(element)) if element.name().as_ref().as_bytes() == b"BINDATA" => {
                if let Some((id, compressed, body)) = current.take() {
                    let packed: String = body.chars().filter(|c| !c.is_whitespace()).collect();
                    let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(packed) else {
                        continue;
                    };
                    let bytes = if compressed {
                        crate::parser::cfb_reader::decompress_stream_limited(
                            &bytes,
                            crate::model::bin_data::MAX_BIN_DATA_BYTES,
                        )
                        .unwrap_or_default()
                    } else {
                        bytes
                    };
                    if id != 0 {
                        items.push((id, bytes));
                    }
                }
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }
    items
}

fn add_default_resources(document: &mut Document) {
    let font = Font {
        name: "함초롬바탕".to_string(),
        ..Default::default()
    };
    document.doc_info.font_faces = (0..7).map(|_| vec![font.clone()]).collect();
    document.doc_info.char_shapes = vec![CharShape {
        ratios: [100; 7],
        relative_sizes: [100; 7],
        base_size: 1000,
        ..Default::default()
    }];
    document.doc_info.para_shapes = vec![ParaShape {
        line_spacing_type: LineSpacingType::Percent,
        line_spacing: 160,
        ..Default::default()
    }];
    document.doc_info.border_fills = vec![BorderFill::default()];
    document.doc_info.tab_defs = vec![TabDef::default()];
    document.doc_info.styles = vec![Style {
        local_name: "바탕글".to_string(),
        english_name: "Normal".to_string(),
        lang_id: 1042,
        ..Default::default()
    }];
}

fn into_paragraph(source: HmlParagraph) -> Result<Paragraph, HmlError> {
    let controls = source
        .controls
        .into_iter()
        .map(into_control)
        .collect::<Result<Vec<_>, _>>()?;
    let mut char_shapes = source.char_shapes;
    if char_shapes.is_empty() {
        char_shapes.push(CharShapeRef::default());
    }
    Ok(Paragraph {
        para_shape_id: source.para_shape_id,
        style_id: source.style_id,
        column_type: source.column_type,
        raw_break_type: source.raw_break_type,
        char_count: source.raw_pos + 1,
        has_para_text: !source.text.is_empty() || !controls.is_empty(),
        text: source.text,
        char_offsets: source.char_offsets,
        char_shapes,
        ctrl_data_records: vec![None; controls.len()],
        controls,
        ..Default::default()
    })
}

fn into_control(source: HmlControl) -> Result<Control, HmlError> {
    match source {
        HmlControl::Equation(equation) => Ok(into_equation(equation)),
        HmlControl::Rectangle(rectangle) => into_rectangle(rectangle),
        HmlControl::Table(table) => into_table(table),
        HmlControl::ColumnDef(column_def) => Ok(Control::ColumnDef(column_def)),
        HmlControl::Note(note) => into_note(note),
        HmlControl::AutoNumber(auto_number) => Ok(Control::AutoNumber(auto_number)),
        HmlControl::Picture(picture) => Ok(into_picture(picture)),
        HmlControl::Unknown => Ok(Control::Unknown(UnknownControl::default())),
    }
}

fn into_note(source: HmlNote) -> Result<Control, HmlError> {
    let paragraphs = source
        .paragraphs
        .into_iter()
        .map(into_paragraph)
        .collect::<Result<Vec<_>, _>>()?;
    let after_decoration_letter = source.suffix_char as u16;
    Ok(match source.kind {
        HmlNoteKind::Footnote => Control::Footnote(Box::new(Footnote {
            number: source.number,
            paragraphs,
            after_decoration_letter,
            ..Default::default()
        })),
        HmlNoteKind::Endnote => Control::Endnote(Box::new(Endnote {
            number: source.number,
            paragraphs,
            after_decoration_letter,
            ..Default::default()
        })),
    })
}

fn into_picture(source: HmlPicture) -> Control {
    let common = CommonObjAttr {
        ctrl_id: crate::parser::tags::CTRL_GEN_SHAPE,
        width: source.width,
        height: source.height,
        horizontal_offset: source.horizontal_offset as u32,
        vertical_offset: source.vertical_offset as u32,
        treat_as_char: source.treat_as_char,
        flow_with_text: source.flow_with_text,
        allow_overlap: source.allow_overlap,
        vert_rel_to: source.vert_rel_to,
        vert_align: source.vert_align,
        horz_rel_to: source.horz_rel_to,
        horz_align: source.horz_align,
        text_wrap: source.text_wrap,
        ..Default::default()
    };
    Control::Picture(Box::new(Picture {
        common,
        shape_attr: source.shape_attr,
        border_x: source.border_x,
        border_y: source.border_y,
        crop: source.crop,
        padding: source.padding,
        img_dim: source.img_dim,
        image_attr: ImageAttr {
            brightness: source.brightness,
            contrast: source.contrast,
            effect: source.effect,
            bin_data_id: source.bin_item,
            // HML Alpha 는 0~255 불투명도 반전값이고, IR 은 0~100 투명도다.
            transparency: ((source.alpha as u32 * 100 + 127) / 255) as u8,
            external_path: None,
        },
        ..Default::default()
    }))
}

fn into_equation(source: HmlEquation) -> Control {
    let (width, height) =
        crate::renderer::equation::intrinsic_size_hwp(&source.script, source.font_size);
    Control::Equation(Box::new(Equation {
        common: CommonObjAttr {
            ctrl_id: crate::parser::tags::CTRL_EQUATION,
            treat_as_char: true,
            width,
            height,
            ..Default::default()
        },
        script: source.script,
        font_size: source.font_size,
        color: source.color,
        baseline: source.baseline,
        version_info: source.version_info,
        font_name: source.font_name,
        ..Default::default()
    }))
}

fn into_rectangle(source: HmlRectangle) -> Result<Control, HmlError> {
    let text_box = if source.text_box.is_empty() {
        None
    } else {
        let paragraphs = source
            .text_box
            .into_iter()
            .map(into_paragraph)
            .collect::<Result<Vec<_>, _>>()?;
        Some(TextBox {
            paragraphs,
            max_width: source.width,
            vertical_align: VerticalAlign::Center,
            margin_left: source.text_margin.left,
            margin_right: source.text_margin.right,
            margin_top: source.text_margin.top,
            margin_bottom: source.text_margin.bottom,
            ..Default::default()
        })
    };
    let common = CommonObjAttr {
        ctrl_id: 0x2472_6563,
        width: source.width,
        height: source.height,
        horizontal_offset: source.horizontal_offset as u32,
        vertical_offset: source.vertical_offset as u32,
        treat_as_char: source.treat_as_char,
        flow_with_text: source.flow_with_text,
        allow_overlap: source.allow_overlap,
        vert_rel_to: source.vert_rel_to,
        vert_align: source.vert_align,
        horz_rel_to: source.horz_rel_to,
        horz_align: source.horz_align,
        text_wrap: source.text_wrap,
        ..Default::default()
    };
    let drawing = DrawingObjAttr {
        shape_attr: source.shape_attr,
        border_line: source.border_line,
        fill: source.fill,
        text_box,
        ..Default::default()
    };
    let x_coords = resolved_coords(source.x_coords, source.width as i32);
    let y_coords = resolved_coords(source.y_coords, source.height as i32);
    Ok(Control::Shape(Box::new(ShapeObject::Rectangle(
        RectangleShape {
            common,
            drawing,
            x_coords,
            y_coords,
            ..Default::default()
        },
    ))))
}

fn resolved_coords(coords: [i32; 4], extent: i32) -> [i32; 4] {
    if coords == [0; 4] {
        [0, extent, extent, 0]
    } else {
        coords
    }
}

fn into_table(source: HmlTable) -> Result<Control, HmlError> {
    let mut cells = Vec::with_capacity(source.cells.len());
    for source_cell in source.cells {
        let paragraphs = source_cell
            .paragraphs
            .into_iter()
            .map(into_paragraph)
            .collect::<Result<Vec<_>, _>>()?;
        cells.push(Cell {
            col: source_cell.col,
            row: source_cell.row,
            col_span: source_cell.col_span,
            row_span: source_cell.row_span,
            width: source_cell.width,
            height: source_cell.height,
            padding: source_cell.padding,
            border_fill_id: source_cell.border_fill_id,
            paragraphs,
            apply_inner_margin: true,
            // [#3189] 종전엔 Center 하드코딩이라 원본의 Top/Bottom 이 유실됐다.
            // reader 가 PARALIST@VertAlign 을 읽어 넣은 값을 그대로 옮긴다
            // (속성이 없으면 reader 가 Center 로 접으므로 기존 동작과 같다).
            vertical_align: source_cell.vertical_align,
            ..Default::default()
        });
    }
    // [#2833] row_count 는 <TABLE RowCount="..."> 를 검증 없이 받은 u16 이다.
    // 종전 코드는 row_count 만큼 순회하며 매 행마다 cells 전체를 다시 스캔해
    // O(row_count × cells.len()) 이었다 — RowCount 만 부풀린 입력이 파싱 자체를
    // 느리게 만들었다(#2751 은 export 경로, 이건 import 경로의 동형 잔여).
    // cells 를 한 번만 순회해 행별로 누적하면 O(row_count + cells.len()).
    // cell.row >= row_count 인 셀은 종전에도 어떤 행에도 카운트되지 않았으므로
    // (0..row_count 범위 밖) get_mut 이 None 인 경우 그대로 무시해 동일하게 맞춘다.
    let mut row_sizes = vec![0i16; source.row_count as usize];
    for cell in &cells {
        if let Some(slot) = row_sizes.get_mut(cell.row as usize) {
            *slot = slot.saturating_add(1);
        }
    }
    let attr = u32::from(source.common.treat_as_char);
    let mut table = Table {
        attr,
        common: source.common,
        row_count: source.row_count,
        col_count: source.col_count,
        cell_spacing: source.cell_spacing,
        padding: source.padding,
        row_sizes,
        border_fill_id: source.border_fill_id,
        cells,
        page_break: TablePageBreak::CellBreak,
        repeat_header: true,
        ..Default::default()
    };
    table.rebuild_grid();
    Ok(Control::Table(Box::new(table)))
}
