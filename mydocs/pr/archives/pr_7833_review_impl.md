# PR #7833 구현·검증 기록

## 범위와 고정 기준

[개별 리뷰](pr_7833_review.md)의 원 head `255413cdbc74d34f7c1d1be09fb7ad640eac6feb`와 실제 소비 경로를 따른다. `review/semanticist21-20261019`에서 고정 devel `238de6de41a208648f4ade6b494fe433a81c1fce` 위로 누적한 45개 원 PR 중 하나다. 추가 PR을 체리픽하거나 다른 HWP로 범위를 확대하지 않는다.

## 원 커밋과 통합 대응

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `cbb2d3ee9b702027a8101526fbd2b5f9152603c8` | `cb6e32d92b76e884cd64d6413448f27f557bc324` |
| `3ae9e3b71ea08fdd1ae06a02aab33b7c31b22ad5` | `54203bb5d828401122aeddad3ec62e21a89fc524` |
| `a0c72f3cb85347958194a88d0fec4a81f28155ba` | `6e5d2ad721d16e4ce3c3904a629e625b0b95a052` |
| `efec7825d325beb5f2eff65ac921f148818367a5` | `aada9428fc1c4dddb92f96280291ba27e7b4c16f` |
| `c03d1f98ed5b19c091c7028e370089dc6a3dfbc3` | `a825b7a531e9eb576f7e2fcdd86e58773aee1b31` |
| `79b7dd66f20f89787521175762879f73df478674` | `40069c0bda36d52e0bbf43b7c0f2390cbc144fe1` |
| `255413cdbc74d34f7c1d1be09fb7ad640eac6feb` |  |

충돌 보정과 원 기능의 유지 여부는 원 커밋·현재 코드 후보·독립 기대값을 대조한다. 메인터너의 공통 renderer·편집 소유 보정과 원 contributor 변경을 혼동하지 않는다. 원 head는 직접 merge 대상이 아니다.

## 실행 순서와 현재 상태

1. 원 PR의 reviewer·head를 확인하고 기능 commit을 최신 devel 위에 누적 적용했다. 원격 원 head를 재작성하지 않았다.
2. 코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 로컬 Rust 검증을 완료했다. 렌더 source는 `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`이며 이후 증거 등록은 런타임을 바꾸지 않았다. 미완료 항목을 완료로 표시하지 않는다.
3. 개별 리뷰의 독립 Print·Native/fresh WASM·소비 경로와 전체 로컬 게이트를 완료한 뒤 최종 판정을 갱신한다. PR 직전 devel head를 재조회하고 변경됐으면 리베이스한 기능과 현재 보정 범위를 함께 검증한다.
4. 승인된 통합 PR을 실제 템플릿에 맞춰 제출하고 정확한 code head CI를 관찰한다. 별도로 승인된 옵션 2의 self-review·오늘할일 후속 PR은 code CI 뒤에 처리한다. 원 PR별 검토 기록은 통합 PR에 유지한다.
5. CI·merge 조건을 확인해 통합하고 merge SHA의 devel 포함을 확인한다. 이 PR에는 보정·검증 결과와 통합 링크를 남긴 뒤 원 PR을 close한다. 관련 이슈는 전체 해결 범위가 확인된 경우만 닫는다.
6. contributor fork·branch를 보존하고 소유·실행 상태를 확인한 검토 worktree·임시 산출물만 정리한다. 공유 `target/pr-review`는 유지한다.

## 제외와 복원 경계

sample16·편람의 삭제된 회귀·파생 참조는 복구하지 않는다. 별도 이관 검사를 이 통합의 통과 증거로 다시 등록하지 않는다. 통합 전 코드 변경이 필요하면 일반 추가 commit으로 보정하고 영향 검증을 재실행한다. contributor history를 reset·amend·force-push하지 않는다.

## 완료 전 남은 기록

