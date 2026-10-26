# PR #7833 리뷰 — Fix inline object positions and boundary cut/paste in cells

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#7833](https://github.com/edwardkim/rhwp/pull/7833) / semanticist21 |
| 고정 원 head | `255413cdbc74d34f7c1d1be09fb7ad640eac6feb` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=true |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 이전 검증 후보(과거) | `d6ea10837218e7e490b166ed76af221490b61ba2` |
| 초기 렌더 소스·증거 등록(과거) | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 이전 고정 devel(과거) | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `cbb2d3ee9b702027a8101526fbd2b5f9152603c8` | `94d0267d8534ca1b0ab64f8ea39b27b16244b3ee` |
| `3ae9e3b71ea08fdd1ae06a02aab33b7c31b22ad5` | `5d80892479114bd613c13ece7044c9889c3dad04` |
| `a0c72f3cb85347958194a88d0fec4a81f28155ba` | `d5a461f2073987e11bf86e2f8e2184e1f40fcfde` |
| `efec7825d325beb5f2eff65ac921f148818367a5` | `8abe0091e1b15b36f434b32614aa0b8433b306d6` |
| `c03d1f98ed5b19c091c7028e370089dc6a3dfbc3` | `5421224c2fae1a357752f90c6265ab64e29c4a86` |
| `79b7dd66f20f89787521175762879f73df478674` | `98b029df5daed2fb7007aa6cbaf99ca816b25b12` |
| `255413cdbc74d34f7c1d1be09fb7ad640eac6feb` |  |

### 실제 소비 경로와 반례

cell boundary cut은 문자열만 지우지 않고 첫/끝 문단의 개체 슬롯과 서식을 같은 선택 범위로 제거한다. flat/nested 주소와 source UTF-16·logical 오프셋을 분리하고 Field·range tag·markpen의 미지원 경계는 변경 전에 거절한다. 빈 범위가 끝 개체를 지우지 않는 대조군을 유지한다. 글꼴 trace는 CharOverlap 뒤 확정할 수 없는 원문 주소를 null/unavailable로 내보내며, WASM 검사의 독립 CFB 원문 경계 fixture가 임의 scalar 주소를 주장하지 못하게 한다. 마지막 source commit의 trace 계약은 base 6097253468에 포함된 대응으로 처리하고 중복 체리픽하지 않는다.



### 현재 소스와 검사

- [src/document_core/commands/clipboard.rs](../../../src/document_core/commands/clipboard.rs)
- [src/document_core/commands/text_editing.rs](../../../src/document_core/commands/text_editing.rs)
- [src/document_core/helpers.rs](../../../src/document_core/helpers.rs)
- [src/document_core/queries/cursor_rect.rs](../../../src/document_core/queries/cursor_rect.rs)
- [src/document_core/queries/font_decision.rs](../../../src/document_core/queries/font_decision.rs)
- [src/renderer/layout/paragraph_layout.rs](../../../src/renderer/layout/paragraph_layout.rs)
- [tests/cases/cell_boundary_object_cut.rs](../../../tests/cases/cell_boundary_object_cut.rs)
- [tests/cases/issue_4961_font_decision_trace.rs](../../../tests/cases/issue_4961_font_decision_trace.rs)
- [tests/issue_1452_saved_caret.rs](../../../tests/issue_1452_saved_caret.rs)

