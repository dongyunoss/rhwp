# 회귀 검사 등록 환경과 저장 프레임 순회 보정

## 범위와 시작 기준

작업 브랜치 `review/semanticist21-20261019`, 시작 커밋 `32f534d4d5`이며
base는 `605ccfd92f2f12f18e14182548d729f9929fda00`다. 기존 45개 PR 범위를
유지하고 새 PR을 체리픽하지 않는다. 행정업무편람의 추가 개선과 다른 PR이 수정 중인
두 사양 문서는 제외한다. TABLE 값 2/CELL은 셀 내부 글줄도 나눌 수 있는 의미로
참고하며, 내부 `RowBreak` 이름을 행 경계 전용 정책으로 해석하지 않는다.

## 등록 환경 검토 방법

1924 전체 실행의 단언 실패 39건은 테스트 함수의 등록 커밋·원 체리픽·현재 소스
조건을 먼저 수집했다. `regression-registration-audit-1948.json`에 등록 이력과 현재
실행 명령, 변경된 Rust 테스트 파일 80개를 기록했다. 이는 목록 수집이며 전체 검사의
등록 적합성 확인 완료를 뜻하지 않는다. `src` 내부 단위 검사와 조건부 Native 검사도
해당 실행 경로에 따라 구분한다.

검사별로 원 해결 PR → 실제 통합·보정 커밋 → 독립 출력·실행 근거를 연결한다.
원본/PDF 해시·Print 출처·compat·DPI·프로필·글꼴 공급·feature와 Native/fresh WASM
전체 쪽수 및 raw 최저값을 대조한다. 절대 좌표 검사는 해당 페이지의 양쪽 98% 이상과
직접 판독 근거도 확인한다. 현재 실패만으로 등록 오류를 선언하거나 기대값을 바꾸지 않는다.

## 현재 확인한 사례

| 검사 | 등록 근거·실행 조건 대조 | 현재 판단 |
| --- | --- | --- |
| 80168의 완료 93쪽 | 함수 등록 `ab965e65e0`, 통합 PR #8148. 원본·한컴 2024 Print 해시 일치. 제출 TSV 원본은 Native/fresh 157쪽, 최저 93.90078/91.66667%, 93쪽 99.99223/99.96152%. Print·compat 2024·96 DPI와 글꼴 경로·producer 해시가 명시돼 있다. | 등록 시각 근거를 확인했다. 현재 컷 회귀는 구현에서 보정하며 독립 기대값을 유지한다. |
| 글자겹침의 non-BMP·같은 앵커 2건 | 등록 `c240d29285`, 원 `a3f80b14ad`, 원 PR #7819. 원 PR은 Native 테스트 통과와 별도로 독립 Print·전체 Native/fresh 시각 근거의 부재를 명시한 Draft다. | 당시 통과 주장만으로 시각 선행 조건 충족을 인정하지 않는다. 대응 입력의 독립 검증과 실제 동작 보정을 계속한다. |
| 같은 글자겹침 2건의 현재 환경 | 전체 실행은 Cargo 기본 feature다. 같은 테스트 소스를 동결 Native125 라이브러리와 `native-skia`로 별도 컴파일해도 두 검사 모두 같은 단언으로 실패했다. | 기본 feature와 Native Skia의 차이만으로 설명할 수 없다. 기존 검사를 숨기거나 feature로 우회하지 않는다. |
| 시장구조조사의 저장 되감김 | 등록 `04404a07be`, 원 `8c9f9e925a`. 당시 기록의 원본·315쪽 Print·본문 되감김 근거와 현재 환경을 대조 중이다. 현재 장시간 경로는 별도 스택으로 확인했다. | 시각·등록 환경 확인을 계속하며 성능 문제를 먼저 보정한다. |

자료는 ignored `output/pr-review/semanticist21-20261019/`의 1931~1948 실행 기록에 있다.
1944 환경 진단은 rustc 인자 구성 오류로 빌드 실패했다. 1945에서 명령을 수정해
실제 컴파일·두 검사를 완료했다. 실패한 진단을 통과 증거로 재분류하지 않는다.

## 성능 보정의 독립 근거와 계획

`native_stored_table_has_intra_paragraph_frames`의 도입은 현재 브랜치의
`2d6f00d4cf`, 선행 행 인증 추가는 `6e0aab2090`이다. 현재 구현은 실제 원본 문단의
재시작 여부를 확인하기 전에 셀별 선행 행 합과 `cell_units`를 계산한다. 1939의
gdb 자식 실행에서 이 두 경로를 확인했고, 1940의 CFB 원문 읽기에서 시장구조조사의
큰 표는 2,195/2,196행·8열임을 확인했다. 전체 테스트의 CPU 사용만으로 원인을
추정한 것이 아니다.

최종 참 판정에는 같은 원본 문단의 인접 저장 글줄이 양수 원점에서 0으로 되돌아가는
경계가 필요하다. 그 조건이 없는 셀은 기존 구현에서도 반드시 false다. 동일한 원본
조건을 먼저 검사해 불필요한 선행 행 순회와 재조판을 건너뛴다. 실제 후보의 기존
유닛·원본 높이·물리 프레임 인증은 유지한다. 문서 이름·행 수 임계값·시간 제한으로
결과를 우회하지 않는다.

보정 후 같은 시장구조조사와 원본 저장 프레임 대조군을 실행하고, 반환 결과·본문
소속과 실행 시간을 확인한다. 큰 표를 처리할 수 있게 된 후보에서 전체 Rust를
재실행한다. 80168의 93쪽 및 부분 배치 미달 보정, lint·fresh WASM과 제출 전
등록 환경 검토도 완료해야 하며 현재는 PR 준비 완료가 아니다.

## 작업지시자의 하락폭 허용 조건 변경

2026-10-23 사용자 답변에 따라 고정한 `upstream/devel` 대비 페이지별 raw 하락폭을
기존 0.1%p 재검토 기준에서 **2%p까지 허용, 초과 시 재검토**로 변경했다. 상대 2%가
아니며 Native/fresh WASM을 각각 같은 조건·페이지로 비교한다. 관련 활성 매뉴얼
7개를 함께 수정했다. 앞으로의 비교는 Decimal로 `기준 − 후보 > 2`를 판정한다.
기존 실행 로그·TSV·결과 JSON의 과거 판정은 덮어쓰지 않는다. 과거 비교 스크립트를
재사용할 때는 이전 0.1%p 조건을 그대로 실행하지 않고 새 실행 명령과 판정을 기록한다.
전쪽 90% 하한, 완료 페이지의 양쪽 98% 보호, 쪽수·내용 보존·직접 확대 판독 및
정확한 회귀 테스트의 기대값은 변경하지 않는다.

## 성능 보정의 실행 결과

Native126은 `32f534d4d5`와 해당 저장 프레임 스캐너 수정만 포함한 고정 소스로
빌드했고 245.21초, exit 0을 확인했다. source hash 확인과 producer 보존은
`char-overlap-native-build-126-result.json`에 있다. 이후 지침 전용 커밋
`15acb82f84`가 추가됐으며 제품 코드는 같다.

1950 실행은 기존 검사 소스를 변경하지 않고 같은 동결 라이브러리의 Native Skia
조건으로 컴파일했다. 시장구조조사 검사 1건은 21.22초에 통과했다. 이전 1924
기본 feature 실행에서는 1,100초 이상 실행 중에 진단을 거쳐 중단했으므로 정확한
전후 배율이나 기본 feature 전체 통과로 보고하지 않는다. 후속으로 기존 저장 원점·
TAC 소유·도형 호스트·셀 글줄 중첩 프레임 검사까지 총 13건이 통과했다. 새 기대값이나
formal 회귀를 추가한 결과가 아니다. 명령·소스 해시·각 검사 시간과 로그는
`current-previous-failures-1950-result.json`에 기록했다.

