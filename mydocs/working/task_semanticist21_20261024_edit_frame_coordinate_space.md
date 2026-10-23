# 편집 후 저장 좌표계와 TAC 줄 프레임 소유 보정

Issue: #2185, #7114, #7336. 시작 head `ee4523938ea42d7c10675d53745e28b756a53018`,
base `d9fd85ed5bc3c4d9ce6b2981422e48bf9bb90867`.

## 선행 분석과 독립 근거

- #2185 원 해결 PR #2194, merge `cb51cdfccb1f3f3d2d33958c247766660d5a3e96`은
  한글 글자/어절 매핑을 정정했다. 현재 한 글자 입력은 네 줄 시작점·다음 문단 원점을
  보존하지만 전체 쪽수에서 실패한다. 줄바꿈 매핑을 다시 바꾸거나 기대값을 완화하지 않는다.
- #7114 원 해결 PR #7404, merge `08257e569c2a4614b453bcea7fb35771541fe8de`는
  실제 pagination cut을 저장 스냅숏의 LineSeg에 썼다. 통합 #7568의
  `731de9e1b4`와 현재 코드를 함께 읽었다. 실제 `RHWP_DIAG_WRITEBACK`에서
  live/render의 유닛 수·anchor가 모두 같다. 매핑이 다르다는 가정으로 저장기를
  다시 쓰지 않는다. source/provenance와 재열기의 예산 차이를 먼저 보정한다.
- #7336 원 해결 PR #7368, merge `b3e3d4e2170a43ca449e3d832440a9274e4e8ee4`는
  실제 소비 컷을 덮는 조각 상자와 큰 저장 표의 행 분할을 고쳤다. 원본 두 문서의
  최종 Native/fresh 전쪽 증거는 원 PR 메인터너 보정과 #7902의 `84764fa21c`에 있다.
  현재 동의서 표 검사는 높이 오차가 아니라 앞 표가 한 쪽 늘어 노드를 못 찾아 실패한다.

원문과 timeline은 `remaining-origins-2584.json`·`2585.json`, 세 원 PR은
`pr2194-origin-2587.json`·`pr7404-origin-2587.json`·`pr7368-origin-2587.json`에
보존했다. 설치된 gh가 `--slurp`를 지원하지 않아 첫 timeline 요청은 실패했다.
기존 실패 로그를 보존하고, 페이지별 JSON 배열을 직접 읽는 수정 요청으로 완료했다.

## 같은 공개 API의 실제 관측

`remaining-edit-snapshots-2586-result.json`은 동결 Native165에 공개 즉시/지연 편집과
저장 API를 한 번 링크해 실행한 결과다. 0·1·55·56·61·200자 및 HWP/HWPX 한 글자
편집을 함께 기록했다. 한 글자 편집은 LineSeg가 원래 네 줄 그대로인데 live 116쪽이다.
55/56/61자는 live 116쪽·재열기 117쪽이며, 200자는 둘 다 116쪽이지만 104쪽 소속이 다르다.
무편집 저장은 115쪽·본문 차이 0이다. live IR의 필드는 저장 전후 변하지 않는다.

단계별 실제 원점·컷·물리 예산은 `remaining-edit-trace-2588-result.json`과
`remaining-edit-first-frames-2591.json`, 저장 매핑은
`logs/remaining-edit-trace-2592.log`에 있다. 같은 root의 글줄 예산이 편집 여부만으로
1001.600→1000.267px가 되고, 개체를 수용하는 조각은 1005.373px의 기존 예약 대신
같은 1000.267px를 적용한다. 이 숫자를 구현 조건으로 쓰지 않는다.

`mcp-edited-controls-2590/results.json`은 위 API가 실제 저장한 동일 SHA의 파일을
PrintMethod 0으로 출력했다. 무편집·56자·200자·지연 한 글자 HWP는 한컴 2024,
지연 한 글자 HWPX는 한컴 2020이며 전부 115쪽이다. 버전은 각 저장본 `info`의
lastSavedWith와 원본 제품 세대로 확인했다. 빈 HWP 스크립트 placeholder는 알려진
해시를 확인했고 HWPX에는 스크립트가 없다. 기존 56자 PDF를 새 저장본의 출력으로
대체하지 않았다. 입력·PDF SHA와 실제 job은 해당 receipt에 보존한다.

## 적용할 공통 규칙과 소비 경로

