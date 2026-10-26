# PR #8065 구현·검증 기록

## 범위와 고정 기준

[개별 리뷰](pr_8065_review.md)의 원 head `72c825408e39d2084a85b746440fd29469f2805c`와 실제 소비 경로를 따른다. `review/semanticist21-20261019`에서 고정 devel `238de6de41a208648f4ade6b494fe433a81c1fce` 위로 누적한 45개 원 PR 중 하나다. 추가 PR을 체리픽하거나 다른 HWP로 범위를 확대하지 않는다.

## 원 커밋과 통합 대응

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `72c825408e39d2084a85b746440fd29469f2805c` | `b5aff38d64880ab4ed44834e1e78fc75cf60c71d` |

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

- [tests/cases/cell_padding_exceeding_declared_height_grows_row.rs](../../../tests/cases/cell_padding_exceeding_declared_height_grows_row.rs): SHA-256 `da4166c7c9fd1d71c0522d9dcd442f0ae1561db5a3188fed423b2a385d5f785a`.
  - `cell_padding_exceeding_declared_height_grows_row`: PASS, `logs/final-full-nextest-4052.log:4307` (`rhwp::regression_suite_002`).
- [tests/cases/issue_5751_dense_table_row_growth.rs](../../../tests/cases/issue_5751_dense_table_row_growth.rs): SHA-256 `0dd2c71d4c4e0fe990c0c08ac999f7c1d973a52758bedfd9b8ccc6ad136a63cc`.
  - `issue_5751_dense_row_grows_to_content_plus_padding`: PASS, `logs/final-full-nextest-4052.log:8385` (`rhwp::regression_suite_016`).
- [tests/cases/issue_6442_unused_cell_padding_is_garbage.rs](../../../tests/cases/issue_6442_unused_cell_padding_is_garbage.rs): SHA-256 `6facaefae946cb165e5e61af269aff049adcd4086b5db88c4a6fdaad35f9d620`.
  - `both_back_side_cards_on_page2_carry_their_content`: PASS, `logs/final-full-nextest-4052.log:6135` (`rhwp::regression_suite_008`).
  - `page3_control_group_is_unchanged`: PASS, `logs/final-full-nextest-4052.log:6137` (`rhwp::regression_suite_008`).
  - `unused_inner_margin_field_is_not_charged`: PASS, `logs/final-full-nextest-4052.log:6136` (`rhwp::regression_suite_008`).
- [tests/cases/issue_7234_scaffold_table_cell_height.rs](../../../tests/cases/issue_7234_scaffold_table_cell_height.rs): SHA-256 `2aa9d7f8251b1cc445829cd902fe28a80c843ca8d5cc105565a323830dcadfb8`.
  - `issue_7234_scaffold_cell_height_is_one_line_row_height`: PASS, `logs/final-full-nextest-4052.log:7615` (`rhwp::regression_suite_013`).
  - `issue_7234_scaffold_tall_table_first_fragment_stays_inside_body`: PASS, `logs/final-full-nextest-4052.log:7616` (`rhwp::regression_suite_013`).
  - `issue_7234_existing_minimum_height_rows_fit_the_painted_fragment`: PASS, `logs/final-full-nextest-4052.log:7614` (`rhwp::regression_suite_013`).
  - `issue_7234_content_cut_fit_must_not_accept_an_overfull_whole_row`: PASS, `logs/final-full-nextest-4052.log:7613` (`rhwp::regression_suite_013`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `72c825408e39d2084a85b746440fd29469f2805c` | `b5aff38d64880ab4ed44834e1e78fc75cf60c71d` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