이 결과는 저장 프레임 순회 보정의 국소 검증이다. 미달인 80168의 93쪽과 중첩 표
부분 배치, 남은 기존 실패의 해결·등록 환경 대조 및 최종 전체 검증을 대체하지 않는다.

## 80168의 첫 조각 이월 판정 보정 계획

1952의 Native126 원본 실행에서 pi=784는 현재 높이 950.8px, 첫 조각 가용 높이
16.7px인데 `min_content=9.1`로 현재 쪽을 유지했다. 실제 첫 CellUnit과 패딩을
소비한 스캔은 30.5px를 요구했고 그 첫 행을 강제로 수용했다. 다음 쪽에는 `[1]`
이후만 배치돼 독립 Print의 전체 표와 후속 제목이 위로 당겨진다. 기존 93쪽 검사의
독립 기대값을 바꿀 문제가 아니다.

생산은 `cell_units`의 원본 유닛, 측정·예산 소비는 `row_cut_content_height`, 실제
부분 배치는 같은 컷의 `table_partial`이다. 현재 사전 이월만 `MeasuredCell`의
빈 호스트 첫 글줄 5.3px와 패딩을 사용해 다른 최소값을 만들었다. 저장 줄 없는
블록 자식의 재조판 프레임이 확정된 행은 첫 원본 유닛을 선택한 컷의 실제 높이를
사전 이월에도 소비하도록 보정한다. 일반 저장 글줄·걸침 행의 기존 계약은 유지한다.
형상·문서 번호·좌표를 기준으로 하지 않으며, 자식 원장을 물리 테두리 높이로
덮어쓰지 않는다. 92·93쪽 및 자식이 실제로 이어지는 3행 입력을 함께 대조한다.

## 누름틀 SVG 실패의 실제 표현 대조

1954에서 동결 Native126의 같은 `samples/field-01.hwp`를 screen layer와 legacy로
직접 출력했다. 양쪽 모두 안내문 25글자가 `fill="#ff0000"`,
`font-style="oblique 18.418deg"`를 갖는다. 원 검사는 SVG 전체 문자열에 `italic`이
있어야 한다고 검사해 실패했다. 안내문이 실제로 직립으로 바뀌었다는 관측과 구분한다.
변경 출처는 현재 브랜치 `bf4579737f`·`0a766f5413`의 독립 Print 합성 기울임 보정이며
`text_italic.rs`의 공통 CSS 표현을 양쪽 SVG가 소비한다. 관련 Print 행렬 근거는 기존
`task_semanticist21_20261020_jamo_tone_italic.md`에 연결돼 있다.

첫 legacy 진단은 `--profile screen`을 함께 지정해 CLI 계약상 exit 2였다. 명시적
profile을 제거한 두 번째 명령으로 실제 legacy 출력을 확보했다. 실패 명령을 성공으로
덮어쓰지 않았다. 원본 전체 시각 근거와 현재 출력 판독을 확인한 뒤 검사 표현을
판정하며, 여기서는 테스트나 렌더링 코드를 변경하지 않았다. XML 속성과 경로는
`field-guide-svg-environment-1954/results.json`에 있다.

## 첫 조각 보정의 Native127 검증 결과

Native127 빌드는 `7c50f3b0ed` 뒤 해당 공통 컷 조회와 사전 이월 소비 수정본으로
242.53초, exit 0이다. 1955에서 기존 80168 완료 쪽 검사 9건과 앞의 저장 프레임
대조군 13건, 총 22건이 기대값 변경 없이 통과했다. 실패하던 93쪽 독립 기준선
검사도 통과했다. 1956의 실제 Native/Print 비교는 92쪽 99.97251%, 93쪽
99.99223%다. 양쪽 review와 standalone overlay를 직접 열어 92쪽에 잘못 남던
표 머리가 없어지고 93쪽의 완전한 표·후속 제목·본문 순서가 보존됨을 확인했다.
일반 모드에는 silhouette.tsv가 없으므로 1956 결과의 빈 pages 목록을 전쪽 TSV로
보고하지 않는다. 수치는 overlay/overlay_metrics.json의 실제 페이지별 raw다.

1960에서 같은 원본·한컴 2024 Print·compat 2024·96 DPI로 전 157쪽 Native
TSV를 별도로 산출해 최저 **93.96812%**, 90% 미만 0쪽을 확인했다. 93쪽은
이 실행에서도 99.99223%다. 명령은 logs/regulatory-native127-full-1960.log,
TSV는 regulatory-native127-full-1960/regulatory-native127-full/silhouette.tsv,
요약은 regulatory-native127-full-1960-result.json에 있다. 현재 fresh WASM과
고정 base 605ccfd92f의 전쪽 직접 비교는 아직 이 결과에 포함되지 않는다.

1958의 소형/대형 자식·영역 제한 4조건도 각각 96.09434/96.45868/99.87220/
99.88779%로 이전 Native125와 같고 전부 1쪽 Print에 대응한다. 1957의 18개
분할 정책 진단도 각 2쪽이며 Native125와 페이지별 결과가 같다. 4조건만 90%
이상이므로 남은 14조건을 완료로 처리하지 않는다. 자식 원장을 물리 부모 높이로
덮어쓰지 않고도 93쪽 회귀를 해소했으며, 분할 정책·부분 배치 보정은 계속한다.

## 18개 정책 조합의 물리 프레임과 소유 범위 분석

1961은 같은 18개 원본·독립 Print의 해시를 재확인하고 PDF 벡터 선·글자 원점과
Native127의 렌더 트리를 각각 기록했다. PDF는 pt, 렌더 트리는 96 DPI px이므로
단위 환산 후 대조한다. 진단 로그와 원장은 ignored output의
`logs/reflow-fragment-owners-1961.log`·`reflow-fragment-owners-1961.json`이다.

- 제한 false·부모 CELL·자식 CELL: Print의 첫 부모는 첫 쪽에 닫히지만 자식은
  2행/1행으로 이어진다. Native는 부모를 두 쪽 모두 이어 그리며 자식 첫 원점에
  부모 텍스트 패딩과 앞줄 점유를 더했다. 첫 자식의 Print 원점은 부모의 물리 상자
  원점과 같고, 현재 원점은 텍스트 원점을 사용한다. 단순한 x 이동만의 결함이 아니다.
- 제한 true·부모 CELL·자식 CELL: 자식 첫 원점은 이미 대응하지만 둘째 조각의
  시작 바깥여백과 첫 조각의 끝 바깥여백이 빠졌다. 사전 예산과 부모 외곽이 함께
  누락한 공간이므로 paint 테두리만 늘려서는 안 된다.
- 제한 false·부모 NONE·자식 CELL: Print의 부모 원자 정책이 제한 없는 자식의
  2행/1행 소유를 없애지 않는다. 현재 Native는 자식 세 행을 첫 쪽에 두었다.
- 제한 false·자식 NONE: Print는 자식 세 행을 첫 쪽에 두고 후속 본문을 둘째 쪽
  상단에서 시작한다. 현재 부모 컷 예산이 자식을 둘째 쪽으로 이월했다. 자식의
  쪽 정책과 부모의 짧은 물리 프레임을 같은 높이로 판정해서는 안 된다.
