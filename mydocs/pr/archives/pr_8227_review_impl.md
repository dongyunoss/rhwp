# PR #8227 적용·보정 실행 기록

- 원 head: `a34d1fb70dcdc2e1f692efaa63bfd2d77ee0e74c`
- 통합 branch: `review/integration-20261026-document-updates`
- base: `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 완료한 로컬 code/test 검증 head: `1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559`
- 최종 판정·입력·명령·제한: [pr_8227_review.md](pr_8227_review.md)
- 단계 계획·독립 근거: [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)

## 실제 체리픽 순서

| 순서 | 원 SHA | 실제 적용 SHA / 처리 |
| ---: | --- | --- |
| 1 | `b8a0e876d5fad426108ea7173f9c6ab8d7f47e17` | 동일 patch 선행 반영으로 중복 제외 |
| 2 | `2b3982b028b5b80398d6d0b75bf96716762995a3` | 동일 patch 선행 반영으로 중복 제외 |
| 3 | `1a69db15352e171d2a16b4195d78cc445af2e9e6` | 동일 patch 선행 반영으로 중복 제외 |
| 4 | `462c3dd380da3e3f617c39082f341efcb8fc6afe` | `2ff52e2d79726a541bf5e6e52780c7abbecfbd15` |
| 5 | `a19b8bc9359d8b80d86c7b497d9d1e7f40c72774` | `56870979a454b0335b6f609c56516b1e2400486b` |
| 6 | `6d1ad105f8b56d4639855200e1b6ccf9b7ddc618` | `3453e4d4b6505377c06fbeb4ff173b866aae4271` |
| 7 | `a34d1fb70dcdc2e1f692efaa63bfd2d77ee0e74c` | `87eacd3484cd9b8009baf395d817da3315bd4916` |

적층 선행 commit은 patch-id로 대조했다. merge commit을 다시 체리픽하지 않았고 원 저자·`-x` 출처를 보존했다.
#8227·#8237의 사전 가상 충돌은 적층 부모를 명시한 예상 tree와 실제 결과가 일치했다. 실제 체리픽 충돌은 없었다.

## 보정·검증·후속 경계

1. 고정 원 head 적용과 예상 tree·중복을 원장으로 확인했다.
2. 승인된 기존 미달 페이지를 공통 로직에서 보정하고 단계별 일반 커밋으로 고정했다.
3. 관련 전쪽 Native/fresh WASM 선행 검증·직접 판독 뒤 정식 소유 회귀를 보강하고, 최종 최적화 WASM 전쪽·기준 대비 하락 기준을 검증했다.
4. 전체 필수 로컬 검증을 실제 완료한 뒤 원 PR별 이 기록을 작성했다.
5. 통합 PR을 제출한다. 이후 원격 CI·승인·머지·원 PR 종료는 별도 후속 조건이다.

보정 SHA: 이 PR 단독 보정 없음; 공통 통합 보정은 상위 계획 참조.

원 contributor head를 rewrite하거나 원격 브랜치를 변경하지 않았다. 되돌림은 필요 시 이 원장의 적용 commit과
관련 공통 보정을 검토해 별도 revert로 처리하며 다른 PR 변경까지 한꺼번에 롤백하지 않는다.
