# planet6897 문서 처리 PR 체리픽 통합 계획

- 작업 브랜치: `review/integration-20261026-document-updates`
- 기준: `upstream/devel` `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 사용자 지시: 충돌 없는 열린 PR을 체리픽 통합한 뒤 검토한다. #8237도 편람 수정이 아니면 포함한다.
- 편람을 성능측정 대조군으로 사용하는 것만으로 제외하지 않는다. 성능 PR #8205·#8209·#8210·#8213·#8215·#8217은 포함한다.
- 실제 편람 수정 #8268과 충돌 상태 #8161은 이번 통합에서 제외하고 열린 상태를 유지한다. 두 PR에 안내 댓글이나 close를 수행하지 않는다.
- 기본 경로는 collaborator 매개 외부 PR의 주 작업공간 체리픽 통합이다. 원 PR별 reviewer `jangster77` 지정·API 확인 후 조사·fetch했다.
- 각 원 head는 고정 ref로 fetch해 SHA를 확인했다. 최신 기준과 각 원 head의 `git merge-tree --write-tree`는 20건 모두 exit 0이다.
- 통합 전체에 대응하는 단일 대표 이슈가 없어 작업 시작일을 브랜치 식별자로 사용한다.

## 고정 원 PR

| PR | 고정 원 head | 통합 축 |
| --- | --- | --- |
| [#8115](https://github.com/edwardkim/rhwp/pull/8115) | `a47a41d3004a0458873cf1a00e90a356f19f2fa5` | 표·각주 경계 |
| [#8128](https://github.com/edwardkim/rhwp/pull/8128) | `21c40f21c5ee019fdf1ee4e68bdfeed35d3ae26c` | 이어지는 쪽 저장 reset·자기 각주 |
| [#8133](https://github.com/edwardkim/rhwp/pull/8133) | `465d85f13cab961628ec07247ea6ade82e0cbc46` | Native/Canvas/CanvasKit 획 |
| [#8142](https://github.com/edwardkim/rhwp/pull/8142) | `e1abc4c7f149268ea63995eeedeb02cfcc92e5cd` | 병합 칸 안쪽 행 |
| [#8184](https://github.com/edwardkim/rhwp/pull/8184) | `567ff9d18ac1f7e702e94042eb38a03a44f95943` | 빈 host 자리차지 표 |
| [#8202](https://github.com/edwardkim/rhwp/pull/8202) | `d0f6adbeef179e17d0431eb84c38443d318a887a` | 글자처럼 도형 |
| [#8205](https://github.com/edwardkim/rhwp/pull/8205) | `5de6de6a1586e4ce714ee4c8e51a92a3b4812c97` | 수식·미주 측정 캐시 |
| [#8206](https://github.com/edwardkim/rhwp/pull/8206) | `dd3ad0b29bb55893caf8d618e42a24776d64673f` | 글자처럼 그림과 위아래 개체 |
| [#8209](https://github.com/edwardkim/rhwp/pull/8209) | `01588f1e997b37795d2dbafecdf1f015341d553c` | 글꼴 판정·봉인 다이제스트 |
| [#8210](https://github.com/edwardkim/rhwp/pull/8210) | `276d569c2f5abce5b77bfb89aed016951e314bd3` | 큰 표 스캔·높이 측정 |
| [#8213](https://github.com/edwardkim/rhwp/pull/8213) | `d9aab2292be0da4c71dc2c28665fd9ddc8c07a59` | 중첩 표·제목행·행 상자 |
| [#8215](https://github.com/edwardkim/rhwp/pull/8215) | `31e5e700791c89c743feacd84acc55bc034569a3` | HWP5 그림 저장 스트림 |
| [#8217](https://github.com/edwardkim/rhwp/pull/8217) | `b717326395c5e9556f6ccb0ec3dc1c127c2daf49` | HWPX 그림 deflate |
| [#8219](https://github.com/edwardkim/rhwp/pull/8219) | `bfe5d44cf1d08806d9ac12aff1096096ed0853c0` | HWP3 BinData 순서 |
| [#8221](https://github.com/edwardkim/rhwp/pull/8221) | `1a69db15352e171d2a16b4195d78cc445af2e9e6` | PDF 문단 경계 |
| [#8227](https://github.com/edwardkim/rhwp/pull/8227) | `a34d1fb70dcdc2e1f692efaa63bfd2d77ee0e74c` | PDF 편집 우선 배치 |
| [#8229](https://github.com/edwardkim/rhwp/pull/8229) | `195ce6297541f2136ea02fb55cdc531320d2d714` | Studio PDF 배치 설정 |
| [#8237](https://github.com/edwardkim/rhwp/pull/8237) | `f58e857b1294dd1ed0ea1f355f6323fbc5dbcd51` | PDF 머리·꼬리 띠 분리 |
| [#8265](https://github.com/edwardkim/rhwp/pull/8265) | `65196bb3f97293763b7736438feb6d47a5072922` | Studio AI 제공자 |
| [#8283](https://github.com/edwardkim/rhwp/pull/8283) | `43d7eb24e84c84e6fd7a817dad579b9baebaaae3` | HWPX 재열기와 그림 host |

## 적용과 검증 순서

1. 원 PR 번호 순서로 기능·테스트·증적 commit을 `git cherry-pick -x`한다. merge commit은 제외한다.
   PDF·성능·그림 host의 적층 PR은 이미 적용한 원 SHA와 patch-id를 대조해 중복 commit을 다시 적용하지 않는다.
   원 저자·원 SHA·적용 SHA·순서·중복·보정은 원장에 기록하며 원 fork head는 변경하지 않는다.
2. 체리픽 전 기준 Native CLI와 최적화 WASM을 고정 공용 `target/pr-review`에서 순차 준비하고 source/build/font provenance를 기록한다.
   기준 코드·후보 코드의 같은 입력·페이지 비교에 사용하며, 원 PR의 첨부 수치를 통합 head의 직접 검증으로 대신하지 않는다.
3. 통합 후 각 변경의 실제 소비 경로·독립 기대값·반례·focused 검사를 검토한다. 각주·표 분할·저장 줄·획·그림 저장과 PDF 두 배치를 구분한다.
   이미 해결된 이슈를 보정할 때 원 해결 PR·통합/메인터너 commit·독립 근거를 먼저 추적한다.
4. Rust fmt·세 Clippy·workspace build·suite/unit tier 정책·전체 nextest·Native Skia 3종을 순차 실행한다.
   Studio TypeScript·단위·실제 브라우저, 두 제공자·PDF 설정, HWP/HWPX 그림 보존·재저장 계약을 검증한다.
5. 실제 출력 영향 입력의 전쪽 Native/fresh 최적화 WASM TSV와 대표 확대 판독을 수행한다.
   최소 90%와 기준 대비 페이지별 2%p 하락 gate를 모두 적용하고, 기존 목표 문서의 개선도 보존한다.
   편람 수정 자체는 #8153의 별도 작업으로 유지한다. 성능 대조군 언급과 편람 조판 수정 범위를 혼동하지 않는다.
6. 승인된 검증·보정을 마친 뒤 통합 PR을 제출한다. 정확한 code head CI 성공 후 원 PR별 archive review와 오늘할일을
   같은 통합 PR의 trailing 문서 commit에 반영한다. 각 원 PR의 수용/보류와 실제 검증 범위를 분리해 기록한다.
7. 최신 trailing head의 CI·정책·병합 가능 상태 확인 후 통합하고, 반영한 원 PR만 통합 사실을 정중히 안내해 종료한다.
   #8268·#8161에는 이 종료 처리를 적용하지 않는다. 원 contributor fork branch는 삭제하지 않는다.

## 산출물과 자원 경계

- 작업 중 임시 파일·렌더 결과·검증 로그: `output/pr-review/planet6897-20261026/`.
- 선정·원 SHA·검토 gate·명령/결과 운영 기록: `/home/tsjang/rhwp-pr-evidence/planet6897-20261026/`.
- Cargo는 `/home/tsjang/rhwp/target/pr-review` 하나를 순차 재사용한다. 아직 실행 중인 빌드·테스트 산출물은 삭제하지 않는다.
- 정식 입력·한컴 기준 PDF·전쪽 TSV·대표 PNG는 역할별 정식 경로에 반영한다. 대용량 PDF는 별도 로컬 보관 규칙을 따른다.
- 검토·후속 처리 종료 후 output 전체와 사용 종료된 review debug를 별도 지시·백업 없이 정리한다.
  다른 profile과 다른 작업의 worktree를 보존하며 유휴 셸 cwd만으로 삭제를 보류하지 않는다.