- 제한 true·부모 TABLE: Print는 부모 전체를 둘째 쪽으로 이월하고 후속 본문을
  첫 쪽에 남긴다. 현재는 부모를 두 쪽에 나누고 후속 본문도 둘째 쪽에 보낸다.
  같은 내용 순서만으로 지연 부동 개체의 물리 소유를 대신할 수 없다.

추가 수정은 부모 프레임 종료·자식의 미소비 원본 범위·부분 조각의 실제 원점/여백을
분리한 공통 계획에서 수행한다. 제한 없는 자식의 정책을 부모에 일괄 덮어쓰거나
부모를 강제로 원자화하는 방식은 위 반례를 깨뜨린다. 원본 유닛 주소는 보존하고,
사전 요구·스캔의 수용 높이·컷·실제 배치가 같은 조각 소유를 소비하도록 추적한다.
다음 보정의 기대값은 이 독립 Print 관측이며, 현재 수치나 잘못된 테두리에서 역산하지 않는다.

누름틀 원본의 1963 Native 대조는 기존 한컴 2020 Print의 전 3쪽과 쪽수가 같고,
raw는 100.00000/99.99969/100.00000%다. 해당 fresh WASM과 screen 안내문 직접
판독은 별도다. 인쇄 프로필의 누름틀 안내문 생략을 화면 안내문의 기울임 증거로
대신하지 않으며, 아직 원 테스트를 변경하지 않았다.

글자겹침의 과거 058 fixture 생성기를 현재 검사와 diff한 결과, 검사 본체·모든 단언은
같지만 생성기의 `char_shape_ids`가 `[0, 0]`에서 `[0, 0, 0]`으로 확장돼 있다.
따라서 그 생성 입력의 Print/TSV만으로 원 검사 입력의 시각 선행 조건을 충족했다고
보고할 수 없다. 형식상 유효성·누락 모양 fallback과 실제 원본 출력은 각각 확인한다.
이 차이를 근거 없이 원본 검사 제거 또는 새 입력으로의 교체 사유로 사용하지 않는다.

## 완료한 재검증과 구분해야 할 실행

1959의 50입력·145쪽 Native 재검증은 전부 완료했고 최저 93.39366%, 미달 0건이다.
각 페이지 raw를 Native125와 다시 대조한 1968에서도 변경 0건이다. 이 결과는
이전 후보 대조이며 최신 devel 직접 비교나 fresh WASM 완료로 바꾸어 보고하지 않는다.
Native127 제품 소스 627경로가 `17d4346fcd`의 Git 바이트와 같은지는 1962에서
확인했다.

1964의 전체 기본 feature 실행은 4,660개까지 로그가 있으나, 턴 중단 뒤 실행기와
nextest PID가 모두 없고 최종 요약·exit code도 없다. 완료로 인정하지 않는다.
`current-full-rust-1964-process-audit-1972.json`에 프로세스 부재를 확인한 근거를
보존하고 같은 소스·공용 target·잡 8개·스레드 12개로 1972를 실행했다. 실행기는
별도 세션으로 보존했고 실제 PID를 확인하며 종료 요약까지 관측한다.

## 같던 여백·위치를 분리한 독립 한컴 Print 대조

1967에서 부모 여백만 변경한 2개, 자식 여백만 변경한 2개를 만들어 총 4개를
한컴 2020 MCP Print로 출력했다. 원본과 구분한 입력·PDF의 해시와 실제 engine,
한컴 빌드, `pdf_print_method=0`·`hancom2020_pdf_driver_one_up`을 확인했다.
부모/자식의 다른 XML 속성 및 나머지 ZIP entry는 원본과 같음을 검증했다.
초기 생성기는 ZIP entry 메타데이터 객체를 쓰기 과정에서 재사용해 두 번째 사본을
완성하지 못했다. 그 불완전 입력은 별도 failed-generation 폴더로 보존했고,
MCP 호출 전 개수 검사에서 멈췄다. entry 메타데이터를 복사하도록 생성기만 고친
두 번째 시도는 4개 ZIP의 CRC와 의미 속성을 확인한 뒤 실제 Print를 완료했다.
원본 파일을 손상이나 Print 실패로 분류하지 않는다.

1969의 Native 대조도 실행했으며 새 대조군 4개는 모두 미달이다. 구체적인 Print
벡터와 글자 원점은 `fragment-margin-owner-print-vectors-1970.json`에 있다.
추가 1971은 부모의 가로·세로 오프셋만 바꾼 제한 false/true 2개다. 두 Print도
같은 인쇄 계약으로 성공했고 벡터는 `fragment-parent-position-print-vectors-1973.json`에 있다.

핵심 판정은 다음과 같다.

1. **제한 false의 자식은 부모 여백과 위치 오프셋을 바꿔도 움직이지 않는다.** 자기
   바깥여백을 바꿀 때만 움직인다. 앞선 1961의 부모/자식 원점 일치는 같은 여백
   수치에서의 관측이었다. 그것만으로 부모 셀 좌표계 소유를 입증할 수 없다.
   현재 `ReflowBlockOrigin.cell_relative`와 `reflow_block_table_origin`의 부모 셀
   원점 재사용 가정은 이 반례에서 성립하지 않는다. 제한 해제 개체의 host 문단/
   단 좌표계와 자기 여백을 보존하는 계획이 필요하다.
2. 제한 true는 부모를 따르고 내용 원점·자기 여백을 소비한다. 부모 세로 오프셋은
   첫 조각에서만 보이며 다음 쪽에서는 소진된다. 가로 오프셋은 이어받기에도 유지된다.
3. 제한 true에서 여백을 늘리면 실제 분할이 기존 2행/1행에서 1행/2행으로 바뀐다.
   그러므로 시작·끝 여백을 paint에서만 보충하거나 원본 컷을 그대로 두는 방식은
   새 예산에서 올바른 내용 소유를 보존하지 못한다. 예산과 실제 배치가 공통 조각
   계획을 소비해야 한다.

현재 전체 Rust 실행이 끝날 때까지 제품·테스트 소스를 고정한다. 위 자료는 후속
공통 계획 보정의 독립 근거이며 미달을 수용한 완료 보고나 신규 formal fixture가 아니다.

## 전체 Rust 실행 범위 확정과 공통 좌표계 보정 시작

1972도 최종적으로 외부 종료 신호 두 번으로 중단됐다. 정상 단언 결과 10,519건을
보존하고, 신호 중단·미실행 1,035건만 같은 빌드에서 1975의 200건 이하 묶음으로
이어 실행했다. 1974 전체 목록과 합집합을 대조해 **11,554건 모두 종료 결과**를
확보했다: 11,498 통과, 56 단언 실패, 미실행 0건, 기본 ignored 45건 별도다.
단일 무중단 실행으로 보고하지 않는다. 1977은 소스 3,324경로의 바이트 불변과
56개 실행 파일의 기존 mtime·현재 SHA를 확인했다. 전체 범위·실패 TSV·로그는
`current-full-rust-1977-audited.json`·`current-full-rust-1977-failures.tsv`에 있다.

1976의 입력군 한정 진단 모델은 Native의 잘못된 좌표를 쓰지 않고 원본 HU·글자 크기·
저장 선행 줄·각 개체의 쪽 정책과 여백으로 프레임·컷을 계산했다. 24조건 모두
페이지별 행 소유가 독립 Print와 같고 양방향 표 선 대응 최대 오차는 0.332022px다.
종료한 부모 프레임을 다음 쪽에 다시 발행한 초기 모델의 오류도 바로잡았다.
이는 제품·formal 회귀의 통과 근거가 아니며 제품에 옮길 계획의 독립 대조다.

