# PR #8031 구현·검증 기록

## 범위와 고정 기준

[개별 리뷰](pr_8031_review.md)의 원 head `3b79ee07292b6d386af1ee9e909e0a5549ec52f8`와 실제 소비 경로를 따른다. `review/semanticist21-20261019`에서 고정 devel `238de6de41a208648f4ade6b494fe433a81c1fce` 위로 누적한 45개 원 PR 중 하나다. 추가 PR을 체리픽하거나 다른 HWP로 범위를 확대하지 않는다.

## 원 커밋과 통합 대응

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `3b79ee07292b6d386af1ee9e909e0a5549ec52f8` | `e70dbe177571863d533c72237feceb67344605d6` |

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

- [tests/cases/page_number_pos_in_table_cell.rs](../../../tests/cases/page_number_pos_in_table_cell.rs): SHA-256 `54fbd165a9ab9ac43b914beb5675cd93f930eeba58ca8eee98b1fd759dbb2051`.
  - `legacy_paginator_reads_page_number_pos_inside_table_cell`: PASS, `logs/current-full-nextest-4184.log:5405` (`rhwp::regression_suite_005`).
  - `page_number_pos_inside_table_cell_paints_page_number`: PASS, `logs/current-full-nextest-4184.log:5406` (`rhwp::regression_suite_005`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

## API 생성 Print 대조군 추가 — 2026-10-26

작업 시작 head는 `a72ce747362f537cc6dba4906dd38a8ea40daece`, 고정 base는
`49bc935a149c20b942d3045c2f5c733aeccf57d0`다. 엔진은 c2513250과 동일하며
이번 단계에서는 제품 코드를 수정하지 않았다. 기존 최소 IR 검사·입력의 미달을
완료된 다른 입력으로 대체하지 않는다. 아래 표는 새 대조군에 한정한다.

| 항목 | 직접 확인한 결과 |
| --- | --- |
| 독립 입력·Print | [입력·기준 PDF·생성 절차·SHA](../assets/pr_8031_page_number_cell/README.md) |
| Native/fresh 전쪽 | 각각 1/1쪽, raw 100.00000%, 미달·누락 0쪽 |
| 직접 판독 | 양쪽 review·standalone overlay에서 표 외곽·후속 BODY·꼬리말을 대조함 |
| 새 이름의 정식 검사 | `page_number_cell_print_contract.rs`: 2 PASS/0 FAIL, `regression_suite_007` |
| 검사 SHA-256 | `eac944341598401ea1640e235ac4f9053435ead0a845341f99e8565858f1affe` |
| 독립 기대값 | 셀 선언 소유·각 1회 내용 보존·순서 및 Print 꼬리말 기준선 ±2px |
| 기존 검사 유지 | `page_number_pos_in_table_cell.rs`는 HEAD의 바이트 그대로 유지함 |
| 판정 경계 | 기존 최소 IR 원본의 시각 미달과 통합 전체 잔여 게이트는 아직 해결로 세지 않음 |

자동 suite 최초 실행은 오래된 생성 목록으로 0건·exit4였고, manifest `--prepare` 후
새 함수 2건을 실제 실행해 exit0을 확인했다. base070에서도 쪽 번호가 보이므로
수정 전 FAIL 근거라고 기록하지 않는다. 이는 상속 기능 보존의 양성 대조다.
로그는 `output/pr-review/semanticist21-20261019/logs/page-number-cell-auto-nextest-4322.log`다.
기존 과거 전체 검증 수치는 아래/앞선 기록의 head에만 적용한다.