1. 저장된 조각 좌표계를 쓴다는 사실과 옛 컷을 그대로 수용할 수 있다는 사실을 구분한다.
   다른 문단의 편집만으로 남은 저장 좌표계까지 단조 재생성 사다리로 바꾸지 않는다.
   원본 줄·개체 프레임의 기하가 남아 있는지는 그 자료에서 확인하며, 기존 저장 컷의
   유효성을 판정하는 provenance 검사는 유지한다.
2. 프레임 근거는 두 갈래다. 원본 부모 줄의 실제 되감김은 페이지 상대 좌표계를 보이고,
   유효한 저장 줄과 선언 개체보다 큰 원본 행 상자의 합은 셀 상대의 분할 저장 프레임을
   보인다. 원본 거대 셀의 선언 셀 높이는 패딩뿐이므로 후자 하나를 필수 전제로 삼지 않는다.
   단조 분할 저장본은 둘 다 없으므로 기존 바깥 물리 조각 예산을 유지한다.
3. 일반 유닛처럼 표현된 TAC 줄도 실제로 개체의 줄 상자를 소유할 수 있다. #7336의
   마지막 문단은 `9395+141+141=9677HU`로 원본 LineSeg와 같다. 제어문 슬롯이
   현재 컷에 속하고 같은 소유 기하를 확인한 줄은 개체 viewport의 예약을 소비한다.
   빈 일반 글줄이나 다른 쪽의 TAC 존재로 현재 예산을 바꾸지 않는다.

생산은 원본 LineSeg·행/개체 기하 → 저장 좌표계 종류와 실제 TAC 소유 줄 판정이다.
소비는 `rowbreak_fragment_bounds`의 내용/빈 띠 하단과 `row_step`의 현재 컷 예산이다.
실제 컷·요구 높이·paint는 기존 공통 CellUnit 원장을 계속 소비한다. 저장 좌표계의
종류를 확인한 것을 옛 강제 컷의 새 인증으로 쓰지 않는다.

## 검증 계획

Native166을 공용 target·8잡으로 한 번 빌드한다. 기존 189개 검사, 편집 전 HWP/HWPX
230쪽, 고정 분할본 196쪽, 35문서/66쪽과 #7840·현대 수식을 함께 대조한다.
같은 5개 편집 저장본도 해당 새 Print와 확인한다. 현재 API의 저장 바이트가 바뀌면
그 출력은 별도 MCP Print를 확보한다. 새 회귀·기대값·fixture는 시각 선행 조건을
충족하기 전에는 추가·변경하지 않는다. 전체 Rust와 fresh WASM 완료를 이 선행 확인으로
대신하지 않으며, 실패나 미달이 남으면 원인을 계속 보정한다.

## Native166 중간 검증 결과

- 빌드: `char-overlap-native-build-166-result.json`, 8잡·공용 target, exit 0.
  기본 Native Clippy도 `logs/native166-clippy-2603.log`에서 exit 0을 확인했다.
- 기존 189개: `reviewed-contract-tests-2594-result.json`의 **184 PASS / 4 FAIL / 1 ignored**.
  #2185 한 글자 편집과 #7336 두 검사가 다시 통과했고 이전 실제 정상 181개와 제외 1개 상태를 유지했다.
  남은 네 건은 #7114의 55/56/61/200자 저장·재열기 차이다. 검사를 바꾸지 않았다.
- 원본 HWP/HWPX: `issue1949-exact-originals-native166-2595-result.json`, 각각
  115/115쪽·최저 95.69614%. 두 경로 모두 Native이며 fresh WASM을 대신하지 않는다.
- #7336: `issue7336-native166-2602-result.json`, HWP 6/6쪽 최저 97.07162%,
  HWPX 7/7쪽 최저 96.87735%. 기존 한컴 2020 Print를 같은 입력 SHA로 재사용했다.
- 고정 분할 저장본: `issue4138-native166-boundary-batch-2596-result.json`,
  196/196쪽·최저 38.91518%. 177~196쪽의 기존 미달은 남아 있으며 통과가 아니다.
  #7840 9쪽 최저 93.43362%, 현대 수식 v5 5쪽 99.98497%, v6 2쪽 100%를 유지했다.
- 35문서/66쪽 정책·원점·괘선 대조군: `reflow-*-native166-2597/results.json`,
  6개 driver exit 0, 각 쪽 raw 100%와 쪽수 일치를 확인했다.