- `tests/cases/cell_boundary_object_cut.rs`: SHA-256 `409364a7b7993261f42e3c89eee92de12e5e886188decd8545746bf2e406fde7`. 직접 선언 검사 17개: `flat_cell_cut_removes_boundary_pictures_and_tables`, `path_cell_cut_removes_boundary_pictures_and_tables_at_each_depth`, `same_paragraph_cell_cut_removes_only_selected_object`, `invalid_cell_ranges_keep_document_and_clipboard`, `cell_boundary_with_field_is_rejected_before_mutation`, `empty_cell_ranges_keep_tail_objects_raw_stream_and_events`, `cell_boundary_range_tags_are_rejected_before_mutation`, `cell_boundary_markpen_marks_are_rejected_before_mutation`, `metadata_without_boundary_objects_keeps_existing_deletion_paths`, `styled_inline_objects_keep_first_and_last_cell_paragraph_logical_offsets`, `styled_inline_objects_keep_body_run_logical_offsets`, `styled_inline_objects_keep_logical_offsets_across_line_breaks`, `styled_inline_objects_keep_prior_slots_on_continuation_pages`, `cell_cut_paste_restores_text_offsets_after_inline_and_floating_objects`, `cell_single_control_paste_keeps_same_anchor_objects_before_the_scalar_offset`, `cell_text_paste_keeps_scalar_fast_path_and_multiline_split_after_inline_objects`, `flat_table_caption_text_delete_keeps_legacy_address`.
- `tests/cases/issue_4961_font_decision_trace.rs`: SHA-256 `289815610b16ab6b0e99bd71f158f5383de79772c20111587581e91d378fa17f`. 직접 선언 검사 12개: `font_trace_profiles_follow_source_ownership_and_preserve_font_decisions`, `stage4_public_trace_limit_and_unsupported_backends_fail_closed`, `public_fixture_trace_is_bounded_and_deterministic`, `issue4967_combined_evidence_uses_one_tree_and_preserves_trace`, `issue4967_combined_evidence_reports_actual_cache_key_rejection`, `standalone_native_trace_requires_a_prepared_renderer_snapshot`, `prepared_native_renderer_snapshot_preserves_custom_font_inventory`, `trace_options_fail_closed`, `logical_tac_runs_join_exact_scalar_source_in_body_and_nested_cells`, `synthetic_overlap_cannot_claim_repeated_source_and_real_pua_keeps_source`, `note_markers_are_transparent_to_body_and_nested_source_glyph_addresses`, `coanchored_logical_copy_preserves_endpoint_inclusion_and_source_in_both_formats`.
- `tests/issue_1452_saved_caret.rs`: SHA-256 `4a66d919b6da3938fd4654b1ed48df97acac6bb1fc1f480dab1e2a90a805c148`. 직접 선언 검사 9개: `transparency_sample_restores_saved_caret_after_second_inline_picture`, `enter_before_second_tac_picture_splits_inline_controls`, `repeated_enter_before_second_tac_picture_keeps_picture_after_blank_lines`, `repeated_enter_after_second_tac_picture_appends_blank_lines`, `arrow_up_from_second_tac_picture_end_moves_to_first_picture_end`, `clicking_after_second_tac_picture_hits_picture_end_position`, `short_inline_picture_hits_follow_styled_empty_carets_in_both_formats`, `arrow_up_after_enter_before_first_tac_picture_moves_to_previous_picture_end`, `transparency_test_sample_text_line_boundaries_keep_visual_affinity`.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

원 PR의 증적이나 helper assertion만으로 독립 Print 일치를 선언하지 않는다. 적용 입력·최종 head의 결과를 개별적으로 대조한 뒤 최종 판정을 갱신한다.

전체 로컬 Rust 검증은 완료했다. 현재 함수별 실제 PASS와 종료 근거는 아래 「통합 후보의 실제 검사 실행」에 기록했다. 독립 시각 근거와 원격 CI는 별도로 판정한다.

통합 후보의 현재 공통 시각 비교에서는 17조건·213쪽/backend의 후보 최저가 Native 92.04708%, fresh WASM 91.53306%였다. [전쪽 TSV](../assets/semanticist21_rebase_qualified_20261026/README.md)의 수치 대응 가능한 16조건·211쪽/backend에서 0.1%p 초과 하락 0쪽을 확인했고, #6133은 base 1쪽에서 독립 Print와 같은 2쪽으로 복원된 구조 개선으로 따로 기록했다. 이는 이 PR의 별도 입력이나 아래 필수 증거가 자동으로 검증됐다는 뜻이 아니다. #8228의 변경 기능 보존과 현재 보정 대상만 확인하며 다른 HWP로 범위를 넓히지 않는다.

| 조판 검토 항목 | 현재 근거와 판정 경계 |
| --- | --- |
| 규칙·공통 소비 | 위 실제 소비 경로와 반례를 따른다. helper 뒤 덮어쓰기·다른 frame을 실제 결과와 대조한다. |
| 입력·독립 기대값 | 입력 생성 방식과 한컴 Print 출처를 증거별로 구분한다. source와 무관한 값이나 원 작성자의 주장만으로 통과 처리하지 않는다. |
| 줄·문단·개체 소유 | 현재 검사 소스의 순서·누락·중복·포함·겹침 관계를 확인한다. |
| 시각·px 자격 | 전쪽 양쪽 raw 90% 이상·쪽수 일치·직접 판독이 필요하다. ±2px는 양쪽 98% 이상을 확인한 해당 페이지의 독립 좌표에만 허용한다. |
| 리베이스 기능 보존 | 새 base와 현재 후보의 동일 입력·producer·Print 조건을 대조한다. 평균값으로 페이지별 0.1%p 초과 하락을 상쇄하지 않는다. |
| 완료된 로컬 게이트 | 아래 실제 실행 결과와 공통 종료 기록을 대조했다. 이 PR의 독립 시각 증거와 원격 CI는 별도로 판정한다. |

