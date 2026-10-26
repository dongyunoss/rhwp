# PR #7694 리뷰 — 기능: 인라인 개체를 논리 캐럿 위치로 원자적으로 이동하는 API 추가

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#7694](https://github.com/edwardkim/rhwp/pull/7694) / semanticist21 |
| 고정 원 head | `0e7383d03753ac2546b0be74f19a244fe81747ff` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=true |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `f7c0acb2c9580cae3ebc14e09bb503dba261ffaa` | `cca5817c81a6679b2fadc957997dc2ef95286450` |
| `3e2a9dc1c6597e357634835208e0efcfca5288c0` | `f7be47cbf4abdfb0f849faf3d980df4c673d80cb` |
| `9dab2711be1156aefc5318a1adb4e98fb0a309fe` | `e854311d59c6faf5824dfe4d3df8fc24b0d9d7ea` |
| `30a29eaf5481b6ca8b1c283c10b695a735eadc35` | `f21c4a7da8481e65ca324449b3c2b9051919a4ed` |
| `5f7a4fca71f816c2fb72154639c31e3826423a28` | `44903a1025a656b46beb7b579f26e9385ebe70e6` |
| `3e68f620a9c2643bb52a78d305b2f7e95f06a631` | `19969825ec2779f74b8b6673d258eac66e0d5742` |
| `55f06f5505d6f6ddd449a3f7b1cb4acd84c29ac2` | `cc29ccf21e930832d06756e4210ce6efe3419866` |
| `09f91ff16986b6f64db4a3b46d1a63ca0f6e8de5` | `ce062af329914df8564bed3f525eb01a0e9c6585` |
| `9052f0bde427f93bf3cefe5de09875065a49bfbb` | 원 devel merge; 기능 체리픽에서 제외 |
| `133e09e154d0d496850d64deb7e823b2599cceba` | `4fa3c6d58382f40dd586fccc5742a752c47156b6` |
| `680f44ae667ba16b8ecc1073ce815286788d138b` | `46a476abbbffad8f085cb701fca049aa6be458bb` |
| `0e7383d03753ac2546b0be74f19a244fe81747ff` | `f0e526e3e33dd6b29d221b719035fe94e4b6dbe1` |

### 실제 소비 경로와 반례

move_inline_control_native는 배포 문서·원본/대상 주소·잠금·logical caret를 먼저 검증하고 영향 구역 복제본에서 remove/insert를 실행한다. 실제 문서 commit 이전에 실패할 수 있는 검사를 끝내는 순서를 확인한다. 같은 위치 no-op은 이벤트를 발행하지 않으며, source 제거 후 sibling 경로와 목적지 offset을 다시 계산한다. inline_control_move의 19개 검사는 Unicode·필드·형광펜·범위 태그·BinData·실제 OOXML 차트 payload 및 공개 HWP/HWPX 저장을 포함한다. 시각 chart-before/after 미달을 뒤의 OLE/차트 공통 보정과 구분해 기록한다.



### 현재 소스와 검사

- [src/document_core/commands/header_footer_ops.rs](../../../src/document_core/commands/header_footer_ops.rs)
- [src/document_core/commands/text_editing.rs](../../../src/document_core/commands/text_editing.rs)
- [src/document_core/converters/hwpx_to_hwp.rs](../../../src/document_core/converters/hwpx_to_hwp.rs)
- [src/document_core/inline_control.rs](../../../src/document_core/inline_control.rs)
- [src/document_core/mod.rs](../../../src/document_core/mod.rs)
- [src/model/event.rs](../../../src/model/event.rs)
- [src/serializer/body_text.rs](../../../src/serializer/body_text.rs)
- [src/wasm_api.rs](../../../src/wasm_api.rs)
- [tests/cases/inline_control_move.rs](../../../tests/cases/inline_control_move.rs)