후속 보정은 root 호스트 문단의 좌표계를 전체/부분 표 배치의 진입점에서 보존하고,
제한 없는 재조판 자식이 그 좌표계와 자기 여백을 소비하게 한다. 현재 `cell_relative`
가 부모 셀 원점·패딩·오프셋을 물려주는 가정을 제거한다. 제한 있는 자식은 실제
선택된 부분 문단의 흐름 위치를 유지하며 조각마다 자기 시작 여백을 연다. 미소비
자식의 컷·부모 물리 프레임·소스 예약 높이는 계속 분리한다. 이 좌표계 단계만으로
남은 물리 높이·분할 정책 문제를 완료로 판정하지 않는다.

## 호스트 좌표계 단계의 Native129 검증

Native128에서 부분 표의 자식 원점은 개선됐으나, 전체 표 진입점에 이미 위 여백을
포함한 `y_start`를 호스트 문단 원점으로 저장해 위 여백을 중복 소비했다. Native129는
`layout_table_control_block`이 가진 원래 `para_y_for_table`과 단의 문단 시작점을 먼저
보존한다. 재귀 전체/부분 배치는 활성 범위를 덮어쓰지 않으며 제한 없는 자식이
그 원점과 자기 여백을 소비한다. 제한 있는 자식은 선택된 부분 흐름의 원점과
자기 시작 여백을 사용한다. 부모 셀의 clip과 자식의 물리 용지 viewport를 구분한다.

동결 Native129는 컴파일 잡 8개로 248.68초에 빌드 종료 코드 0을 확인했다.
`char-overlap-native-build-129-result.json`이 입력 소스·라이브러리·실행 파일 해시와
명령을 보존한다. 다음 결과는 이 동결 제품에 대한 단계 검증이며 최종 head 검증이나
fresh WASM 검증을 뜻하지 않는다.

| 범위 | 실행 결과·증거 | 판단 |
| --- | --- | --- |
| 기존 저장 프레임·80168 완료 페이지 계약 | `current-previous-failures-1988-result.json`, 22 통과·실패 0·검사 누락 0 | 기존 독립 기대값을 변경하지 않았다. |
| 원래 자식 높이·제한 속성 4조건 | `two-tac-child-height-native129-1991/results.json`, 각각 96.09434·96.45868·99.87220·99.88779%, 한컴 Print와 각각 1쪽 일치 | Native128의 전체 표 원점 후퇴는 복구했다. Native만의 결과다. |
| 부모·자식 분할 정책 18조건 | `reflow-policy-native129-1989/results.json`, 36쪽, 4조건 통과·14조건 미달 | 자식 원점 보정만으로 부모 외곽·예약·정책 소유를 해결하지 못했다. |
| 서로 다른 부모·자식 여백 4조건 | `fragment-margin-owner-native129-1990/results.json`, 모두 2쪽 일치·raw 미달 | 예산·실제 조각의 공통 계획 보정이 필요하다. |
| 부모 위치 변경 2조건 | `fragment-parent-position-native129-1990/results.json`, 모두 raw 미달 | 부모 위치와 제한 없는 자식 위치의 독립성은 높이·컷 정책과 함께 검증해야 한다. |
| 80168 92·93쪽 직접 출력 | `regulatory-page93-native-comparison-1992/results.json`, 실행 종료 코드 0 | 일반 review 모드의 빈 `pages` 배열을 전쪽 TSV 통과 증거로 사용하지 않는다. |

`host-origin-native127-vs129-1995.json`은 직전 후보 Native127과 Native129를
십진수로 페이지별 대조했다. 이는 `upstream/devel` 비교가 아니다. 기존 통과 4조건은
유지됐다. 제한 false·부모 CELL·자식 CELL의 최저값은 34.32542%에서 80.62233%로,
제한 true의 같은 조합은 73.38141%에서 84.22101%로 개선됐지만 여전히 미달이다.
부모 TABLE·제한 true·자식 CELL/TABLE의 2쪽은 각각 28.65269%에서 23.52518%로
5.12751%p 하락했다. 이 후퇴를 2%p 허용폭이나 다른 조건의 개선으로 면제하지 않는다.

`reflow-host-origin-review-1993/`의 1쪽 review와 standalone overlay를 함께 열어
자식 표 원점뿐 아니라 남은 부모 외곽의 파란 선과 이모지 모양 차이를 확인했다.
raw는 약 80.62%이며 시각 완료가 아니다. 구조 차이·글꼴 차이를 숨기지 않는다.

빌드 이후 `layout.rs`의 한 호출을 rustfmt 형식으로 정리했다. 최초 fmt 검사는 그
한 호출에서 실패했고 정리 후 재검사는 종료 코드 0이었다. 동결 빌드 소스 사본과
현재 파일의 rustfmt 정규화 본문이 동일함을 `native129-format-only-1995.json`에
확인했다. 초기 단순 공백 비교는 끝 쉼표 차이를 거부했고, 다음 비교는 도구가 붙인
파일명 헤더 차이를 거부했다. 실제 헤더를 각각 검증한 뒤 본문을 비교했다.
이 형식 동등성은 최종 커밋의 필수 검증을 대신하지 않는다.

## 합친 문단의 TAC 뒤 글자 축 진단

`inline-axis-probe-1987/`은 기존 검사 소스에서 만든 진단용 실행이며 formal 검사를
변경하지 않았다. Plain·Field·Bookmark 각각 편집 직후와 HWP/HWPX 재열기 9상태의
문단 텍스트, 공개 text/logical 변환 결과와 실제 렌더 런을 기록했다. 모델의 텍스트는
`첫문단가나다라`지만 마지막 `라` 런의 `char_start`는 7이고, 공개 변환의 text index 6은
여전히 logical index 6이다. 따라서 단순히 테스트 helper에 공개 변환을 호출하는 것만으로
해결된다고 결론 내릴 수 없다. 원래 원인·축 계약·독립 출력의 대조를 이어간다.
진단 실행 성공을 올바른 화면 배치나 신규 회귀의 시각 선행 조건 충족으로 보고하지 않는다.

## 제한된 자식 조각의 시작·종료 여백 보정 계획

1997은 동결 Native129의 소스 유닛·선택 컷을 6조건에서 실제 실행했다. 자식 3행은
55.533333·51.760000·55.533333px 유닛으로 투영된다. 실제 행 본체는 각각 51.76px이며
위·아래 여백은 전체 자식의 첫·마지막 유닛에만 들어 있다. 그러므로 첫 두 행의 컷과
마지막 한 행의 이어받기에 유닛 합만 사용하면 각 조각의 닫기/열기 여백이 빠진다.
선행 저장 줄의 21.333333px 전진과 자식이 이월된 호스트 빈 줄도 구분해야 한다.

선택된 `nested_row`·재귀 `NestedTableCut`에서 자식의 실제 조각 높이를 질의하고,
동일한 호스트 줄 높이와 자기 양쪽 여백으로 제한된 자식의 내용 프레임을 계산한다.
이 결과를 행의 수용/점유 높이와 셀의 가시 높이가 함께 소비하도록 한다. 유닛 주소와
저장 컷을 바꾸지 않는다. 제한 없는 자식은 부모 외부 프레임과 독립된 내용 스트림을
가지므로 이 포함 관계를 적용하지 않으며, 그 예약·정책 소유 보정은 계속 필요하다.
18정책과 서로 다른 여백/위치 대조군, 기존 완료 페이지 계약을 같은 후보로 확인한다.

## 공통 컷 높이의 Native130 결과와 소비 경로 추가 보정

