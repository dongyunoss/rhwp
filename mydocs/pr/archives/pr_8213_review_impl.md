# PR #8213 적용·보정 실행 기록

- 원 head: `d9aab2292be0da4c71dc2c28665fd9ddc8c07a59`
- 통합 branch: `review/integration-20261026-document-updates`
- base: `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 완료한 로컬 code/test 검증 head: `1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559`
- 최종 판정·입력·명령·제한: [pr_8213_review.md](pr_8213_review.md)
- 단계 계획·독립 근거: [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)

## 실제 체리픽 순서

| 순서 | 원 SHA | 실제 적용 SHA / 처리 |
| ---: | --- | --- |
| 1 | `ad6a918fa7e872d84f13f925c4da35e8f0b49819` | 동일 patch 선행 반영으로 중복 제외 |
| 2 | `b360b1cd3437327a81304b70cbb32a0af32ff7bf` | 동일 patch 선행 반영으로 중복 제외 |
| 3 | `ab52479a8e6abd7291c134005356f8a7d775a1c6` | 동일 patch 선행 반영으로 중복 제외 |
| 4 | `3920830577969a6bfbe256d6970a21f3216dc64e` | 동일 patch 선행 반영으로 중복 제외 |
| 5 | `e405b57d2a648db40993b74dbeb9bb40fd8a7df9` | 동일 patch 선행 반영으로 중복 제외 |
| 6 | `df4b51036e736bf626032c8afe23240987ef3cd1` | 동일 patch 선행 반영으로 중복 제외 |
| 7 | `e0355691ab52253e4c6b5b4714b5b64a85804869` | 동일 patch 선행 반영으로 중복 제외 |
| 8 | `ef90aed188c899fb5d67d33725523d057d925765` | 동일 patch 선행 반영으로 중복 제외 |
| 9 | `a6060b598c785cc5a7c1d63a1e3f4ff5e46b62dd` | 동일 patch 선행 반영으로 중복 제외 |
| 10 | `276d569c2f5abce5b77bfb89aed016951e314bd3` | 동일 patch 선행 반영으로 중복 제외 |
| 11 | `32301f8a2d3f2b88fe037b37a2af8167f2d81b3c` | `626c676302c902fb085756d6d6512a8ec43bf850` |
| 12 | `2e720473b79548a5d5fc38346ab4348447f33384` | `f510ce652ed39036c5cb101962c5add91942026d` |
| 13 | `d1b4197b988a4117886aa771d297030752d586c6` | `9603550d08932417e248f29e0ea13716dc376048` |
| 14 | `46f702c76b69c7d4bf3408404b70683b172caf15` | `ed00e9d2b24080bb121bdd4205d75d482256e2a7` |
| 15 | `6621445d1510b26b5dc2835c692cda91a7977d29` | `11e7e46912e026fdde9d55363052d904c8ae6909` |
| 16 | `2a8142924baa7da16e151a3a4520bf564eb7f354` | `2edbfb844aae32266d0b35028ae078797e3473eb` |
| 17 | `d9aab2292be0da4c71dc2c28665fd9ddc8c07a59` | `c2aebcf431fe69895c3a469182962127f7b54999` |

적층 선행 commit은 patch-id로 대조했다. merge commit을 다시 체리픽하지 않았고 원 저자·`-x` 출처를 보존했다.
#8227·#8237의 사전 가상 충돌은 적층 부모를 명시한 예상 tree와 실제 결과가 일치했다. 실제 체리픽 충돌은 없었다.

## 보정·검증·후속 경계

1. 고정 원 head 적용과 예상 tree·중복을 원장으로 확인했다.
2. 승인된 기존 미달 페이지를 공통 로직에서 보정하고 단계별 일반 커밋으로 고정했다.
3. 관련 전쪽 Native/fresh WASM 선행 검증·직접 판독 뒤 정식 소유 회귀를 보강하고, 최종 최적화 WASM 전쪽·기준 대비 하락 기준을 검증했다.
4. 전체 필수 로컬 검증을 실제 완료한 뒤 원 PR별 이 기록을 작성했다.
5. 통합 PR을 제출한다. 이후 원격 CI·승인·머지·원 PR 종료는 별도 후속 조건이다.

보정 SHA: `8c699b0c6a8548885fc572e01b831b981273c380`, `71d7ac1d0f7eacf3e318c37a0a1bbc8c26fa1f33`, `2f358577db579aedc822b4e5f220dee414d42f66`, `3ae26ef0a1a20a8810813a085179220f56d2561a`, `f34157026afad07c423e58a9b3018b9c46593325`, `642d29dab1d06596de784c113527a2482073e828`, `401d9070b6bf3cca3803c3c87ac88a0ad4a07a6b`, `56a638b9960e8bad9c3f73185a17665d7f7c94b1`, `59c7fef1674658c138625cc9348becbef5cbcd04`.

원 contributor head를 rewrite하거나 원격 브랜치를 변경하지 않았다. 되돌림은 필요 시 이 원장의 적용 commit과
관련 공통 보정을 검토해 별도 revert로 처리하며 다른 PR 변경까지 한꺼번에 롤백하지 않는다.
