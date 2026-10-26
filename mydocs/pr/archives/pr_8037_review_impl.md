# PR #8037 구현·검증 기록

## 범위와 고정 기준

[개별 리뷰](pr_8037_review.md)의 원 head `66736eac830d16db841e0c59fb3cbebe64579a9f`와 실제 소비 경로를 따른다. `review/semanticist21-20261019`에서 고정 devel `238de6de41a208648f4ade6b494fe433a81c1fce` 위로 누적한 45개 원 PR 중 하나다. 추가 PR을 체리픽하거나 다른 HWP로 범위를 확대하지 않는다.

## 원 커밋과 통합 대응

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `66736eac830d16db841e0c59fb3cbebe64579a9f` | `1ef765106488199dfbd0b59fd1bda531dc74a34a` |

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

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/picture_on_table_host_keeps_following_flow.rs](../../../tests/cases/picture_on_table_host_keeps_following_flow.rs): SHA-256 `5b646e55e3c188c972f0c91098277fc6cbd57ab16c03dae876e45dafa55181a4`.
  - `a_cell_picture_keeps_the_table_and_the_following_paragraph_in_place`: PASS, `logs/current-full-nextest-4184.log:5973` (`rhwp::regression_suite_007`).
  - `a_floating_picture_on_the_table_paragraph_keeps_the_flow_in_place`: PASS, `logs/current-full-nextest-4184.log:5974` (`rhwp::regression_suite_007`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