Native130은 잡 8개·공용 target으로 256.37초에 빌드 종료 코드 0을 확인했다.
첫 실행기는 진행 중이던 fmt를 감지해 컴파일 전에 거절됐다. fmt 호출 형식을 고친
재검사가 끝난 뒤 실제 빌드를 순차 실행했다. 실패한 fmt와 실행기 기록을 보존한다.

- `current-previous-failures-1999-result.json`: 기존 계약 22 통과·실패 0·누락 0.
- `reflow-policy-native130-2000/results.json`: 18조건·36쪽 중 6조건 통과,
  12조건 미달. 제한 true·부모 CELL·자식 CELL/TABLE은 84.22101%에서
  94.63329%로 개선됐다. 정책 전체가 완료됐다는 뜻은 아니다.
- `fragment-margin-owner-native130-2001/results.json`: 제한 true의 자식 여백은
  52.83858% → 93.93876%, 부모 여백은 84.92662% → 94.52942%다.
  제한 false의 두 조건은 81.43136/52.14760%로 미달이다.
- `fragment-parent-position-native130-2002/results.json`: 제한 true는
  52.49219% → 94.71542%, false는 81.26038%로 미달이다.
- `two-tac-child-height-native130-2003/results.json`: 원래 4조건은 모두 90% 이상이며
  Native129의 점수와 쪽수를 유지했다.

`restricted-child-margin-review-2006/`의 2쪽 review와 standalone overlay를 함께
직접 읽었다. 93.94% 통과 이후에도 부모 외곽을 자식 폭에 맞춰 늘린 파란 선과
이모지 모양·색 차이가 보인다. 수치만으로 완료하지 않는다. source/Print의 이모지
차이와 프레임 차이를 분리해 남긴다.

생산·소비 경로는 `reflow_cut_content_height` → `row_cut_height`의 Capacity/Occupied와
`cell_cut_visible_height` → 부분 배치의 행 높이다. 그 뒤의 별도 소비도 대조했다.
`extend_clipped_cell_horizontal_clip_to_nested_table_borders`가 부모 칸 폭을 바꾸고,
`fold_last_row`가 선행 저장 줄간격 8px를 다시 접으며, 정렬은 별도 내용 높이를 선택한다.
현재 후속 후보는 같은 컷의 명시적 프레임을 정렬·셀 clip·꼬리 접기에 전달한다.
부모 괘선 폭과 자식 viewport를 분리하고, 컷을 받은 자식을 부모 clip으로 다시 자르지 않는다.
여러 행에 걸친 칸은 기존 걸침 조각 원장이 소유하므로 단일 행의 인증을 그대로 적용하지 않는다.
Native131의 실제 실행 결과는 별도로 확인하며 Native130을 새 후보 검증으로 재인용하지 않는다.

## TAC 뒤 글자 실패 원 PR과 현재 독립 Print

기존 통합 기록에서 원 PR #7616을 찾고 GitHub 본문·커밋 목록을 다시 확보했다
(`inline-axis-pr7616-2016.json`). 원 등록 46a8bd7b9d가 인용한 테스트 전용 커밋은
7a58f1d7c7이며 원 PR은 줄 경계·누름틀/책갈피 갭과 뒤 글자 소유의 결함을 설명한다.
원 PR은 한컴 출력·fresh WASM 미실행을 명시했고, 이후 통합 보정의 Print 100% 근거는
기존 `semanticist21_20261008_integrated_review.md`에 있다. 원 커밋의 자동 PR 연결
조회는 빈 배열이었으며 이를 원 PR 부재로 판정하지 않고 기존 통합 기록으로 추적했다.

현재 테스트 helper 그대로 만든 HWP/HWPX 6개의 한컴 2020 MCP Print를 완료했다.
HWP에는 템플릿의 2010, HWPX에는 2020 저장 메타데이터가 있으며 실제 생성은
Native129의 공개 API와 기존 helper다. 메타데이터를 한컴 저작·저장 실행 증거로
승격하지 않는다. 각 요청의 실제 engine 2020·PrintMethod 0·1-up 출력 모드·입력/PDF
해시는 `mcp-inline-axis-2005/`에 보존했다. 실패 0건이다.

`inline-axis-native130-2007/results.json`에서 6개 모두 Print와 1쪽 일치·raw 100%다.
렌더 런은 논리 축으로 표 슬롯을 포함하지만 기존 `char_box`는 텍스트 글자 인덱스와
직접 비교한다. `text_to_logical_offset`은 같은 위치의 개체 앞 삽입 경계를 반환하므로
그 함수 호출만 넣는 수정은 부적절하다. 실제 런의 논리 시작을 텍스트 축으로 역변환해
소유 글자를 찾는 방향으로 진단한다. formal 검사 변경은 같은 원본의 fresh WASM과
직접 판독까지 확인한 뒤 수행하며, 기존 내용·줄 순서 관계와 실패 검출은 유지한다.

## Native131의 실제 배치 소비 결과와 남은 차이

공용 `target/pr-review`·잡 8개로 246.04초에 빌드 종료 코드 0을 확인했다.
산출물 SHA-256은 `ce7943ec066da88c1f7ddf159ff793e0709099713b8b9d10234020e54e7eaac0`이며
`char-overlap-native-build-131-result.json`의 소스 628개 해시와 현재 바이트를 대조했다.
`current-previous-failures-2010-result.json`의 기존 계약 22개는 모두 통과했다.

- `reflow-policy-native131-2011/results.json`: 18조건 중 7조건 통과·11조건 미달이다.
  제한 true·부모 CELL·자식 NONE은 40.79432% → 93.37929%로 개선됐다.
  자식 CELL/TABLE은 93.95085%이며, 부모 NONE의 세 조건은 93.39512%다.
  Native130 대비 각각 0.68244%p와 1.71550%p 하락했다. 후자는 내용 경계의
  좌표 이동 없이 기존 clip 밖 잉크가 드러난 차이를 포함하므로 수치만으로 완료하지 않는다.
- 여백 4조건과 위치 2조건은 Native130의 점수와 쪽수를 유지했다
  (`fragment-margin-owner-native131-2012/`, `fragment-parent-position-native131-2013/`).
- 기존 자식 높이 4조건은 모두 90% 이상이다. 제한 false의 큰/작은 글자는
  96.09434/99.87220%, 제한 true는 95.15682/99.03650%다
  (`two-tac-child-height-native131-2014/results.json`). true의 값은 이전보다 낮아
  clip 변경 영역을 직접 판독해야 하며 기존 결과를 그대로 재인용하지 않는다.
- TAC 뒤 글자 6개 저장본은 모두 Native raw 100%·독립 Print 1쪽과 일치했다
  (`inline-axis-native131-2015/results.json`). 원 검사는 변경하지 않았다.
  `inline-axis-helper-diagnostic-2019/`의 별도 진단에서 원 helper 3개는 실패하고
  실제 런의 논리 시작을 텍스트 축으로 변환한 helper 3개는 통과했다. 원본 내용과
  LineSeg·앞뒤 순서 검사는 유지했다. fresh WASM 확인 전 정식 검사 변경은 보류한다.

`restricted-child-margin-review-2018/`의 review와 standalone overlay를 직접 읽었다.
부모 오른쪽 괘선의 폭 차이는 여전히 남아 있다. 앞 절에서 clip 경로를 소비 지점으로
찾았다는 사실이 폭 문제의 해결을 뜻하지 않는다. 선언 폭·행 격자·부분 배치의 최종
폭 선택을 계속 대조한다. 제한 false의 독립 스트림과 부모 TABLE 정책도 미해결이다.
fmt와 diff 공백 검사는 통과했으며, 현재 단계는 로컬 WIP 고정이다. PR 완료 판정이 아니다.

