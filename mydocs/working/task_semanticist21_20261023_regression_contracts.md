# semanticist21 통합 검토: 검증된 입력의 회귀 계약 보정

## 범위와 엔진

앞 단계는 [검증 환경·Studio 글리프 보고서](task_semanticist21_20261023_regression_environment.md)와
커밋 `4ed947b5bb9bf8bde0ef43243e5bab1961283198`에 고정했다. 이 단계는 검증한 원본에
등록된 기존 실패의 기대값·선택기를 재검토하며 새 PR 체리픽이나 조판 구현 변경은 없다.

동결 Native142 library/CLI와 fresh WASM048의 엔진 source는
`8c32167d3d87f41beb5c6b045615f4f4e7df112d`다. 현재와 `src/`, `Cargo.toml`, `Cargo.lock`,
`build.rs`의 차이가 없고 library SHA가 영수증과 같은지 확인한 뒤 사용했다. Rust 진단은 공유
`target/pr-review/release-test`에서 순차 컴파일했다. 정식 전체 Cargo·lint·최종 PR source의
검증을 진단으로 대체하지 않는다. 임시 실행물은 ignored
`output/pr-review/semanticist21-20261019/`에만 남긴다. 아래 경로의 `output/` 접두어는 생략한다.

## #5790: 실제 글자겹침의 SVG 내용 검사

원 해결 PR #5814와 통합 기록을 대조했다. 실제 입력은
`samples/issue1880_takeplace_oracle_p13.hwpx`이며 원래 검사는 과거 SVG의
`text-anchor="middle" dominant-baseline="central">`라는 속성·순서로 글자겹침을 찾았다.
현재 canonical SVG는 같은 원문자 ③·④를 그리지만 이 마커를 쓰지 않아 빈 배열로 실패했다.
본문의 다른 원문자가 누락을 가릴 수 있으므로 전역 문자열 검색으로 바꾸지 않았다.

실제 paint의 CharOverlap 소유 leaf, 원래 source ID·paired fallback·clip·layer·resource를
유지해 SVG를 재생한다. 제거한 본문의 text source table 항목도 함께 동기화하며 canonical
source 검증을 우회하지 않는다. 실제 SVG XML에서 두 글리프의 보존과 맨 숫자로의 변환 금지를
검사한다. 최초 진단에서는 source table을 남겨 cardinality 오류로 실패했으며, 해당 로그를
보존하고 table 동기화를 보완했다. 테스트를 통과시키기 위해 검증기를 끄지 않았다.

같은 원본의 MCP 2020 Print는 11.0.0.9136·PrintMethod 0이다. Native142는 전 13쪽 최저
96.10356%, fresh WASM048은 96.10629%로 쪽수도 일치한다. 5쪽의 ③·④와 주변 본문을
Native/fresh review·overlay에서 직접 확인했다. [정식 PDF·SHA·TSV·PNG 목록](../pr/assets/issue_5790_charoverlap/README.md)에
원본을 연결했고 PDF와 두 TSV·대표 PNG를 바이트 그대로 보존했다. 절대 px 핀을 추가하지 않았다.

## #7819: 공개 선택 JSON과 payload 독립 슬롯 폭

원 PR #7819의 `7ded7ca1`·`a3f80b14` 및 현재 반영 커밋을 대조했다. 핵심 계약은 글자겹침
하나가 논리 한 슬롯이고, 원래 제어·본문 순서와 주석 소비·끝 캐럿을 보존하는 것이다.

- `non_bmp_…boundaries`: 공개 선택 JSON은 `cursor_nav.rs`에서 x와 width를 각각 소수 첫째
  자리로 직렬화한다. 반올림한 두 값의 합을 연속 좌표의 오른쪽 끝과 비교해 오차를 중복
  계상하던 검사를 바꿨다. 같은 줄의 연속 선택이 한 사각형인지, x와 width가 각각 실제
  시작 글리프·마지막 본문 끝 관계의 공개 반올림 값인지 정확히 검사한다. 허용치를 키우지 않았다.