정확한 전체 검증 terminal 결과, 적용 입력의 시각 증거·직접 판독과 최종 판정, 실제 통합 PR·CI·merge SHA·후속 comment URL은 수행 후 기록한다. 이 계획 자체를 해당 단계의 완료로 간주하지 않는다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/cell_boundary_object_cut.rs](../../../tests/cases/cell_boundary_object_cut.rs): SHA-256 `409364a7b7993261f42e3c89eee92de12e5e886188decd8545746bf2e406fde7`.
  - `flat_cell_cut_removes_boundary_pictures_and_tables`: PASS, `logs/final-full-nextest-4052.log:7750` (`rhwp::regression_suite_014`).
  - `path_cell_cut_removes_boundary_pictures_and_tables_at_each_depth`: PASS, `logs/final-full-nextest-4052.log:7756` (`rhwp::regression_suite_014`).
  - `same_paragraph_cell_cut_removes_only_selected_object`: PASS, `logs/final-full-nextest-4052.log:7753` (`rhwp::regression_suite_014`).
  - `invalid_cell_ranges_keep_document_and_clipboard`: PASS, `logs/final-full-nextest-4052.log:7752` (`rhwp::regression_suite_014`).
  - `cell_boundary_with_field_is_rejected_before_mutation`: PASS, `logs/final-full-nextest-4052.log:7741` (`rhwp::regression_suite_014`).
  - `empty_cell_ranges_keep_tail_objects_raw_stream_and_events`: PASS, `logs/final-full-nextest-4052.log:7747` (`rhwp::regression_suite_014`).
  - `cell_boundary_range_tags_are_rejected_before_mutation`: PASS, `logs/final-full-nextest-4052.log:7742` (`rhwp::regression_suite_014`).
  - `cell_boundary_markpen_marks_are_rejected_before_mutation`: PASS, `logs/final-full-nextest-4052.log:7740` (`rhwp::regression_suite_014`).
  - `metadata_without_boundary_objects_keeps_existing_deletion_paths`: PASS, `logs/final-full-nextest-4052.log:7751` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_first_and_last_cell_paragraph_logical_offsets`: PASS, `logs/final-full-nextest-4052.log:7761` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_body_run_logical_offsets`: PASS, `logs/final-full-nextest-4052.log:7755` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_logical_offsets_across_line_breaks`: PASS, `logs/final-full-nextest-4052.log:7757` (`rhwp::regression_suite_014`).
  - `styled_inline_objects_keep_prior_slots_on_continuation_pages`: PASS, `logs/final-full-nextest-4052.log:7758` (`rhwp::regression_suite_014`).
  - `cell_cut_paste_restores_text_offsets_after_inline_and_floating_objects`: PASS, `logs/final-full-nextest-4052.log:7748` (`rhwp::regression_suite_014`).
  - `cell_single_control_paste_keeps_same_anchor_objects_before_the_scalar_offset`: PASS, `logs/final-full-nextest-4052.log:7743` (`rhwp::regression_suite_014`).
  - `cell_text_paste_keeps_scalar_fast_path_and_multiline_split_after_inline_objects`: PASS, `logs/final-full-nextest-4052.log:7746` (`rhwp::regression_suite_014`).
  - `flat_table_caption_text_delete_keeps_legacy_address`: PASS, `logs/final-full-nextest-4052.log:7749` (`rhwp::regression_suite_014`).
- [tests/cases/issue_4961_font_decision_trace.rs](../../../tests/cases/issue_4961_font_decision_trace.rs): SHA-256 `289815610b16ab6b0e99bd71f158f5383de79772c20111587581e91d378fa17f`.
  - `font_trace_profiles_follow_source_ownership_and_preserve_font_decisions`: PASS, `logs/final-full-nextest-4052.log:6459` (`rhwp::regression_suite_009`).
  - `stage4_public_trace_limit_and_unsupported_backends_fail_closed`: PASS, `logs/final-full-nextest-4052.log:6412` (`rhwp::regression_suite_009`).
  - `public_fixture_trace_is_bounded_and_deterministic`: PASS, `logs/final-full-nextest-4052.log:6413` (`rhwp::regression_suite_009`).
  - `issue4967_combined_evidence_uses_one_tree_and_preserves_trace`: PASS, `logs/final-full-nextest-4052.log:6408` (`rhwp::regression_suite_009`).
  - `issue4967_combined_evidence_reports_actual_cache_key_rejection`: PASS, `logs/final-full-nextest-4052.log:6407` (`rhwp::regression_suite_009`).
  - `standalone_native_trace_requires_a_prepared_renderer_snapshot`: PASS, `logs/native-trace-contracts-4119.log:10` (`rhwp::regression_suite_009`).
  - `prepared_native_renderer_snapshot_preserves_custom_font_inventory`: PASS, `logs/native-trace-contracts-4119.log:18` (`rhwp::regression_suite_009`).
  - `trace_options_fail_closed`: PASS, `logs/final-full-nextest-4052.log:6415` (`rhwp::regression_suite_009`).
  - `logical_tac_runs_join_exact_scalar_source_in_body_and_nested_cells`: PASS, `logs/final-full-nextest-4052.log:6409` (`rhwp::regression_suite_009`).
  - `synthetic_overlap_cannot_claim_repeated_source_and_real_pua_keeps_source`: PASS, `logs/final-full-nextest-4052.log:6414` (`rhwp::regression_suite_009`).
  - `note_markers_are_transparent_to_body_and_nested_source_glyph_addresses`: PASS, `logs/final-full-nextest-4052.log:6411` (`rhwp::regression_suite_009`).
  - `coanchored_logical_copy_preserves_endpoint_inclusion_and_source_in_both_formats`: PASS, `logs/final-full-nextest-4052.log:6405` (`rhwp::regression_suite_009`).
- [tests/issue_1452_saved_caret.rs](../../../tests/issue_1452_saved_caret.rs): SHA-256 `4a66d919b6da3938fd4654b1ed48df97acac6bb1fc1f480dab1e2a90a805c148`.
  - `transparency_sample_restores_saved_caret_after_second_inline_picture`: PASS, `logs/final-full-nextest-4052.log:10410` (`rhwp::regression_suite_024`).
  - `enter_before_second_tac_picture_splits_inline_controls`: PASS, `logs/final-full-nextest-4052.log:10406` (`rhwp::regression_suite_024`).
  - `repeated_enter_before_second_tac_picture_keeps_picture_after_blank_lines`: PASS, `logs/final-full-nextest-4052.log:10408` (`rhwp::regression_suite_024`).
  - `repeated_enter_after_second_tac_picture_appends_blank_lines`: PASS, `logs/final-full-nextest-4052.log:10407` (`rhwp::regression_suite_024`).
  - `arrow_up_from_second_tac_picture_end_moves_to_first_picture_end`: PASS, `logs/final-full-nextest-4052.log:10404` (`rhwp::regression_suite_024`).
  - `clicking_after_second_tac_picture_hits_picture_end_position`: PASS, `logs/final-full-nextest-4052.log:10405` (`rhwp::regression_suite_024`).
  - `short_inline_picture_hits_follow_styled_empty_carets_in_both_formats`: PASS, `logs/final-full-nextest-4052.log:10409` (`rhwp::regression_suite_024`).
  - `arrow_up_after_enter_before_first_tac_picture_moves_to_previous_picture_end`: PASS, `logs/final-full-nextest-4052.log:10403` (`rhwp::regression_suite_024`).
  - `transparency_test_sample_text_line_boundaries_keep_visual_affinity`: PASS, `logs/final-full-nextest-4052.log:10411` (`rhwp::regression_suite_024`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `cbb2d3ee9b702027a8101526fbd2b5f9152603c8` | `cb6e32d92b76e884cd64d6413448f27f557bc324` |
| `3ae9e3b71ea08fdd1ae06a02aab33b7c31b22ad5` | `54203bb5d828401122aeddad3ec62e21a89fc524` |
| `a0c72f3cb85347958194a88d0fec4a81f28155ba` | `6e5d2ad721d16e4ce3c3904a629e625b0b95a052` |
| `efec7825d325beb5f2eff65ac921f148818367a5` | `aada9428fc1c4dddb92f96280291ba27e7b4c16f` |
| `c03d1f98ed5b19c091c7028e370089dc6a3dfbc3` | `a825b7a531e9eb576f7e2fcdd86e58773aee1b31` |
| `79b7dd66f20f89787521175762879f73df478674` | `40069c0bda36d52e0bbf43b7c0f2390cbc144fe1` |
| `255413cdbc74d34f7c1d1be09fb7ad640eac6feb` |  |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
