# PR #8184 적용·보정 실행 기록

- 원 head: `567ff9d18ac1f7e702e94042eb38a03a44f95943`
- 통합 branch: `review/integration-20261026-document-updates`
- base: `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 완료한 로컬 code/test 검증 head: `1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559`
- 최종 판정·입력·명령·제한: [pr_8184_review.md](pr_8184_review.md)
- 단계 계획·독립 근거: [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)

## 실제 체리픽 순서

| 순서 | 원 SHA | 실제 적용 SHA / 처리 |
| ---: | --- | --- |
| 1 | `17e8267d99eeef5a5d32a304433ef0f84c20ecaf` | `94f7f8d88864dd67aef442d3e8dbfaaf8821da0f` |
| 2 | `5918b082bf14f5d2b8fb030087a3da35e545804f` | `c5d52c49d02d32abf9014243753da3c8aa2763db` |
| 3 | `90386d7de39a0092d645184c6ad783d16ae30013` | `7c569a1524cd47df32451a32abd87ec8310de687` |
| 4 | `f7626011e8c48a61ce589f11c93fd84608d149f7` | `8bbb26833885ca9488fc49abbf4b5d69256bfd93` |
| 5 | `0eab5aef81e66973001241f590af0e84186fe762` | `a8be358e1dd6cd9de885f981dfe496b26da9dc5c` |
| 6 | `553805f8615f19abaf3190216eb98d7f493b49f7` | `22aa4d8d9ea4bafce10176edca5aa39404562c82` |
| 7 | `05cef5908cf6d7b7a7580f38bfd7ae888b96e16f` | `ab095bf8b59f0b77ce94175ca9ab2fd3df55f214` |
| 8 | `c255b7320d1ef4eeff93d796877d37bbe8aa99eb` | `4d882e71b6fd2cf2f7b0f68f29f69a9972bb500c` |
| 9 | `add1ee699818af2a8364dee9ff051425a50da272` | `a4e0334f1d853f36dc3085f88f875151ef3ee868` |
| 10 | `2c6ebe553de79a3b21e0adaf5b2942322fd889e9` | `bb3437efe4261c10c556a6cdf1b16f2260bfd011` |
| 11 | `ca6112a6c91c2eef40f1b46e88e442b7723bdc2c` | `9ec584a15132a492fd6dc164649eb7f79aec7d56` |
| 12 | `29dbddf9ce78156860c2625598f4dacda0811ecc` | `b37c86dde8f65b09dad7351893ee3886c4b38ac4` |
| 13 | `62d23c09b1ff6f430484ddc46d57f2dd036e1ea1` | `c0a3c74b23754c2a88d490d799ff370ca35d04f2` |
| 14 | `78ba587d11ec911731e1c6eaa5a1a28c956536d3` | `22d99f138d8da7ae43a77b760dae356b80420928` |
| 15 | `b9334d5e2039c66261550da0f8bd4333a982d041` | `635d5eb12ac67e7b19bfd32e90f29f187316a6ed` |
| 16 | `5be3dd4c710f76af75ad9bd1efd92b7b1a02c9ca` | `e7537e3a93c3ff9b189d8faf146f548da3d719df` |
| 17 | `67525acc3c30cdc91ed78bb92e14db3dcb9f63bf` | `906eaf7530277df49e1066582b4c5f79e066c792` |
| 18 | `821ee71a40107fdbec3ba20289e49833e87fb122` | `d8372d2865a2e2f4dcd159474680c715ebd52998` |
| 19 | `00cd6ef8275bc0c96a08ccbce4a1a7e4150c866a` | `95b1418e47dbc878500f26d45b6c01de5a076edb` |
| 20 | `1db32c93f993d1e37bd0f204ac81eca7b86632ec` | `7f88c15d23fa1ade5540c036908631c653854a01` |
| 21 | `f862393e880095a1fdb8ff356e0f7aa7a3b78110` | `6ee44837af1f492b78b021d2c005d2ddc2c0c2f1` |
| 22 | `d878c2ad53a5caca5d1d099fdc2d5da3a718add1` | `46dce61d94bb098b1b55409b2f7d454e57aa2b34` |
| 23 | `1aa8a62489894af1b0c8adf2154e4422eb00eea9` | `ad8f4e5038f3d7707a9fa78b9e4a7ff65e33439e` |
| 24 | `567ff9d18ac1f7e702e94042eb38a03a44f95943` | `d8c1bd1d61cf234212f3b8913a77edab616f1d19` |

적층 선행 commit은 patch-id로 대조했다. merge commit을 다시 체리픽하지 않았고 원 저자·`-x` 출처를 보존했다.
#8227·#8237의 사전 가상 충돌은 적층 부모를 명시한 예상 tree와 실제 결과가 일치했다. 실제 체리픽 충돌은 없었다.

## 보정·검증·후속 경계

1. 고정 원 head 적용과 예상 tree·중복을 원장으로 확인했다.
2. 승인된 기존 미달 페이지를 공통 로직에서 보정하고 단계별 일반 커밋으로 고정했다.
3. 관련 전쪽 Native/fresh WASM 선행 검증·직접 판독 뒤 정식 소유 회귀를 보강하고, 최종 최적화 WASM 전쪽·기준 대비 하락 기준을 검증했다.
4. 전체 필수 로컬 검증을 실제 완료한 뒤 원 PR별 이 기록을 작성했다.
5. 통합 PR을 제출한다. 이후 원격 CI·승인·머지·원 PR 종료는 별도 후속 조건이다.

보정 SHA: `065b49490cce13c91b8e635e0d3fe2b7ca64cf42`, `0e5dfd02642624d013bdc51c5d67839a6f67fd72`, `642d29dab1d06596de784c113527a2482073e828`.

원 contributor head를 rewrite하거나 원격 브랜치를 변경하지 않았다. 되돌림은 필요 시 이 원장의 적용 commit과
관련 공통 보정을 검토해 별도 revert로 처리하며 다른 PR 변경까지 한꺼번에 롤백하지 않는다.
