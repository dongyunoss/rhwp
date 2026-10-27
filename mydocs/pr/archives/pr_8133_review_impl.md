# PR #8133 적용·보정 실행 기록

- 원 head: `465d85f13cab961628ec07247ea6ade82e0cbc46`
- 통합 branch: `review/integration-20261026-document-updates`
- base: `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 완료한 로컬 code/test 검증 head: `1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559`
- 최종 판정·입력·명령·제한: [pr_8133_review.md](pr_8133_review.md)
- 단계 계획·독립 근거: [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)

## 실제 체리픽 순서

| 순서 | 원 SHA | 실제 적용 SHA / 처리 |
| ---: | --- | --- |
| 1 | `5fdd263ee9f05cc977d5bee5cabe03849f6d995c` | `827b5648414eadbec889dd4d4068a1785f9bac3b` |
| 2 | `d39696d63cd6c7ff03c65b0ecb429827b91ce252` | `efa62c14eccb840a395c0b7da80bf6a52ebb4bbf` |
| 3 | `68e11eb4b78a837e9c935a5ce04d6e9539110618` | `11fdf1fb986c94173d6cda84209343efb6edcda9` |
| 4 | `c0b1451409d1e5e529fa5009c3849105440b75a1` | `f73494e5517725759c9414785d5e6536f1730285` |
| 5 | `b733b47ec089f72a2a1786635f25dcb9032f6c67` | `fed6122337595052b233743687630d784a8cbe82` |
| 6 | `d40fa79b0124c2fc254b466819ad5aaeb5664bac` | `4cfdacc32c31ceb87c69b99829bbecb998de0563` |
| 7 | `57d2b6d4f56cf9c0d9baf0978f3a65aafc63b7dd` | `845b57e3fdef78f8dddb16dbbf8f1da623d79b6b` |
| 8 | `c273dcecb5e5c475caeed769a77d75829176a61e` | `07a14e4fd1b27f9182b6bcb016fdaeefb4819c1f` |
| 9 | `3c31f51f9c656d0070f29fd17b0f669ff763554c` | `f9a9bf6b73a9ec6136605846447c49ad8340e7c9` |
| 10 | `561f849625c17f63eda6ca84b60ac59fa5dd7025` | `47ba5dfcc26070db1510fc574c976da1434460fd` |
| 11 | `9af0aeceb2331fe5d29bedcfd9535aa6f91d8071` | `d6ae01db05b08230876dc770ccd4b8732b844166` |
| 12 | `465d85f13cab961628ec07247ea6ade82e0cbc46` | `856f7265c702eb57801e21b5c168aa7819ebcf2c` |

적층 선행 commit은 patch-id로 대조했다. merge commit을 다시 체리픽하지 않았고 원 저자·`-x` 출처를 보존했다.
#8227·#8237의 사전 가상 충돌은 적층 부모를 명시한 예상 tree와 실제 결과가 일치했다. 실제 체리픽 충돌은 없었다.

## 보정·검증·후속 경계

1. 고정 원 head 적용과 예상 tree·중복을 원장으로 확인했다.
2. 승인된 기존 미달 페이지를 공통 로직에서 보정하고 단계별 일반 커밋으로 고정했다.
3. 관련 전쪽 Native/fresh WASM 선행 검증·직접 판독 뒤 정식 소유 회귀를 보강하고, 최종 최적화 WASM 전쪽·기준 대비 하락 기준을 검증했다.
4. 전체 필수 로컬 검증을 실제 완료한 뒤 원 PR별 이 기록을 작성했다.
5. 통합 PR을 제출한다. 이후 원격 CI·승인·머지·원 PR 종료는 별도 후속 조건이다.

보정 SHA: `343142255a9ba509223bfff99016aad68eb1d16e`, `7ccf8dffc8afe509fb08b20ddb4bab292178d87a`, `d0ba736ddaa89917450bec032e807e55ee27aa7f`.

원 contributor head를 rewrite하거나 원격 브랜치를 변경하지 않았다. 되돌림은 필요 시 이 원장의 적용 commit과
관련 공통 보정을 검토해 별도 revert로 처리하며 다른 PR 변경까지 한꺼번에 롤백하지 않는다.
