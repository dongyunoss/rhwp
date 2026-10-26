# PR #8043 구현·검증 기록

## 범위와 고정 기준

[개별 리뷰](pr_8043_review.md)의 원 head `a0bb863a08edf4e1461fbbf18ac2baaeaaea4450`와 실제 소비 경로를 따른다. `review/semanticist21-20261019`에서 고정 devel `238de6de41a208648f4ade6b494fe433a81c1fce` 위로 누적한 45개 원 PR 중 하나다. 추가 PR을 체리픽하거나 다른 HWP로 범위를 확대하지 않는다.

## 원 커밋과 통합 대응

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `a0bb863a08edf4e1461fbbf18ac2baaeaaea4450` | `edceca08577a63fbe28c7a913dd77e76fb86edff` |

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

## 주소 메타데이터와 조판 범위의 대조 — 2026-10-26

원 commit `a0bb863a08edf4e1461fbbf18ac2baaeaaea4450`의 실제 diff와 현재
`model/hyperlink.rs`, `document_core/hyperlink.rs`, `queries/bookmark_query.rs`,
`wasm_api/hyperlink.rs`, Studio의 `HyperlinkInfo`를 다시 대조했다.

| 소비 단계 | 실제 계약과 검증 경계 |
| --- | --- |
| Command 생성·조회 | `bookmark_command`는 `?` 접두어와 `;0;0;0;` 꼬리를 만들며 `\\`, `:`, `;`를 escape한다. `command_bookmark`는 표시 문자열과 별개로 대상을 읽는다. |
| 대상 존재 확인 | `has_bookmark_named`는 점 책갈피와 블록 책갈피 필드를 모두 조회한다. 없는 대상은 문단 복제본을 commit하기 전에 거절한다. |
| 삽입의 공통 경로 | 웹·책갈피가 같은 `insert_hyperlink_command`를 호출한다. 같은 Hyperlink 필드 종류·FieldRange·원문 축 이동·기존 서식 보관을 소비하며 주소 문자열만 달라진다. 새 target 종류에 따라 글줄 폭·원점·글꼴·표 배치를 바꾸는 분기는 추가하지 않았다. |
| 주소 수정 | `update_hyperlink_command`는 같은 종류·같은 주소의 no-op과 잘못된 종류의 거절을 구분한다. 표시 문자열·FieldRange를 바꾸는 `replace_hyperlink_text_native`와 주소 수정은 서로 다른 연산이다. |
| 공개·저장 경로 | WASM API는 같은 Native 함수를 호출하고 JSON에 선택적 `bookmark`를 더한다. HWP/HWPX 재열기 뒤 이름·URI·선택 범위가 보존되는 기존 실제 검사를 대조했다. Studio 변경은 정보 타입의 선택적 필드이며 클릭 이동 기능을 새로 구현한 것으로 보고하지 않는다. |

이 기능에서 새로 바꾼 것은 링크 대상의 인코딩·종류·존재 검증·조회 정보다. 같은
표시 문자열에 대한 기존 웹 링크 삽입·서식·조판 소비와 주소 종류를 분리했다.
그 때문에 이 PR의 target 메타데이터를 확인하려고 다른 HWP 문서나 Print 코퍼스를
추가하지 않는다. 기존 `hwpctl_Action_Table__v1.1.hwp` 검사는 실제 Command 문자열과
블록 책갈피의 동일 대상 no-op 근거이며 그 문서 전체의 시각 검증으로 승격하지 않는다.

독립 기대값은 실제 한컴 Command와 테스트의 한글·콜론·세미콜론·역슬래시 대상,
존재하지 않는 대상·동일 대상·웹/책갈피 종류 변경 거절, 두 저장 형식의 재열기다.
아래 실제 실행표의 네 계약 검사와 passthrough guard 실행을 재사용한다. 현재 진행 중인
공통 renderer 보정 이후의 통합 후보 필수 검증은 별도이며, 이 분석만으로 전체 통합
head의 새 검증이나 원격 CI를 완료 처리하지 않는다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/issue_6963_bookmark_hyperlink.rs](../../../tests/cases/issue_6963_bookmark_hyperlink.rs): SHA-256 `fb9ace1ce4c092a6ace6b6feadbd5678e8525e07d10da783d6bbc4284ea21a91`.
  - `bookmark_command_matches_hancom_encoding`: PASS, `logs/final-full-nextest-4052.log:7341` (`rhwp::regression_suite_012`).
  - `hancom_sample_bookmark_link_reports_its_bookmark`: PASS, `logs/final-full-nextest-4052.log:7345` (`rhwp::regression_suite_012`).
  - `bookmark_link_inserts_updates_and_survives_hwp_and_hwpx`: PASS, `logs/final-full-nextest-4052.log:7343` (`rhwp::regression_suite_012`).
  - `web_link_is_not_turned_into_bookmark_link`: PASS, `logs/final-full-nextest-4052.log:7344` (`rhwp::regression_suite_012`).
- [tests/issue_2724_passthrough_invalidation_guard.rs](../../../tests/issue_2724_passthrough_invalidation_guard.rs): SHA-256 `e2663ebd831dc39e4787a31c4cacd11a95cc2136a915d5cfae59047e37ebe7dd`.
  - `classification_drift_is_blocked`: PASS, `logs/final-full-nextest-4052.log:8333` (`rhwp::regression_suite_016`).
  - `stale_exemptions_are_reclaimed`: PASS, `logs/final-full-nextest-4052.log:8337` (`rhwp::regression_suite_016`).
  - `delegation_targets_actually_invalidate`: PASS, `logs/final-full-nextest-4052.log:8334` (`rhwp::regression_suite_016`).
  - `invalidation_density_ledger_is_ratcheted`: PASS, `logs/final-full-nextest-4052.log:8335` (`rhwp::regression_suite_016`).
  - `guard_scanner_self_check`: PASS, `logs/final-full-nextest-4052.log:8336` (`rhwp::regression_suite_016`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `a0bb863a08edf4e1461fbbf18ac2baaeaaea4450` | `edceca08577a63fbe28c7a913dd77e76fb86edff` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