sample16·편람의 기존 삭제를 유지하며 삭제 파일을 향하던 파생 참조도 재생성하지 않는다. 미해결 이관 문서를 완료 범위나 이슈 닫기 목록에 넣지 않는다.

## 이전 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/cell_boundary_object_cut.rs](../../../tests/cases/cell_boundary_object_cut.rs): SHA-256 `409364a7b7993261f42e3c89eee92de12e5e886188decd8545746bf2e406fde7`.
  - `cell_boundary_markpen_marks_are_rejected_before_mutation`: PASS, `logs/current-full-nextest-4184.log:7733` (`rhwp::regression_suite_014`).
  - `cell_boundary_range_tags_are_rejected_before_mutation`: PASS, `logs/current-full-nextest-4184.log:7742` (`rhwp::regression_suite_014`).
  - `cell_boundary_with_field_is_rejected_before_mutation`: PASS, `logs/current-full-nextest-4184.log:7734` (`rhwp::regression_suite_014`).
  - `cell_cut_paste_restores_text_offsets_after_inline_and_floating_objects`: PASS, `logs/current-full-nextest-4184.log:7750` (`rhwp::regression_suite_014`).
  - `cell_single_control_paste_keeps_same_anchor_objects_before_the_scalar_offset`: PASS, `logs/current-full-nextest-4184.log:7736` (`rhwp::regression_suite_014`).
  - `cell_text_paste_keeps_scalar_fast_path_and_multiline_split_after_inline_objects`: PASS, `logs/current-full-nextest-4184.log:7737` (`rhwp::regression_suite_014`).
  - `empty_cell_ranges_keep_tail_objects_raw_stream_and_events`: PASS, `logs/current-full-nextest-4184.log:7738` (`rhwp::regression_suite_014`).
  - `flat_cell_cut_removes_boundary_pictures_and_tables`: PASS, `logs/current-full-nextest-4184.log:7739` (`rhwp::regression_suite_014`).
  - `flat_table_caption_text_delete_keeps_legacy_address`: PASS, `logs/current-full-nextest-4184.log:7740` (`rhwp::regression_suite_014`).
  - `invalid_cell_ranges_keep_document_and_clipboard`: PASS, `logs/current-full-nextest-4184.log:7745` (`rhwp::regression_suite_014`).
  - `metadata_without_boundary_objects_keeps_existing_deletion_paths`: PASS, `logs/current-full-nextest-4184.log:7743` (`rhwp::regression_suite_014`).
  - `path_cell_cut_removes_boundary_pictures_and_tables_at_each_depth`: PASS, `logs/current-full-nextest-4184.log:7748` (`rhwp::regression_suite_014`).
  - `same_paragraph_cell_cut_removes_only_selected_object`: PASS, `logs/current-full-nextest-4184.log:7746` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_body_run_logical_offsets`: PASS, `logs/current-full-nextest-4184.log:7747` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_first_and_last_cell_paragraph_logical_offsets`: PASS, `logs/current-full-nextest-4184.log:7754` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_logical_offsets_across_line_breaks`: PASS, `logs/current-full-nextest-4184.log:7749` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_prior_slots_on_continuation_pages`: PASS, `logs/current-full-nextest-4184.log:7751` (`rhwp::regression_suite_014`).
- [tests/cases/issue_4961_font_decision_trace.rs](../../../tests/cases/issue_4961_font_decision_trace.rs): SHA-256 `289815610b16ab6b0e99bd71f158f5383de79772c20111587581e91d378fa17f`.
  - `coanchored_logical_copy_preserves_endpoint_inclusion_and_source_in_both_formats`: PASS, `logs/current-native-trace-4191.log:14` (`rhwp::regression_suite_009`).
  - `font_trace_profiles_follow_source_ownership_and_preserve_font_decisions`: PASS, `logs/current-native-trace-4191.log:19` (`rhwp::regression_suite_009`).
  - `issue4967_combined_evidence_reports_actual_cache_key_rejection`: PASS, `logs/current-native-trace-4191.log:17` (`rhwp::regression_suite_009`).
  - `issue4967_combined_evidence_uses_one_tree_and_preserves_trace`: PASS, `logs/current-native-trace-4191.log:15` (`rhwp::regression_suite_009`).
  - `logical_tac_runs_join_exact_scalar_source_in_body_and_nested_cells`: PASS, `logs/current-native-trace-4191.log:12` (`rhwp::regression_suite_009`).
  - `note_markers_are_transparent_to_body_and_nested_source_glyph_addresses`: PASS, `logs/current-native-trace-4191.log:16` (`rhwp::regression_suite_009`).
  - `prepared_native_renderer_snapshot_preserves_custom_font_inventory`: PASS, `logs/current-native-trace-4191.log:18` (`rhwp::regression_suite_009`).
  - `public_fixture_trace_is_bounded_and_deterministic`: PASS, `logs/current-native-trace-4191.log:13` (`rhwp::regression_suite_009`).
  - `stage4_public_trace_limit_and_unsupported_backends_fail_closed`: PASS, `logs/current-native-trace-4191.log:11` (`rhwp::regression_suite_009`).
  - `standalone_native_trace_requires_a_prepared_renderer_snapshot`: PASS, `logs/current-native-trace-4191.log:10` (`rhwp::regression_suite_009`).
  - `synthetic_overlap_cannot_claim_repeated_source_and_real_pua_keeps_source`: PASS, `logs/current-native-trace-4191.log:8` (`rhwp::regression_suite_009`).
  - `trace_options_fail_closed`: PASS, `logs/current-native-trace-4191.log:9` (`rhwp::regression_suite_009`).