## 함초롬바탕 지정 문자와 실제 흑백 대체 글꼴

사용자가 한컴 GUI의 지정 글꼴을 함초롬바탕으로 확인했다. 원본 HWPX도 같은 글꼴이며,
U+1F9A6 문자는 시스템의 해당 프로그램 cmap에 없다. 독립 Print PDF의 이 문자 네 런은
`SegoeUIEmoji`·색 0의 흑백 출력이다. 지정 글꼴과 실제 fallback을 구분한다
(`emoji-font-source-observations-2020.json`). 기존 RHWP는 로컬 Noto Color Emoji를
사용해 다른 모양과 색을 보였으며, 이를 올바른 출력이나 해결 불가능한 차이로 판정하지 않는다.

Windows 검증 호스트의 원본 `C:\Windows\Fonts\seguiemj.ttf`를 ignored 진단 경로로
읽기 전용 복사했다. 버전 1.29·SHA-256
`7c0244dd8eeb7c6bdecdfc3f9e59833527fc18a66d0295ce47339069692a2b4f`이며,
해당 문자에는 7개 윤곽과 별도 COLRv0 색상 레이어가 모두 존재한다
(`emoji-font-probe-2021/seguiemj.json`). 시스템 등록이나 저장소 글꼴 추가는 하지 않았다.

같은 Native131에 이 프로그램만 추가 공급한 `restricted-child-margin-segoe-2022/`의
새 review·overlay를 함께 판독했다. 2쪽 raw는 93.93876% → 97.41572%로 높아졌고
모양은 가까워졌지만, RHWP의 색상 레이어가 여전히 표시된다. 실제 `overlay_metrics.json` 값을 우선하며
반올림한 PNG 점수와 구분한다. 글꼴 공급과 일반 텍스트의 흑백 윤곽 선택은 별개 원인이다.
해당 출력은 진단 결과이며 수정 완료 증거가 아니다. 일반 텍스트 fallback과 명시적인
컬러 글리프 payload 계약을 구분해 공통 경로의 해결 방법을 검토한다.

## 일반 텍스트의 COLR 기본 윤곽 선택 보정 계획

Native131의 실제 렌더러 snapshot을 쓴 `emoji-font-probe-2021/findings.json`에서
문서 face는 함초롬바탕, 로컬 paint 선택은 Noto Color Emoji, Windows 프로그램 공급 시
선택은 Segoe UI Emoji로 확인했다. 독립 PDF의 부분 글꼴 glyph 3은 Windows 원본
U+1F9A6의 기본 윤곽과 98개 명령의 좌표까지 동일하다
(`emoji-font-probe-2021/pdf-outline-comparison.json`).