- `same_anchor_…geometry`: 원래 `1000/75` px를 고정하던 폭 검사를 같은 글꼴·글자모양의
  한 payload 대조군과 비교하도록 바꿨다. 실제 font advance를 13px 같은 관측값으로 복사하지
  않았다. payload 수·각주 순서·정렬과 무관한 슬롯 폭, 제어 소속·본문·캐럿 관계를 유지했다.

수정 전 원 실패 API로 저장한 20개와 한 payload 대조군 2개를 MCP 2020에서 각각 Print했다.
모두 전쪽 Native/fresh WASM raw 100%, 전체 쪽수 1쪽 일치다. Native/fresh 원본 SHA와
페이지 범위·명령은 `caret-original-current-api-native142-2191`,
`caret-original-current-api-wasm048-2206`, `single-payload-native142-2217`,
`single-payload-wasm048-2237`의 결과에 있다. `caret-contract-reviews-2238`과
`caret-contract-detail-2242`에서 원문·주석·뒤 본문의 소속과 순서, 표시 영역을 직접 대조했다.
HWPX의 큰 내부 글자는 같은 원본의 한컴 Print에도 있다. 서로 다른 형식의 동일성을 주장하거나
원본 크기를 임의로 바꾸지 않았다.

입력 22개·PDF 22개·Native/fresh TSV 44개를 정식 경로로 복사한 뒤 모든 SHA와 raw 원문을
다시 확인했다. [입력 설명](../../tests/fixtures/char_overlap_caret_axis/README.md)과
[증적 목록](../pr/assets/issue_7819_caret_axis/README.md)을 따른다. 네 `nonbmp` 입력의
완료된 첫 쪽에만 독립 Print glyph 원점에서 정한 ±2px 캐럿 경계 검사를 새 이름으로 추가했다.
좌표계·DPI·SHA·페이지·TSV·확대 자료와 본문 끝 산출 근거를 입력 설명에 기록했다.
기존 소유·내용 보존·순서·hit-test·선택 관계 검사도 유지한다.

## #6520: additive schema의 기능 최소 버전

원 PR #6668 → 통합 #6694, `b6b9384ed9e679a8d143c6dcbaacef502614eb9e`의
`src/paint/schema.rs`에서 controlLabel의 도입 버전 1.23을 확인했다. 현재 정확한 minor 34는
schema 자체 단위 검사가 보호한다. control-label 검사는 major 1과 minor ≥23, 실제 7개 표식,
used/known feature, SVG 및 Skia replay를 검사한다. 과거 전역 minor 29를 새 전역 34로
단순 교체하지 않았다. 이는 합성 transport·feature 계약이며 한컴 조판 일치의 증거가 아니다.

## 실행 결과와 남은 검증

- 세 모듈 전체: `reviewed-contract-tests-2244-result.json`, **38 PASS / 0 FAIL**.
  `logs/reviewed-contract-tests-2244-driver.log`, `…-build.log` 및 테스트별 로그에 명령과
  정확한 source SHA·컴파일 옵션·종료 코드를 보존했다. 추가한 ±2px 검사도 실제 실행했다.
- 기존 외부 실패 목록 전체: `current-previous-failures-2245-result.json`,
  **9 PASS / 38 FAIL / 누락 0**. 직전 같은 Native142의 5 PASS / 42 FAIL과 비교해 위 네
  잘못된 기대값·선택기 실패를 해결했다. 다른 미달 입력을 통과했다고 보고하지 않는다.
- #4138 현재 API 저장본은 새 한컴 2024 Print와 Native의 쪽수가 모두 196이지만 전 196쪽
  중 195쪽이 90% 미만이다. compat 2022와 2024 모두 최저 17.92141%이며,
  `issue4138-native142-compat2024-2234/results.json`과 2쪽 review·overlay에서 내용 컷과
  한 줄 이상의 세로 경계 차이를 확인했다. 197쪽 핀을 196쪽으로 갱신하지 않았다.
  원인 보정과 양쪽 재검증은 계속 필요하다.

최종 전체 Rust·lint·feature 보존·시각 검증 및 PR 제출·CI·merge·후속 처리는 아직 완료하지
않았다. 활성 작업의 미완료 항목으로 계속 처리하며 이 상태로 PR을 제출하지 않는다.