- 공개 편집 API: `remaining-edit-snapshots-2598-result.json`. 무편집·즉시/지연
  한 글자 HWP 및 지연 한 글자 HWPX는 115쪽과 쪽별 본문이 저장·재열기 전후 같다.
  55/56/61자는 live 115·재열기 116쪽, 200자는 live 116·재열기 117쪽이다.
- 새 저장본의 독립 Print: `mcp-edited-controls-2600/results.json`. 실제 Native166
  56/200자·지연 한 글자 HWP 바이트를 한컴 2024 PrintMethod 0으로 출력했다.
  이 PDF를 이전 Native165 저장본의 기준으로 바꾸어 보고하지 않는다.

현재 source의 전쪽 결과와 정책 대조군은 개선·보존됐지만 저장 왕복 네 실패와 분할본 미달,
최종 전체 Rust·lint 묶음·fresh WASM은 남았다. PR 준비 완료나 시각 승인으로 판정하지 않는다.

## 다음 보정의 확인된 경계

`edited-unit-differences-2608.json`과 `logs/remaining-edit-trace-2601.log`에서
56자 편집의 live/reload는 유닛 수와 각 높이가 같지만 저장 원점과 종료 줄 소유가 다르다.
3쪽 컷 115 앞뒤의 빈 문단 59는 원본 다음 프레임(문단 60) 바로 앞의 종료 줄이다.
live의 다음 조각에서는 무높이로 지나가나, 재열기는 `owns_stored_closing_line`으로
다시 소유해 같은 컷의 요구가 한 줄(25.6px) 커지고 추가 쪽이 생긴다.
종료 줄 자체의 앞 조각 점유는 보존하고, 다음 조각의 시작에서는 앞 프레임 소유 줄을
중복 계상하지 않아야 한다. 원래 값이나 테스트 허용치를 변경할 근거가 아니다.

## Native167: 종료 빈 줄의 앞 조각 소유

`is_free_leading_spacer`의 공통 술어에서 `owns_stored_closing_line` 유닛은 다음
조각의 시작 전진으로 다시 예약하지 않는다. 앞 조각 안의 같은 유닛 높이는 그대로다.
컷 선택·물리 범위 측정·저장 시작 컷은 기존 공통 술어를 함께 소비한다.

- `char-overlap-native-build-167-result.json`: 8잡·공용 target 빌드 exit 0, 247.36초.
- `reviewed-contract-tests-2614-result.json`: 기존 **189개 중 187 PASS / 1 FAIL / 1 ignored**.
  55/56/61자의 세 실패가 통과했고 기존 실제 정상 184개 검사도 유지했다. 새 검사나 기대값 변경은 없다.
  남은 실패는 200자 편집의 중첩 표 내부 컷 뒤 저장·재열기 내용 소속이다.
- `remaining-edit-snapshots-2615-result.json`: 55/56/61자 모두 live/reload 115쪽과
  쪽별 본문이 같다. 200자는 live 116·재열기 117쪽, 첫 내용 차이는 103쪽이다.
  무편집·즉시/지연 한 글자 HWP·지연 한 글자 HWPX도 115쪽과 본문을 보존한다.
- 8개 API 저장 바이트는 Native166과 모두 SHA-256이 같다. 읽기·이어받기 처리가 바뀐 것이며
  새 저장 바이트를 Print했다고 보고하지 않는다. 같은 SHA의 `mcp-edited-controls-2600`
  Print를 재사용한다. `edited-print-pagecounts-2625.json`의 실제 PDF는 세 파일 모두 115쪽이다.
- `issue1949-exact-originals-native167-2616-result.json`: 원본 HWP/HWPX 각 115쪽,
  최저 95.69614% 유지. `issue4138-native167-boundary-batch-2617-result.json`의 고정
  분할본 196쪽은 최저 38.91518%로 미달 유지, #7840·현대 수식 대조군도 이전 값을 유지했다.
- `logs/native167-clippy-2622.log`: 기본 Native Clippy exit 0. 전체 Rust·WASM lint·
  workspace lint·fresh WASM 완료를 뜻하지 않는다.

### 쪽수·본문 왕복 통과와 독립 Print의 시각 미달

`edited-print-native167-2623-result.json`은 같은 API 저장본·같은 SHA의 독립 Print로
전쪽을 확인했다. 56자 저장본은 115/115쪽이지만 **102쪽 미달·최저 11.38223%**이며,
한 글자 저장본은 **8·104·105쪽 미달·최저 19.93037%**다. 200자 저장본은 쪽수가
다르므로 TSV를 산출하지 못했다. 두 측정 가능 문서의 gate는 `re_review_required`이며,
쪽수 불일치도 통과가 아니다. 관계 검사 통과를 독립 출력 일치로 승격하지 않는다.

