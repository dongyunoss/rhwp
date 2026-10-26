# planet6897 문서 처리 PR 체리픽 통합 계획

- 작업 브랜치: `review/integration-20261026-document-updates`
- 기준: `upstream/devel` `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 사용자 지시: 충돌 없는 열린 PR을 체리픽 통합한 뒤 검토한다. #8237도 편람 수정이 아니면 포함한다.
- 편람을 성능측정 대조군으로 사용하는 것만으로 제외하지 않는다. 성능 PR #8205·#8209·#8210·#8213·#8215·#8217은 포함한다.
- 실제 편람 수정 #8268과 충돌 상태 #8161은 이번 통합에서 제외하고 열린 상태를 유지한다. 두 PR에 안내 댓글이나 close를 수행하지 않는다.
- 2026-10-27 사용자 지시로 이미 체리픽한 아래 20건으로 범위를 고정한다. 이후 열린 PR은 추가로 체리픽하지 않는다.
  별도 HWP 시각 검증은 원 PR의 직접 대상·재현 입력에 한정하며 불필요한 다른 문서 대조를 추가하지 않는다.
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
6. 최신 사용자 지시에 따라 승인된 로컬 검증·필요한 메인터너 보정을 마친 뒤 원 PR별 archive review를 작성해
   같은 통합 PR에 포함한다. 문서에는 실제 로컬 결과와 미실행 원격 CI를 구분한다. 통합 PR의 정확한 head CI를 확인하고
   각 원 PR의 수용/보류와 실제 검증 범위를 기록한다.
7. 현재 작업은 로컬 검증·원 PR별 검토 문서·통합 PR 제출까지 진행한다. 이후 CI 결과를 보고하며, 원격 머지·원 PR 종료는
   이 작업에 대한 별도 사용자 지시를 따른다. #8268·#8161은 제외한 열린 상태를 유지하고 원 contributor fork branch는 보존한다.

## 산출물과 자원 경계

- 작업 중 임시 파일·렌더 결과·검증 로그: `output/pr-review/planet6897-20261026/`.
- 선정·원 SHA·검토 gate·명령/결과 운영 기록: `/home/tsjang/rhwp-pr-evidence/planet6897-20261026/`.
- Cargo는 `/home/tsjang/rhwp/target/pr-review` 하나를 순차 재사용한다. 아직 실행 중인 빌드·테스트 산출물은 삭제하지 않는다.
- 정식 입력·한컴 기준 PDF·전쪽 TSV·대표 PNG는 역할별 정식 경로에 반영한다. 대용량 PDF는 별도 로컬 보관 규칙을 따른다.
- 검토·후속 처리 종료 후 output 전체와 사용 종료된 review debug를 별도 지시·백업 없이 정리한다.
  다른 profile과 다른 작업의 worktree를 보존하며 유휴 셸 cwd만으로 삭제를 보류하지 않는다.

## 2026-10-27 통합과 첫 보정

- 원 PR 20건의 고유 commit 94개를 `-x`로 적용하고 적층 중복 35개를 제외했다.
  원본·적용 SHA와 순서는 [적용 원장](assets/review_integration_20261026/cherry_pick_ledger.json)에 기록한다.
  체리픽 완료 head는 `be1de8e41ed32f461706f9c8ddcefcfd392b0e5c`다.
- 각 PR 적용 결과는 누적 기준에 원 head의 변경을 병합한 예상 tree와 모두 일치했다.
  #8227·#8237의 사전 tree 비교는 이미 체리픽한 적층 부모를 각각 `1a69db153`·`a34d1fb70`으로
  명시했다. 중복 도입에 따른 함수 공개 범위·검사 파일의 가상 충돌이었으며 실제 체리픽 충돌은 없었다.
- 기준 Native 빌드와 합성 입력 21쌍의 전쪽 비교를 완료했다. 기준 WASM은 원본 head에서 Rust
  컴파일을 끝낸 뒤 바이너리 최적화와 코드 체리픽을 병행했다. 기준 패키지/public의 JS·WASM 해시와
  복사한 기준 모듈의 `WebAssembly.compile`을 확인했다. 후보 비교는 남았다.
- 사용자가 불필요한 범위 확장을 지적해 sample16 별도 시각 재검증과 그 자식 프로세스를 종료했다.
  이 별도 재검증은 재실행하지 않는다.
- #8265 원 head의 `readSse`는 CRLF의 CR과 LF가 서로 다른 청크에 오면 LF를 빈 줄로 처리했다.
  event 이름과 여러 data 줄이 사라지는 실패를 원 코드와 실제 새 검사에서 각각 확인했다.
  [SSE 해석 규약](https://html.spec.whatwg.org/multipage/server-sent-events.html#event-stream-interpretation)에
  따라 청크 끝 CR을 다음 청크 또는 EOF까지 보존했다. 모든 바이트 분할·UTF-8·LF/CR/CRLF 검사와
  기존 제공자 검사를 합쳐 22개가 통과했고 Studio TypeScript 검사도 통과했다.
  보정 commit은 `57abd92f0`이며 실제 API 키는 사용하지 않았다.
- #8167·#8168·#8271 입력의 마지막 저장 제품을 확인하고, 누락됐거나 다른 저장본·세대의 PDF만 있던
  8개 입력을 자체 원본에서 MCP 2020 Print로 출력했다. 입력·PDF 해시, 실제 한컴 버전,
  `pdf_print_method=0`·1-up 출력 모드는 [Print 근거](assets/review_integration_20261026/hancom_print_inputs.json)에 기록한다.
  이 PDF와 기존 PR의 2024 비교는 세대를 구분한다.
- 현재 수용 판정은 미완료다. Rust 전체 검증, Native/fresh WASM 후보 전쪽 비교, 관련 Studio 브라우저
  검증과 확대 판독을 마친 뒤 PR을 준비한다. 최신 사용자 지시대로 로컬 검증 후 원 PR별 archive review를 먼저 작성하고
  통합 PR을 만든다. 지침의 기본 CI 후 trailing 순서보다 이 명시적 지시를 우선한다.
- Studio 전체 단위 검사는 2,599개 통과·기존 skip 2개·실패 0개로 완료했다.
- 첫 통합 Native 빌드는 `stored_tac.rs`의 E0614로 실패했다. #8184의 `9ec584a15`는 다음 문단을
  `*first + 1`로 조회했고, 이후 #8205의 `9e72098ba`가 같은 함수의 패턴을 값 복사로 바꿔 `first`가
  `usize`가 됐다. 두 원 변경과 `d254c9bac` 원 commit을 추적했다. 추가된 다음 문단 조회도
  `first + 1`로 맞췄으며 조판 정책·기대값은 바꾸지 않았다. 보정 head에서 Native 빌드부터 다시 검증한다.


## 기존 미달 페이지 보정과 CanvasKit 검토

- 사용자는 기존 미달 오류도 이번 브랜치에서 보정하도록 지시했다. #8133의 sungeo 21·22쪽,
  #8184의 hwpspec 23·54쪽, #8213의 중첩 표 입력 2·8·9·10·11·12·14쪽을 포함하며,
  해당 PR을 제외하거나 입력·기준값을 바꿔 회피하지 않는다. 추가 PR은 체리픽하지 않는다.
- 이 보정 전에 진행하던 전체 Rust 검사는 새 코드가 필요해 중지했다. 6,769개 부분 통과·실패 0개는
  완료 결과가 아니다. 기존 후보의 Native 빌드·Clippy 3종·workspace build·suite/unit-tier 정책은
  통과했으며, 최종 보정 head에서 필요한 전체 검증을 완료한다.
- #8133 CanvasKit의 화면 배율만 사용하는 선 두께 하한은 축소된 자식 그룹에서 실제 1픽셀을
  보장하지 못했다. 실제 CanvasKit 소프트웨어 raster에서 반배율 도형 경계가 회색 127로
  그려졌고, 원래 두께 0.48의 HWP 점선 주기도 보정된 획 두께로 계산돼 `[6.46, 3.62]`가 됐다.
  Native·Canvas2D와 동일하게 현재 canvas의 역변환으로 장치 1픽셀을 환산하고, 점선 주기는
  원래 두께를 사용한다. 화면용 SkPicture 기록에서는 외부 변환도 반영하고 cache key에 포함하며
  실패 시 변환 상태를 복원한다. 인쇄 기록은 기존 실제 장치 CTM을 사용한다.
- 실제 raster 경계와 점선 검사 2개는 수정 전 실패·수정 후 통과했다. 항등·축소·확대·회전 변환과
  기록 실패 복원까지 총 8개가 통과했다. TypeScript 검사와 Studio 전체 단위 검사도
  2,607개 통과·기존 skip 2개·실패 0개다. HWP 전쪽 Native/fresh WASM 검증은 아직 남았다.
- 실행 로그는 `output/pr-review/planet6897-20261026/`의 `canvaskit-contract-before.log`,
  `canvaskit-contract-after.log`, `canvaskit-tsc.log`, `studio-unit-after-canvaskit.log`에 있다.


### sungeo 저장 프레임 후속 경계

- 원본 2구역의 문단 43은 저장 위치 64,594HU를 가진 표제어지만 다음 본문과 함께 새 쪽에
  수용된다. 뒤 문단 44부터 새 저장 프레임의 본문을 소비해도, 이전 prefix 판정이 첫 표제어의
  옛 위치만 보고 문단 65·85의 후속 쪽 경계를 계속 무효화했다.
- 원 수정 `8cc5042c1`과 보완 `7f7cb6086`, 관련 #7642의 stage2649·2666·3429 기록을
  대조했다. 기존 whole-fit이 사용하는 실제 소유자·저장 끝 대조 결과를 prefix 유효성 조회에도
  사용한다. 실제 흐름이 저장 프레임 끝에 도달하면 첫 항목의 옛 좌표를 쪽 전체의 무효 상태로
  사용하지 않는다. 명시적 경계와 TAC 원점 계약은 그대로 소비한다.
- `legacy-v2` Native는 수정 전 21쪽 26.13435%·22쪽 38.97308%에서 각각 99.94110%·
  99.93512%로 개선됐다. 전체 94쪽 최저 99.85165%, 90% 미만 0쪽, PDF와 쪽수 일치다.
  fresh WASM·최종 전체 Rust 검증은 아직 남아 있으며 이를 완료 전 전체 수용으로 판정하지 않는다.
- 빌드와 전쪽 로그: `output/pr-review/planet6897-20261026/legacy-v2-native-build.log`,
  `sungeo-v2-all.log`. 고정 바이너리와 source diff 해시는 `legacy-v2-producer.json`에 기록했다.
- OpenAI·Gemini 제공자 E2E, AI panel·설정·side panel E2E도 통과했다. 모의 API이며 실제 키는
  사용하지 않았다. 브라우저가 소비한 Rust 모듈은 기준 WASM이므로 조판 후보 검증과 구분한다.


### hwpspec 공동 앵커·후속 표 간격

- 2구역 문단 126의 첫 표는 본체 8,046HU + 양쪽 여백 282HU + 아래 캡션 1,802HU가
  빈 호스트 줄의 저장 위치 10,130HU를 정확히 닫는다. 첫 표만 남은 공간에 넣으면 표는
  22쪽에 남고 호스트·형제 표는 23쪽으로 갈라졌다. 첫 표 앞 공간을 포함한 저장 호스트 줄과
  앞 문단의 끝을 기존 `saved_positive_frame_reset`에 전달해 개체 방출 전에 쪽 소유를 확정한다.
  `is_textless_coanchored_float_host`의 같은 공동 앵커 경로가 배치를 계속 담당한다.
- 54쪽 문단 347은 나누지 않는 표이며 후속 빈 호스트 문단 348은 저장 위치가 되감기는 새 쪽이다.
  `host_spacing`의 기존 source reset 판정이 RowBreak만 허용해 347→348을 같은 쪽 표 묶음으로
  오인했다. 분할 정책과 후속 쪽 소유를 구분하고, pagination·paint가 같은 원본 되감김 조회를
  사용하도록 공유했다. 표 116의 앞 간격·후행 줄간격 중복이 제거됐다.
- 선행 원 해결 #8184의 `9ec584a15`와 기존 간격 판정 `1d6bc70767` 및 실제 소비 경로인
  `section::stored_reset` → `controls::flow_table` → 빈 호스트/공동 앵커 → `layout`을 대조했다.
  첫 표 상자와 뒤 호스트 줄의 소유를 바꿨으며 내용 숨김·좌표 clamp·문서 ID 분기는 추가하지 않았다.
- `legacy-v3` Native는 hwpspec 전체 178쪽 최저 92.45586%, 미달 0쪽이며 쪽수가 유지됐다.
  23쪽은 86.58353%→99.99389%, 54쪽은 89.58093%→99.99765%다.
  같은 경로를 사용하는 HWP5 공식 사양 71쪽도 최저 95.27413%를 유지했다.
  원문 공식 PDF는 변경하지 않았다. 최종 fresh WASM과 전체 회귀 검증은 계속 남았다.
- 로그: `output/pr-review/planet6897-20261026/hwpml-v3-all.log`, `hwp5spec-v3-all.log`.
  빌드 원본 diff와 고정 바이너리의 해시는 `legacy-v3-producer.json`에 기록했다.


### 중첩 표의 빈 줄 쪽 소유·세로 오프셋 1차 보정

- 같은 원본을 MCP 2020 Print로 다시 출력했다. 저장 제품 revision `7.5.12.754`는 2022 이전이며,
  job `cc627da2-d599-49f6-95d2-72f0693a4f08`의 실제 Hancom `11.0.0.9136`, Print method 0,
  `hancom2020_pdf_driver_one_up`, 17쪽을 확인했다. 기존 `-hwp-2020.pdf`의 Creator는
  `Hwp 2022 0.0.0.0`이므로 새 Print와 버전·출처를 구분한다. 파일명만으로 기존 출력을
  2020 제품으로 판정하지 않는다. 원본·기존 PDF는 변경하지 않았다.
- 원 해결 #3820 stage56과 #6697 및 #7445의 중첩 offset 계약을 대조했다. `CellUnit` 98(문단 73)의
  10.6667px 빈 줄은 저장된 새 쪽의 첫 슬롯이지만, 직전 TAC 표에 float placement가 없다는
  이유로 프레임 종료 인증이 거절돼 앞쪽 컷에 흡수됐다. 완전한 원본 개체 줄은 이미 저장 줄
  상자를 소유하므로, 기존 `owns_saved_object_line` 결과를 같은 body 종료 조회에 전달한다.
  인증된 빈 줄 경계는 재귀 투영에도 전달하며 일반 빈 overlay 리셋은 기존 계약을 유지한다.
- 부분 표 배치는 저장 줄이 있다는 사실만으로 세로 오프셋이 이미 소비됐다고 간주했다.
  실제 확정 float frame의 유무로 판단하고, 셀 끝 호스트의 첫 행 예약과 실제 배치가 같은
  `para_relative_float_table_lead`를 소비하도록 맞췄다. 2쪽 선언 offset 713HU를 복원했다.
- `legacy-v4` Native 17쪽은 쪽수를 유지했다. 새 Print 기준 2쪽 68.22392%→98.82900%,
  9쪽 49.65136%→94.27405%, 10쪽 61.16692%→90.26331%로 개선됐다.
  11·12·14·16쪽 미달은 아직 남아 있어 보정을 계속한다. 새 회귀의 기대값으로 고정하지 않는다.
- 로그: `output/pr-review/planet6897-20261026/nested-v4-all.log`, `nested-mcp-finish.log`,
  `nested-units-v2.log`. 고정 바이너리는 `candidate-v4-rhwp`, 빌드 출처는 `legacy-v4-producer.json`이다.

### 중첩 셀의 물리 끝 프레임과 재귀 유닛 후속 보정

- 선행 `3e9de474086707cf7c0b0ebdf60801a840d66859`의 `saved_nested_row_frames`는
  첫 물리 프레임은 `common.height`로 예약하지만 뒤 프레임은 글줄 높이만 사용했다.
  원본 셀 높이는 두 물리 조각의 합이므로 `cell.height - common.height`의 남은 상자를
  같은 예약·컷·부분 배치 조회에 전달한다. 남은 실제 내용이나 전진이 그 상자에 들어가지
  않으면 인증하지 않는다. 가운데 정렬의 빈 공간을 새 텍스트 유닛으로 세지 않는다.
- 작은 저장 호스트 줄은 큰 개체의 바깥여백을 대신 소비하지 않는다. 직접 HWPX에서 쓰던
  같은 원본 앵커·미편집·단일 개체 조건을 Native HWP5에도 적용해 원본 컷의 여백 예약과
  실제 원점이 같은 결과를 읽도록 한다. 첫 줄이 전체 개체 상자를 소유한 경우는 제외한다.
- `legacy-v5`는 17쪽을 유지하며 11쪽 62.77638%→90.18496%,
  16쪽 89.82594%→95.38000%다. 12·14쪽은 미달이고 15쪽은 92.10444%→82.35367%로
  하락했다. 이 후보를 수용하지 않으며, 다음 후보에서 재귀 자식의 마지막 예약 유닛을
  상위 cursor가 버리는 경로를 보정한다. scalar fallback의 합성 꼬리와 실제 원본 자식
  cursor의 빈 물리 밴드를 구분한다. 원본·PDF·기대값·공차는 변경하지 않는다.
- 로그: `output/pr-review/planet6897-20261026/legacy-v5-native-build.log`,
  `nested-v5-all.log`, `nested-fragment-alignment-trace.log`, `nested-p12-alignment-trace.log`.

### 재조판 그림·부동 글상자의 원본 control 줄 소유 조사

- #8168의 두 여러 줄 입력을 같은 원본에서 Hancom 2020으로 임시 HWPX 재저장했다.
  원본 생성 절차와 마지막 control의 원시 슬롯 위치를 대조했다. 원본을 대체하거나
  임시 저장본을 새 회귀 입력으로 사용하지 않는다.
- `above`의 실제 저장 줄은 세 번째 줄을 6,647HU→13,182HU로 옮긴다.
  차이 6,535HU는 offset 300 + 도형 높이 5,669 + 바깥 위·아래 여백 283씩과 같다.
  `below`의 offset 9,000HU에서는 같은 세 번째 줄이 6,647HU를 유지한다.
  두 입력의 `vertRelTo=PARA`, `horzRelTo=PAPER`와 도형 offset은 그대로다.
  배제 후 밀린 최종 글줄을 다시 개체 앵커로 쓰지 않고 배제 전 control 소속을 유지해야 한다.
- 실제 job은 `71dc25d1-fc74-4984-bf53-20512bd9b684`·
  `e08d5684-f3a9-4457-b533-350c7f932f88`이며 결과는
  `output/pr-review/planet6897-20261026/multiline-hancom-save-results.json`에 기록한다.
  이 두 저장본은 조사 자료이며 아직 구현·시각 검증을 마친 것으로 판정하지 않는다.
- `legacy-v6`는 마지막 재귀 예약 유닛을 보존해 12쪽을 57.03039%→99.24307%로 개선했다.
  전체 17쪽 중 15쪽은 90% 이상이며 14쪽 75.65324%·15쪽 82.35367%가 남는다.
  같은 원본의 자식 표는 첫 물리 높이 35,362HU, 셀 총높이 70,018HU를 저장한다.
  남은 선언 34,656HU보다 실제 종료 줄·안여백 점유 34,682HU가 커서 프레임 조회가 거절된다.
  첫 상자가 유효해도 마지막 상자가 내용의 최소 점유를 담지 못하면 끝 행은 성장해야 한다.
  다음 후보에서 선언된 잔여를 최소로, 실제 내용 점유를 성장 하한으로 함께 소비하도록 한다.
  이 26HU 차이를 별도 px 허용치로 면제하지 않는다. 로그는 `nested-v6-all.log`,
  `nested-v6-p14-detail.log`, `nested-v6-p15-detail.log`다.