- [tests/issue_1452_saved_caret.rs](../../../tests/issue_1452_saved_caret.rs): SHA-256 `4a66d919b6da3938fd4654b1ed48df97acac6bb1fc1f480dab1e2a90a805c148`.
  - `arrow_up_after_enter_before_first_tac_picture_moves_to_previous_picture_end`: PASS, `logs/current-full-nextest-4184.log:10460` (`rhwp::regression_suite_024`).
  - `arrow_up_from_second_tac_picture_end_moves_to_first_picture_end`: PASS, `logs/current-full-nextest-4184.log:10461` (`rhwp::regression_suite_024`).
  - `clicking_after_second_tac_picture_hits_picture_end_position`: PASS, `logs/current-full-nextest-4184.log:10462` (`rhwp::regression_suite_024`).
  - `enter_before_second_tac_picture_splits_inline_controls`: PASS, `logs/current-full-nextest-4184.log:10463` (`rhwp::regression_suite_024`).
  - `repeated_enter_after_second_tac_picture_appends_blank_lines`: PASS, `logs/current-full-nextest-4184.log:10464` (`rhwp::regression_suite_024`).
  - `repeated_enter_before_second_tac_picture_keeps_picture_after_blank_lines`: PASS, `logs/current-full-nextest-4184.log:10465` (`rhwp::regression_suite_024`).
  - `short_inline_picture_hits_follow_styled_empty_carets_in_both_formats`: PASS, `logs/current-full-nextest-4184.log:10466` (`rhwp::regression_suite_024`).
  - `transparency_sample_restores_saved_caret_after_second_inline_picture`: PASS, `logs/current-full-nextest-4184.log:10467` (`rhwp::regression_suite_024`).
  - `transparency_test_sample_text_line_boundaries_keep_visual_affinity`: PASS, `logs/current-full-nextest-4184.log:10468` (`rhwp::regression_suite_024`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_7833_review_impl.md)를 따른다.

## NO_LS·인라인 프레임 보정의 제한된 재검증 — 2026-10-26

이 단계의 제품 후보는 시작 head `35fa0d12960059d2a982347717bdeb408962cc01`에
추가한 공통 줄 프레임 변경이다. 실제 Native264 빌드 source별 SHA는 ignored
`reflow-visible-host-native-build-4340-result.json`에 고정했다. 바이트별 전체 source가
이전 후보와 같다는 뜻은 아니다. 새 가시 NO_LS 호스트는 실제 셀 폭에서 구성한
줄 점유를 측정·전체/부분 배치가 공유하며, 단일 토큰 fallback은 기측정 폭을 쓴다.
인라인 그림·수식도 새 원시 줄 경계를 발행해 가시 글자 경계로 소속을 추측하지 않는다.

같은 wrapper 8조건·기존 주석 대조군 8조건의 Native 비교는 12통과/4미달이다.
중첩 그림은 94.13019%/97.26027%, flat 그림과 주석 10조건은 100%다. 인라인 표
4조건의 미달은 유지하며 통합 승인으로 판정하지 않는다. 기존 셀 경계 편집 17건,
#8188 상속 88건, 쪽 번호 4건은 실제 자동 수집에서 **109 PASS/0 FAIL**이었다.
로그는 `output/pr-review/semanticist21-20261019/logs/reflow-inherited-contracts-nextest-4339.log`다.

작은 TAC 표는 한컴 진단 저장본의 공통 폭이 1200→2606 HU로 커져도 셀 선언 600 HU는
유지됐다. 현재 닫힌 그리드의 합의만으로 실제 물리 폭을 확정하는 가정을 계속 보정해야 한다.
기준은 기존 독립 Print이며 진단 저장본이나 폭 변형을 원본의 통과로 대체하지 않는다.
fresh WASM·전체 시각 검증·전체 Rust/lint와 최종 원격 CI는 아직 완료하지 않았다.