새 공식 PNG는 `native166-7336-reviews-2612`의 #7336 6쪽 review/overlay와
`native166-edited-reviews-2618`의 한 글자 저장본 105쪽 review/overlay다. 후자는
그림이 다른 쪽에 놓이고 본문 원점도 크게 달라 직접 판독에서도 실패했다.
`untouched-one-character-print-boundaries-2626.json`에서 원본·무편집 저장본·한 글자
저장본의 실제 한컴 Print 104/105쪽 이미지 bbox는 같다. 반면
`one-character-all-source-lines-2621.json`에서 rhwp 저장기는 편집하지 않은 부모 문단
54개의 LineSeg 원점을 바꿨다. 빈 시작 문단 158과 TAC 그림 호스트 2290의 원본 재시작이
그 뒤 문단으로 옮겨졌다. 단순히 페이지 수·낱말만 비교하면 이 배치 차이를 검출하지 못한다.

다음 분석은 줄 기하가 같은 편집까지 전체 표의 저장 프레임 되쓰기를 요구하는 출처 표식과,
부모 사다리의 실제 쪽 경계·개체 로컬 원점의 구분이다. 원본 줄이 남아 있는데 다른 호스트의
생성 tag 하나 때문에 셀 전체의 저장 좌표계를 기각하는 조건도 200자 재열기와 함께 확인한다.
같은 높이만으로 불변을 판정하지 않고 줄별 원점·높이·간격·소유 변화를 함께 대조한다.

`native167-focused-summary-2628.json`에서 전체 producer source hash 일치, 8개 저장 바이트
불변, 기존 실제 187 PASS / 1 FAIL / 1 ignored와 35문서/66쪽 raw 100%를 묶어 확인했다.
정식 회귀 추가·변경과 PR 제출의 시각 선행 조건은 충족하지 않았으며 보정을 계속한다.

## 다음 보정: 실제 줄 기하 변경과 저장 좌표계의 국소 유효성

현재 head `466c57122dd868d5bae9eb55bdc59320c3c663a9`에서 다음 두 생산 지점을 함께 확인한다.

1. 셀 텍스트 삽입/삭제는 실제 줄 기하가 그대로여도 `mark_table_text_reflowed_after_edit`를
   무조건 기록한다. 저장기는 이 표식을 보고 전체 표의 원본 사다리를 되쓴다.
   한 글자 입력의 원본 줄별 원점·높이·간격은 그대로인데 편집하지 않은 54개 문단의
   원점을 바꿨고, 독립 Print는 원본·무편집·한 글자 저장본의 그림 위치가 같다고 확인했다.
   삽입/삭제가 같은 원본 수직 줄 프레임과 소유 분류를 보존하면 새 프레임 무효화 표식을
   추가하지 않는다. 기존에 실제 재조판으로 기록한 표식은 지우지 않는다.
   전체 높이 하나 대신 줄별 원점·높이·간격, 원본 줄의 유효성, 빈 줄·제어 소유를 함께 비교한다.
2. 저장 좌표계의 종류를 찾는 판정은 지금 셀의 모든 문단이 원본이어야 되감김 하나도
   인정한다. 200자 저장본의 호스트 81에 생성 tag가 붙어도 다른 원본 줄들의 페이지 상대
   좌표계는 남아 있다. 생성·dirty 줄은 근거에서 제외하되, 연속한 실제 원본 줄 쌍의
   되감김은 그대로 인정한다. 이것을 옛 컷 전체의 재인증으로 사용하지 않는다.

공개 편집 8종·기존 189개, 원본 230쪽과 고정 분할본 196쪽을 같은 Native168 빌드로
일괄 확인한다. 35문서/66쪽·#7336·#7840·현대 수식의 기존 정상 경계도 유지한다.
새 저장 바이트는 기존 Print와 SHA를 먼저 대조하고, 달라졌으면 같은 제품 세대의
MCP Print를 새로 확보한다. 쪽수·본문 왕복만으로 시각 검증이나 새 회귀 자격을 판정하지 않는다.

## 제외 검사의 집계 정정