`emoji-presentation-controls-2026/plan.json`은 원본 복사와 face/VS16만 바꾼 수동 대조군의
생성 차이를 기록한다. 2020의 명시 face/VS16, 2024의 원본과 명시 face/VS16 Print
3건 모두 같은 흑백 기본 윤곽을 출력했다. 실제 engine·PrintMethod 0·1-up 경로를
`mcp-emoji-presentation-2026/`에서 확인했다. 2024 교차 버전 출력은 동작 대조 자료이며
2020 원본의 정식 기준 PDF를 대체하지 않는다. OpenType COLR 사양도 기본 glyph에
윤곽이 있으면 비컬러 glyph로 그리는 경로를 정의한다
([공식 COLR 사양](https://learn.microsoft.com/en-us/typography/opentype/spec/colr)).

공통 Rust 처리에서 COLR 기본 glyph 모두에 유효한 기본 윤곽이 있는 프로그램만
일반 텍스트용 SFNT 사본으로 만든다. cmap·glyph ID·윤곽·advance·GSUB/GPOS·이름은
유지하고, 컬러 레이어 선택 표와 사본에서 무효가 된 서명만 제외한다. 원본 파일과
명시적인 GlyphRun/GlyphOutline 컬러 자원은 변경하지 않는다. 윤곽 없는 컬러 전용
프로그램은 이 변환을 적용하지 않는다. 특정 문서·문자·face 이름으로 분기하지 않는다.
Native의 nominal·shaping font 생성과 브라우저 일반 FontFace 공급이 같은 함수를
소비하게 하고, strict 자원 replay는 원 프로그램을 계속 사용한다. 공급 글꼴의
원본 해시와 paint 사본 해시를 구분한다. 같은 source/PDF의 Native/fresh WASM과
명시 컬러 자원 대조 검사가 끝나기 전 수정 완료로 판정하지 않는다.

## 일반 텍스트 보정의 Native132 결과

공용 target·잡 8개로 243.53초에 Native132 빌드 종료 코드 0을 확인했다.
`text_font_presentation::ordinary_text_font_program`이 COLR 기본 glyph 전체의 윤곽을
확인하고 SFNT 사본을 만든다. Native의 `fonts::finish`가 이 사본을 캐시해 nominal과
SkShaper 양쪽에 공급한다. 일반 Canvas FontFace와 CanvasKit의 host font 준비는
문서의 같은 Rust API를 소비하며, SVG 임베드도 같은 함수를 사용한다. strict
GlyphRun/GlyphOutline 자원의 원본 프로그램은 이 준비 경로에 넣지 않는다.

Windows 원본 Segoe UI Emoji를 `/usr/local/share/fonts/rhwp-hancom-fallback/seguiemj.ttf`에
등록했다. 원본 SHA-256은 이전 진단과 같으며 실제 `fc-match`가 Noto Color Emoji에서
Segoe UI Emoji로 바뀌었음을 `emoji-system-font-registration-2033.json`에 기록했다.
글꼴 파일은 Git에 추가하지 않았다. 이후 base/후보 비교에는 같은 새 공급 환경을 사용한다.
과거 Noto 환경의 숫자를 새 환경의 무회귀 증거로 재사용하지 않는다.

같은 새 시스템 글꼴·입력·2020 Print·96DPI에서 다음과 같이 직접 대조했다.

| 출력 | 1쪽 raw | 2쪽 raw |
| --- | ---: | ---: |
| Native131, 색 레이어 선택 | 97.91111% | 97.41572% |
| Native132, 기본 윤곽 선택 | 98.82051% | 98.57706% |

증적은 `restricted-child-margin-native131-2034/`와
`restricted-child-margin-native132-2034/`다. 새 2쪽 review·standalone overlay를 함께
확인했으며 해당 문자가 한컴과 같은 흑백 윤곽으로 바뀌었다. 부모 오른쪽 괘선의 폭 차이는
여전히 보이고, 이 페이지를 완료된 98% px 고정 대상으로 승격하지 않는다.

`ordinary-text-font-contract-2035/`는 실제 원본 프로그램에서 12,189개 glyph의 전진폭과
경계 상자, glyph 수·em·cmap·그 외 표의 바이트 보존을 확인했다. 2,786개 컬러 기본 glyph가
일반 사본에서만 비컬러가 되며, 원본에 대한 명시 COLRv0 디코드는 8개 컬러 레이어와 strict
계약을 유지했다. 전체 checksum·재적용의 비적용·잘못된 face/잘린 입력 거절도 통과했다.
비컬러 함초롬바탕과 윤곽 없는 Noto Color Emoji는 사본 변환을 적용하지 않는다.
이는 실행 진단이며 정식 렌더링 회귀나 fresh WASM 완료를 대신하지 않는다.

TypeScript 검사와 기존 글꼴 계약 26개·Sweep 브라우저 계약 2개는 통과했다.
`ordinary-text-font.test.ts`는 원본 바이트 보존·WASM 반환 view와 수명 분리·비적용·무효 face
선택을 검사하며, 기존 계약과 함께 29개 통과했다. 로그는 각각
`ordinary-text-font-tsc-2029.log`, `ordinary-text-font-ts-contracts-2036.log`,
`ordinary-text-font-sweep-browser-2031.log`다. 새 Rust 처리의 직접 실행과 브라우저 연결
계약을 구분하며, 최적화 fresh WASM의 실제 출력 확인 후 최종 판정한다.

앞 단계 Native131과 같은 소스의 WASM045는 864.77초에 빌드·Studio 동기화를 완료했다.
`inline-axis-wasm045-2025/results.json`의 저장본 6개는 모두 raw 100%·전체 쪽수 1 대 1·
누락 0쪽이다. 첫 결과 집계기가 잘못된 manifest 파일명을 읽어 gate를 null로 둔 문제를
바로잡았고, 실제 `silhouette_manifest.json`의 전체 쪽수와 미달/누락 목록을 다시 확인했다.
이 증거는 이전 소스의 TAC 뒤 글자 대조이며 Native132의 fresh 검증으로 재인용하지 않는다.

## WASM046과 Native 50개 입력 재검증

WASM046은 같은 소스에서 849.52초에 빌드 종료 코드 0과 Studio JS/WASM 동기화 해시 일치를
확인했다. 원본 글꼴과 실제 일반 FontFace 사본의 해시는 `textProgramSha256`으로 구분한다.
`restricted-child-margin-wasm046-2043/`의 1/2쪽 raw는 98.81747/98.57541%이며 전체 쪽수는
Print와 2 대 2다. 새 2쪽 review·standalone overlay에서 흑백 윤곽을 확인했다. Native의
98.82051/98.57706%와 소수 차이는 있지만 같은 개선을 확인했으며, 부모 경계 밖 자식 괘선은
양쪽 모두 아직 남아 있다. 이 자료만으로 페이지 완료나 px 회귀 자격을 선언하지 않는다.

`native132-fifty-inputs-2039-audited.json`에서 기존 50개 입력의 같은 범위 145쪽이 모두
90% 이상이며 최저값은 93.39366%다. 입력과 독립 Print의 SHA도 이전 고정본과 일치했다.
원 실행 집계기는 첫 key의 중복 하이픈 정규화를 반영하지 못해 TSV 1개를 찾지 못했다.
원 로그를 보존하고 기존 정규 key의 실제 TSV/manifest로 재출력 없이 감사해 해결했다.
렌더링 실패나 원본 변경으로 분류하지 않는다. 이 50개 범위의 fresh 전체와 새 base 대비
재검증은 별도로 필요하다. 과거 Native127/Noto 환경과의 비교는 devel 무회귀 증거가 아니다.

## 부모 셀 클립의 독립 PDF 근거

`reflow-cell-clip-ownership-2042.json`은 PDF 벡터 선과 실제 클립을 96DPI 좌표로 보존한다.
제한 true에서 자식의 오른쪽 선 자체는 약 696.1px지만 부모 클립 우단은 약 676.3px다.
즉 자식 선언 폭을 줄인 출력이 아니라 부모 셀에서 잘린 출력이다. 제한 false는 자식
자신의 전체 클립을 사용하고 부모 셀 프레임 밖으로 이어진다. 현재 트리에서도 제한 없는
자식은 부모 Cell의 자손이 아닌 Table의 직접 자식으로 배치되므로, 부모 Cell 클립을
다시 켜더라도 이 별도 자식의 클립을 부모 것으로 바꿀 필요는 없다. 다음 보정은 이
소유 관계를 유지하며 닫힌 부모 프레임의 클립과 별도 괘선 확장 게이트를 대조한다.

현재 원격 devel의 읽기 전용 조회는 def947c2149df5bc812d801e6e79be19a403b4d1로,
고정 기준 605ccfd92f와 다르다. 현재 fresh 단계 결과와 생성 JS를 먼저 커밋한 뒤 새 base의
변경·지침을 읽고 동기화한다. 리베이스 자동 적용을 기능 보존 증거로 대신하지 않는다.

## def947c214 동기화와 다음 보정

새 base는 def947c2149df5bc812d801e6e79be19a403b4d1, 리베이스 직후 후보는
c520638aea2f849319c346d4f73b3a27bd0354c8이다. backup 브랜치를 보존한 뒤 197개 커밋을
재적용했다. 진행 중 상태 조회의 인덱스 잠금 경합으로 54번째 적용이 중단됐다. 잠금·활성 Git
프로세스가 자연 종료됐음을 확인하고, staging patch-id가 원본 139811191c와 같은지 검증한
뒤 원 저자·메시지로 해당 커밋을 고정하고 같은 리베이스를 계속했다. 소스 되돌리기·reset·
커밋 누락으로 해결하지 않았다. range-diff의 196개는 같고 2%p 지침 1개의 차이는 새 문서
문맥 앵커 이동뿐이다. 결과 트리의 차이는 새 base의 39개 경로이며 실행 소스 634개는 모두
Native132 snapshot과 같다. 정확한 기록은 `rebase-def947-2045-result.json`과
`rebase-def947-2045-lock-recovery.json`에 있다.

들어온 #7474·#7488·#8171·#8173의 본문·파일·커밋을 `rebase-incoming-pr-*-2047.json`으로
확인했다. 제품 Rust 조판 코드는 바뀌지 않았다. #7488은 명시 opt-in만 cdylib 경로로 바꾸고
기본은 standard release다. #7474는 패키징·브라우저가 같은 standard release 바이트를
소비했는지 확인한다. 새 wrapper/release/workflow Python 계약과 Studio artifact·글꼴 계약,
CI impact 검사를 실행해 성공했다. 실제 최종 standard build의 manifest·브라우저 소비·배포
전후 바이트 일치는 다음 새 후보의 최종 패키지에서 확인해야 하며, 이전 receipt를 새
source SHA의 release manifest로 고쳐 쓰지 않는다. #8171·#8173의 새 절차도 읽었고,
`own-open-pr-preflight-2046.json`에서 본인 열린 PR 0개를 확인했다.

다음 소스 묶음은 독립 PDF 클립 근거에 따라 부모 Cell 클립을 복원한다. 제한 없는 자식은
이미 Cell 밖 Table 자식으로 배치되므로 별도 스트림 소유는 유지한다. 닫힌 reflow 프레임의
괘선 확장 금지는 유지하며 자식 선언 폭·좌표를 줄여 맞추지 않는다. 글꼴 프로그램의 정렬·
중복 검사는 COLR/CPAL/DSIG를 제외하기 전 원본 디렉터리 전체에 적용하도록 보완한다.
두 보정 뒤 같은 입력·Print의 Native와 기존 대조군을 다시 확인한다. 독립 자식 스트림의
부모 물리 프레임 반복과 TABLE 원자 행 이월 문제는 여전히 해결해야 한다.

## Native133의 클립 복원과 디렉터리 검증 결과

새 base 위에서 공용 target·잡 8개로 246.71초에 Native133 빌드 종료 코드 0을 확인했다.
부모 셀 클립은 유지하고 닫힌 reflow 프레임의 괘선 확장 금지도 유지했다. 제한 없는 자식은
기존 Table 직접 자식 소유를 유지하며 셀 경계로 옮기지 않았다. 선언 폭·좌표를 줄이지 않았다.

`restricted-child-margin-native133-2055/`의 전체 2쪽은 Native raw가 각각 100%이며
`native-export.json`의 `pageCount: 2`·`all_pages_exported: true`를 확인했다. 2쪽 review와
standalone overlay를 함께 읽어 부모 경계 밖의 파란 선이 사라진 것을 확인했다. RGB 엄격
일치는 35.99%이므로 이를 전체 픽셀 동일이나 모든 렌더링 정확도 100%로 보고하지 않는다.
fresh WASM은 이 새 후보로 다시 산출해야 한다.

`current-previous-failures-2056-result.json`의 기존 완료 페이지·저장 프레임 계약 22개가
모두 통과했다. `current-previous-failures-2060-result.json`의 strict Native glyph replay
10개도 모두 통과했다. 후자에는 실제 COLRv0 레이어 순서·색 출력, 비트맵·SVG·가변/collection
face와 outline 효과가 포함된다. 일반 텍스트 기본 윤곽 처리를 명시 컬러 자원 재생에 적용하지
않는 계약을 실제 실행으로 확인했다.

`font-directory-duplicate-probe-2058/`은 같은 실제 프로그램의 COLR 디렉터리 항목만 중복한
잘못된 입력을 만들었다. ttf-parser의 face/컬러 glyph 조회가 가능한 이 입력을 이전 Native132
함수는 사본으로 승격했고, Native133은 거절했다. 입력의 glyph·표 payload는 바꾸지 않았다.
이 자료는 진단이며 독립 Print 또는 정식 렌더링 회귀의 증거로 승격하지 않는다.

`reflow-policy-native133-2057/results.json`의 18조건 중 7조건은 모두 전쪽 raw 100%다.
나머지 11조건은 여전히 90% 미만이다. 제한 없는 자식의 독립 스트림과 부모 물리 프레임,
TABLE 원자 행 이월을 해결해야 한다. 지적한 제한 true·자식 여백 사례의 개선으로 이 미달을
상쇄하지 않으며 PR 준비 완료나 전체 회귀 통과를 선언하지 않는다.

다음 fresh 빌드는 새 devel의 #7474 절차대로 `measure_render_diff_wasm.py`의 standard
release 경로와 실제 wasm-opt 관측 manifest를 사용한다. pkg·Studio·보존 package의 바이트
일치를 확인하며, 최종 브라우저 소비까지 같은 package를 사용해야 한다. 새 source SHA를
이전 build receipt에 대입해 release 검증을 우회하지 않는다.

## WASM047의 실제 소비와 흑백 문자 재검증

사용자가 한컴 UI에서 확인한 지정 글꼴을 원본 HWPX와 다시 대조했다. 원본의 이름은
`함초롬바탕`이다. 이 환경의 같은 글꼴 프로그램에는 U+1F9A6 glyph가 없으며, 독립 Print
PDF에 포함된 `SegoeUIEmoji`의 해당 윤곽은 Windows 원본 글꼴의 U+1F9A6 윤곽과 같다.
지정 글꼴과 실제 출력에 사용한 대체 글꼴을 구분한다. 지정명을 바꾸지 않고 일반 텍스트의
대체 글꼴을 흑백 기본 윤곽으로 준비하는 공통 처리를 Native와 fresh WASM에서 확인했다.

WASM047은 source SHA `bbd1bfab9af431b44e5ed85d1ff302438cf7a506`의 깨끗한 tracked
소스에서 standard release·잡 8개로 840.78초에 종료 코드 0을 확인했다. 실제 wasm-opt 실행,
`pkg`·Studio·보존 package의 JS/WASM 해시 일치를 확인했다. 같은 앱 초기화에서 받은 WASM
응답을 CDP로 관측한 해시는 `147d3556d226a353184389787a3179a5cb4ad9525ac30c031849f59763f80bba`다.
`verify_release_wasm.py --require-consumption`도 통과했다. 재요청한 파일이나 추정 해시로
브라우저 소비를 대신하지 않았다. 근거는 `release-standard-wasm047-2063/build.json`,
그 아래 `consumption/`, `logs/wasm047-release-consumption-2070.log`다.

`ordinary-text-browser-consumption-2069-result.json`에서 실제 CanvasKit 앱과 WASM bridge를
통한 일반 글꼴 준비를 확인했다. 입력 프로그램 해시는 `7c0244dd…a2b4f`, 일반 텍스트용 사본은
`4206bc9c…961d8`이며 원본 바이트는 보존된다. 이 관측은 명시 컬러 자원 재생의 계약과 구분한다.

같은 원본과 2020 Print의 전체 2쪽을 Native133과 WASM047에서 다시 비교했다. 양쪽 모두
raw 실루엣 100/100%, 쪽수 2 대 2, 누락 0쪽이다. TSV는
`restricted-child-margin-native133-scores-2067/silhouette.tsv`와
`restricted-child-margin-wasm047-scores-2071/silhouette.tsv`에 있다. fresh의 review·standalone
overlay 두 쪽을 직접 읽어 흑백 문자와 부모 셀의 클립을 확인했다. 2쪽 엄격 내용 픽셀은
38.29%이므로 raw 100%를 전체 색·픽셀 동일로 보고하지 않는다. fresh 원본 이미지·전체 쪽수
메타데이터는 `restricted-child-margin-wasm047-2066/`에 보존했다.

기존 TAC 뒤 글자 helper의 저장본 6개도 Native133에서 전체 1쪽 대 1쪽, 각 raw 100%를
확인했다(`inline-axis-native133-2068/results.json`). 이 여섯 입력의 현재 fresh 검증과 정식
helper 보정은 이어서 진행해야 한다. 아직 남은 11개 재조판 정책 미달과 전체 Rust 실패의
해소를 이 결과로 대신하지 않으며, PR 준비 전체 완료로 판정하지 않는다.

## TAC 뒤 글자 회귀 helper의 위치 축 보정

원 해결 PR #7616의 공개 API 재현·원 테스트와 #7882 통합 근거를 다시 대조했다. 기존
`char_box` helper는 `RenderRun.char_start`의 논리 위치를 본문 글자 번호와 바로 비교했다.
표·누름틀·책갈피 슬롯 뒤에서는 두 축이 달라지므로 문서의 `logical_to_text_offset`으로
변환한 뒤 해당 글자를 찾도록 보정했다. 원래 문단 내용·저장 LineSeg 시작점·앞 글자 같은 줄·
표와 뒤 글자의 순서 관계는 그대로 유지했다. 절대 좌표 핀이나 새 수치 기대값을 넣지 않았다.

같은 원본 HWP/HWPX 저장본 6개와 독립 2020 Print는 Native133·WASM047에서 각각 전쪽
raw 100%, 전체 쪽수 1 대 1, 누락 0쪽이다. 증거는 `inline-axis-native133-2068/results.json`과
`inline-axis-wasm047-2073/results.json`이다. `inline-axis-direct-review-2076/`에서 동일 원본
raster의 6개 확대 비교와 대표 review·standalone overlay를 생성해 내용과 표 앞뒤 순서를
직접 읽었다. 표시용 crop은 별도 manifest에 기록했고 원본 raster와 TSV는 유지했다.

`inline-axis-formal-2077/`은 이전 정식 파일의 사본과 보정한 정식 파일을 같은 Native133
library에 연결해 실행했다. 수정 전 3 FAIL, 수정 후 3 PASS이며 실패를 숨기는 필터·expectation
완화는 없다. 로그는 `logs/inline-axis-formal-2077-{before,after}.log`다. 정식 파일과 기존
자동 suite 경로를 유지했으며, helper 보정은 제품 library의 코드 변경이 아니다.
