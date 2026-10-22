//! 원본 소유의 예약 높이와 완료 셀의 물리 테두리 높이를 함께 발행한다.

use super::{hwpunit_to_px, HeightMeasurer, ResolvedStyleSet, Table};

impl HeightMeasurer {
    pub(super) fn reflow_cell_frame_rows(
        &self,
        table: &Table,
        styles: &ResolvedStyleSet,
        depth: usize,
        source_rows: &[f64],
    ) -> Option<Vec<f64>> {
        let mut frames = source_rows.to_vec();
        let mut resolved = false;
        for (row, height) in frames.iter_mut().enumerate() {
            // 걸침 칸의 물리 frame은 그 칸의 실제 조각 소유가 결정한다.
            if table.cells.iter().any(|cell| {
                let first = usize::from(cell.row);
                first <= row && row < first + usize::from(cell.row_span) && cell.row_span > 1
            }) {
                continue;
            }
            let cells: Vec<_> = table
                .cells
                .iter()
                .filter(|cell| usize::from(cell.row) == row)
                .collect();
            if cells.is_empty() {
                continue;
            }
            let known_frames: Option<Vec<_>> = cells
                .into_iter()
                .map(|cell| {
                    let declared = i32::try_from(cell.height).ok()?;
                    let flow = self.cell_reflow_block_flow(cell, styles, depth)?;
                    let padding = cell.effective_padding(&table.padding);
                    Some(hwpunit_to_px(declared, self.dpi).max(
                        flow.bottom
                            + hwpunit_to_px(
                                i32::from(padding.top) + i32::from(padding.bottom),
                                self.dpi,
                            ),
                    ))
                })
                .collect();
            if let Some(known) = known_frames {
                *height = known.into_iter().fold(0.0_f64, f64::max);
                resolved = true;
            }
        }
        resolved.then_some(frames)
    }
}
