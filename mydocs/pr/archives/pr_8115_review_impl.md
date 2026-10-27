# PR #8115 적용·보정 실행 기록

- 원 head: `a47a41d3004a0458873cf1a00e90a356f19f2fa5`
- 통합 branch: `review/integration-20261026-document-updates`
- base: `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 완료한 로컬 code/test 검증 head: `1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559`
- 최종 판정·입력·명령·제한: [pr_8115_review.md](pr_8115_review.md)
- 단계 계획·독립 근거: [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)

## 실제 체리픽 순서

| 순서 | 원 SHA | 실제 적용 SHA / 처리 |
| ---: | --- | --- |
| 1 | `d23261a2ce4763092c05e648e8610d9d5c6d4732` | `8a1e0ffb42dbda42dbcecc318887ed8a208eb456` |
| 2 | `afa916e30402fd395d4b2742f2b4a5883ea58a9f` | `552f6da8f179bd1cccfcf23ed30fecadd44cbb9e` |
| 3 | `2d46ea9882fb08aa3f451d0cd72bfc9e0e62fcb0` | `c2ebab05c1c23a2a3d2eebe5c3d55e543897d3ab` |
| 4 | `bc88ca507fe4256d36f5a6471cc382d539b5c14c` | `c5bda263db7cdb2aa173e184ec5dbe35c0483305` |
| 5 | `93463d0545c294a42e9b891d473302dd978fbbd0` | `5de4aaef1a4f524599a234683787d17847653bc8` |
| 6 | `ddecddd9da13ea599c22fc0919614fd203190e29` | `e6f214b2104b3dfccd925bdb6f0a0a0c377535d4` |
| 7 | `a7aaa28434b875c8d695337ea4c9b053286bd05a` | `d22870e9b167d9fea007fcfe2cce30febdbd50bc` |
| 8 | `237ed1f5736f714a5878c93a8640bea343b0b163` | `a38dbd2b7a2ad65566bd8ae1a8bda2a5cf4b14dc` |
| 9 | `a47a41d3004a0458873cf1a00e90a356f19f2fa5` | `6436d1d68180f7603e6e2f11796db3491b200726` |

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