`focused-terminal-audit-2635.json`에서 Native165/166/167의 189개 원 로그를 전수
재독했다. 진단 드라이버는 test binary의 exit 0만으로 PASS를 집계해 기존 ignored 검사
`issue_2308_short_rowbreak_child_wraps_where_the_authority_pdf_wraps` 1개를 잘못 포함했다.
정확한 Native165/166/167 결과는 각각 181/7/1, 184/4/1, 187/1/1
(PASS/FAIL/ignored)이다. 기존 JSON·로그는 원 증거로 보존하고 이 감사를 연결한다.
수정한 다음 진단 드라이버는 실제 실행 수와 ignored 수를 따로 보고한다.
이 검사를 활성 회귀나 실행 성공 증거로 승격하지 않는다.

Native168의 실제 집중 검사는 188 PASS / 0 FAIL / 1 ignored, 공개 API 14종의
저장·재열기 쪽별 본문은 모두 같다. 200자는 여전히 116쪽이므로 독립 Print의 115쪽과
다르며, 시각 통과가 아니다. 다음 Native169에서는 원본 쌍을 찾는 도중 저장 줄이 없는
문단을 만나도 이전 줄과의 연결을 끊는다. 생성·dirty 문단뿐 아니라 정보가 없는 문단을
가로질러 가짜 원본 쌍을 만들지 않는 같은 판정의 보수적인 경계다.

## Native168의 실제 실행과 독립 Print 개선

`native168-focused-summary-2650.json`은 실제 기존 188 PASS / 0 FAIL / 1 ignored,
공개 API 14종의 저장·재열기 쪽별 본문 일치, 35문서/66쪽 raw 100%를 기록한다.
직접 삭제와 짧은/줄바꿈 삽입 후 삭제를 HWP/HWPX에서 실제 실행했다.

한 글자 HWP 저장본 SHA는 `9e76b3b462e5c6f5e4adade2356f6f7dca6cb513be1c9fb6327dccec902a0107`로
바뀌어 이전 PDF를 재사용하지 않았다. 한컴 2024 MCP job
`bd1ccbc8-a5ab-4862-9445-4b8cda06f6aa`의 PrintMethod 0 출력은
`mcp-edited-controls-2637/results.json`에 원본·PDF SHA와 함께 있다.
`edited-print-native168-2638-result.json`의 Native 115/115쪽 최저는 95.69614%,
90% 미만 0쪽이다. `one-character-preserved-vertical-frames-2642.json`에서
원본 부모 문단 2,507개의 원점·높이·줄간격 차이도 0개다.

공식 `native168-one-reviews-2643`의 105쪽 review와 standalone overlay를 직접 읽었다.
이전 19.93037%에서 raw 100%로 개선됐고 그림과 뒤 제목·본문의 같은 쪽 소속,
상하 순서와 원점이 Print와 일치한다. `not_evaluated` 실루엣 보조 gate를 최종 승인으로
바꾸어 보고하지 않으며 fresh WASM과 확대 판독 범위의 최종 증거는 아직 남아 있다.

200자는 116쪽으로 저장·재열기는 같지만 독립 Print의 115쪽과 다르다.
56자 편집의 전쪽 시각 미달과 고정 분할본 177~196쪽 미달도 해결 완료가 아니다.
다음 전체 Rust 재실행과 별개로 이 물리 배치 차이를 계속 보정한다.

## Native169 결과

`native169-focused-summary-2654.json`에서 producer source hash를 재확인했다.

- 기존 실제 188 PASS / 0 FAIL / 1 ignored, 35문서/66쪽 전쪽 raw 100% 유지.
- 공개 API 14종은 저장·재열기 본문이 같고, Native168과 각 저장 바이트 SHA도 같다.
  새 저장 바이트를 별도 출력한 것처럼 보고하지 않고 동일 입력 Print를 재사용한다.
- 원본 HWP/HWPX 각 115쪽·최저 95.69614%, 고정 분할본 196쪽·최저 38.91518%로
  기존 뒤 20쪽 미달 유지. #7840와 현대 수식 정상 대조군도 유지했다.
- 전체 format과 Native root Clippy는 각각 exit 0,
  `logs/native169-format-2651.log`·`logs/native169-clippy-2652.log`에 보존했다.
- 다음 전체 Rust 로그는 `current-full-rust-2644/nextest.log`다. 전체 결과 전에는
  과거 68 FAIL을 새 전체 통과로 바꾸지 않으며, fresh WASM 등 나머지 필수 gate도 남아 있다.