- `tests/cases/inline_control_move.rs`: SHA-256 `d76cd31037ae4ba897601230b0499e4b6d152491ea251a403a8404bcb0f777f8`. 직접 선언 검사 19개: `same_paragraph_forward_and_back_preserve_unicode_order_styles_data_and_undo`, `adjacent_objects_have_distinct_before_after_carets_and_no_op_emits_nothing`, `cross_paragraph_move_keeps_paragraphs_and_both_save_formats`, `moving_after_a_trailing_auto_number_preserves_its_placeholder_and_both_save_formats`, `zero_length_field_slot_is_removed_exactly_and_field_indices_and_marks_survive`, `target_sibling_path_reindexes_after_source_removal_and_own_descendant_is_rejected`, `protected_cells_distribution_invalid_control_and_invalid_caret_are_atomic`, `header_and_note_destinations_return_existing_owner_forms`, `picture_binary_identity_and_control_metadata_survive_cross_owner_move`, `moving_out_of_a_cell_updates_its_owner_after_insertion_before_the_parent`, `locked_object_and_locked_parent_reject_without_mutation`, `markpen_object_ranges_move_between_paragraphs_and_preserve_unpaired_metadata`, `nonempty_field_insertion_keeps_object_inside_after_text_is_deleted`, `nested_markpen_colors_keep_outer_inner_order_when_moved`, `table_shape_textbox_and_chart_payloads_are_moved_without_recreation`, `actual_ooxml_chart_keeps_control_and_binary_payloads_after_move_and_save`, `public_hwp_export_preserves_inline_chart_with_and_without_move`, `malformed_source_or_destination_coordinates_reject_without_changes`, `committed_public_fixtures_reopen_and_keep_objects_at_the_saved_carets`.

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

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/inline_control_move.rs](../../../tests/cases/inline_control_move.rs): SHA-256 `d76cd31037ae4ba897601230b0499e4b6d152491ea251a403a8404bcb0f777f8`.
  - `same_paragraph_forward_and_back_preserve_unicode_order_styles_data_and_undo`: PASS, `logs/final-full-nextest-4052.log:9364` (`rhwp::regression_suite_020`).
  - `adjacent_objects_have_distinct_before_after_carets_and_no_op_emits_nothing`: PASS, `logs/final-full-nextest-4052.log:9350` (`rhwp::regression_suite_020`).
  - `cross_paragraph_move_keeps_paragraphs_and_both_save_formats`: PASS, `logs/final-full-nextest-4052.log:9352` (`rhwp::regression_suite_020`).
  - `moving_after_a_trailing_auto_number_preserves_its_placeholder_and_both_save_formats`: PASS, `logs/final-full-nextest-4052.log:9359` (`rhwp::regression_suite_020`).
  - `zero_length_field_slot_is_removed_exactly_and_field_indices_and_marks_survive`: PASS, `logs/final-full-nextest-4052.log:9368` (`rhwp::regression_suite_020`).
  - `target_sibling_path_reindexes_after_source_removal_and_own_descendant_is_rejected`: PASS, `logs/final-full-nextest-4052.log:9366` (`rhwp::regression_suite_020`).
  - `protected_cells_distribution_invalid_control_and_invalid_caret_are_atomic`: PASS, `logs/final-full-nextest-4052.log:9363` (`rhwp::regression_suite_020`).
  - `header_and_note_destinations_return_existing_owner_forms`: PASS, `logs/final-full-nextest-4052.log:9353` (`rhwp::regression_suite_020`).
  - `picture_binary_identity_and_control_metadata_survive_cross_owner_move`: PASS, `logs/final-full-nextest-4052.log:9362` (`rhwp::regression_suite_020`).
  - `moving_out_of_a_cell_updates_its_owner_after_insertion_before_the_parent`: PASS, `logs/final-full-nextest-4052.log:9358` (`rhwp::regression_suite_020`).
  - `locked_object_and_locked_parent_reject_without_mutation`: PASS, `logs/final-full-nextest-4052.log:9354` (`rhwp::regression_suite_020`).
  - `markpen_object_ranges_move_between_paragraphs_and_preserve_unpaired_metadata`: PASS, `logs/final-full-nextest-4052.log:9357` (`rhwp::regression_suite_020`).
  - `nonempty_field_insertion_keeps_object_inside_after_text_is_deleted`: PASS, `logs/final-full-nextest-4052.log:9361` (`rhwp::regression_suite_020`).
  - `nested_markpen_colors_keep_outer_inner_order_when_moved`: PASS, `logs/final-full-nextest-4052.log:9360` (`rhwp::regression_suite_020`).
  - `table_shape_textbox_and_chart_payloads_are_moved_without_recreation`: PASS, `logs/final-full-nextest-4052.log:9365` (`rhwp::regression_suite_020`).
  - `actual_ooxml_chart_keeps_control_and_binary_payloads_after_move_and_save`: PASS, `logs/final-full-nextest-4052.log:9349` (`rhwp::regression_suite_020`).
  - `public_hwp_export_preserves_inline_chart_with_and_without_move`: PASS, `logs/final-full-nextest-4052.log:9367` (`rhwp::regression_suite_020`).
  - `malformed_source_or_destination_coordinates_reject_without_changes`: PASS, `logs/final-full-nextest-4052.log:9356` (`rhwp::regression_suite_020`).
  - `committed_public_fixtures_reopen_and_keep_objects_at_the_saved_carets`: PASS, `logs/final-full-nextest-4052.log:9351` (`rhwp::regression_suite_020`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `f7c0acb2c9580cae3ebc14e09bb503dba261ffaa` | `cca5817c81a6679b2fadc957997dc2ef95286450` |
| `3e2a9dc1c6597e357634835208e0efcfca5288c0` | `f7be47cbf4abdfb0f849faf3d980df4c673d80cb` |
| `9dab2711be1156aefc5318a1adb4e98fb0a309fe` | `e854311d59c6faf5824dfe4d3df8fc24b0d9d7ea` |
| `30a29eaf5481b6ca8b1c283c10b695a735eadc35` | `f21c4a7da8481e65ca324449b3c2b9051919a4ed` |
| `5f7a4fca71f816c2fb72154639c31e3826423a28` | `44903a1025a656b46beb7b579f26e9385ebe70e6` |
| `3e68f620a9c2643bb52a78d305b2f7e95f06a631` | `19969825ec2779f74b8b6673d258eac66e0d5742` |
| `55f06f5505d6f6ddd449a3f7b1cb4acd84c29ac2` | `cc29ccf21e930832d06756e4210ce6efe3419866` |
| `09f91ff16986b6f64db4a3b46d1a63ca0f6e8de5` | `ce062af329914df8564bed3f525eb01a0e9c6585` |
| `9052f0bde427f93bf3cefe5de09875065a49bfbb` | 원 devel merge; 기능 체리픽에서 제외 |
| `133e09e154d0d496850d64deb7e823b2599cceba` | `4fa3c6d58382f40dd586fccc5742a752c47156b6` |
| `680f44ae667ba16b8ecc1073ce815286788d138b` | `46a476abbbffad8f085cb701fca049aa6be458bb` |
| `0e7383d03753ac2546b0be74f19a244fe81747ff` | `f0e526e3e33dd6b29d221b719035fe94e4b6dbe1` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_7694_review_impl.md)를 따른다.
