# PR #7833 구현·검증 기록

## 범위와 고정 기준

[개별 리뷰](pr_7833_review.md)의 원 head `255413cdbc74d34f7c1d1be09fb7ad640eac6feb`와 실제 소비 경로를 따른다. `review/semanticist21-20261019`에서 고정 devel `238de6de41a208648f4ade6b494fe433a81c1fce` 위로 누적한 45개 원 PR 중 하나다. 추가 PR을 체리픽하거나 다른 HWP로 범위를 확대하지 않는다.

## 원 커밋과 통합 대응

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `cbb2d3ee9b702027a8101526fbd2b5f9152603c8` | `94d0267d8534ca1b0ab64f8ea39b27b16244b3ee` |
| `3ae9e3b71ea08fdd1ae06a02aab33b7c31b22ad5` | `5d80892479114bd613c13ece7044c9889c3dad04` |
| `a0c72f3cb85347958194a88d0fec4a81f28155ba` | `d5a461f2073987e11bf86e2f8e2184e1f40fcfde` |
| `efec7825d325beb5f2eff65ac921f148818367a5` | `8abe0091e1b15b36f434b32614aa0b8433b306d6` |
| `c03d1f98ed5b19c091c7028e370089dc6a3dfbc3` | `5421224c2fae1a357752f90c6265ab64e29c4a86` |
| `79b7dd66f20f89787521175762879f73df478674` | `98b029df5daed2fb7007aa6cbaf99ca816b25b12` |
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


## 최소 물리 셀 폭 보정의 Native 재검증 — 2026-10-26

이전 Native264의 TAC 표 4조건 미달은 선언 폭과 물리 폭을 같은 값으로 본 가정에서
발생했다. 한컴 2020의 동일 입력 Print·원시 저장 대조에서 셀은 유효 좌우 안 여백과
내용 폭 하한 283 HU를 확보했다. 글자 크기·바깥 여백·TAC·noAdjust·병합 셀을
대조했고, 이 숫자를 사양에 명시된 값이라고 주장하지 않는다. raw 셀 폭·직렬화는
보존하고 공통 physical_cell_width_hu를 측정·열/칸 제약·최종 경계에 전달했다.

Native265의 같은 16조건은 16 PASS/0 FAIL이다. TAC 표 4조건은
70.06839/77.91698/81.99184/87.13986%에서 모두 100%로 개선됐다. 깊은 그림
94.13019/97.26027%와 나머지 10조건의 100%는 유지했다. 깊은 표 before의 새
review·standalone overlay를 실제 열어 칸 경계·내용 순서·부모/자손 점유를 확인했다.
증거는 ignored `output/pr-review/semanticist21-20261019/`의
`reflow-visible-host-validation-4354-result.json`, `minimum-cell-width-direct-4354/`다.

상속 #8188의 기존 4입력 256쪽도 새 Native에서 raw100·정수 미대응0/0이며 이전
Native264 PNG와 전쪽 SHA가 같다(4355/4358). fresh WASM078과 보정 후 Rust/lint는
아직 완료되지 않았다. 별도 부모 기하 대조군의 개선을 원래 생성기 전체의 해결로
바꾸어 보고하지 않으며, 원본·기준 PDF·기존 검사 기대값도 유지한다. 통합 최종
판정은 남은 필수 검증과 미달 해결 뒤에 갱신한다.


## fresh 재검증과 최소 폭 회귀 추가 — 2026-10-27

같은 16개 대조군은 Native265와 표준 최적화 fresh078에서 모두 90% 이상이며
TAC 표4조건은 양쪽 100%다. 한 쪽의 양쪽 review/standalone overlay를 직접 확인한 뒤
[새 이름의 회귀·영구 입력/PDF·TSV·PNG](../assets/pr_7833_minimum_cell_width/README.md)를
추가했다. 독립 가로 경계 ±2px와 내용·칸 여백·연결·순서·비겹침 관계를 함께 검사한다.
원 helper나 기존17함수는 교체하지 않았다. 같은 새 검사 소스로 Native264의 실제 16px
폭을1 FAIL·exit101로 검출했고 Native265에서1 PASS·exit0이었다. 부모 셀 포함과 문단/개체 소속을 보강한 최종 검사도 전후 재실행했고, 자동 수집
suite023에서 새 함수 1 PASS·exit0을 확인했다(4371/4372). 기존109 PASS와 이 새 한 함수는 서로 다른 실행이다.

#8188 상속4입력의 Native/fresh 총512쪽은 새 코드에서도 raw100·정수 미대응0/0이고
이전 검증 PNG와 전쪽 SHA가 같다. 상속/셀 경계/쪽 번호109건도 새 코드에서 PASS였다.
원 생성기의 잘못된 저장 프레임을 둔 8입력은 4359에서 미달을 재확인했고 독립 한컴의
폭42520→19956·첫 vpos3200→0 갱신을4360에서 추적했다. 원본 미달을 이 대조군으로
대체하지 않으며 최종 판정은 전체 필수 항목을 해결한 뒤 갱신한다.
