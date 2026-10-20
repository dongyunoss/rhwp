# OOXML 값축 글줄 높이와 숫자 표시 간격

## 원인과 독립 근거

이 단계는 `review/semanticist21-20261019`의 전체45개 PR·Draft10개 검토 범위를 유지한다.
원 이슈/해결 경로는 #1882→#1890·#1954, #6624→#6626→#6645이며 이전
[차트 글줄 기록](task_semanticist21_20261020_chart_text_frames.md)의 선행 분석을 재사용한다.
현재 숫자 생략의 `ceil(2font/spacing)`은 외부 PR의 원문이 아니라 이 브랜치의 중간 보정
`fa889d3f48f`에서 추가됐다. 잘못된 현재 출력을 보정하며 기존 기대값을 완화하지 않는다.

작업 디렉터리는 `output/pr-review/semanticist21-20261019/`다. 1003은 같은18pt 차트의
높이만225·230·240으로 바꾼 세 입력이다. 현대 차트와 fallback OLE의 공통 크기를 함께
갱신했고 값·글꼴·범례·차트 XML/CFB 바이트는 보존했다. 독립 진단 입력이며 원본 통과로
보고하지 않는다. 실제 저장 제품에 따라 한컴2020 engine의 PrintMethod0·one-up으로
출력했고 세 요청 모두 성공했다. 입력/PDF SHA와 한컴 버전은 각 complete 영수증에 있다.

1005는 커밋 `11767e35f32b52ab54e899836c5e1dc5ce5be10d`의 Native074 실제 SVG에서
축 끝과 숫자를 읽은 기하 진단이다. 1006은600dpi Print의 숫자 잉크 밴드를 같은0~6 축의
위치와 대조했다. 숫자 자체도 review/overlay에서 직접 확인하며 밴드 위치만으로 완료하지 않는다.

| 차트 높이 | 독립 Print의 숫자(아래→위) | Native074 |
| --- | --- | --- |
| 225 | 0·6 | 0 |
| 230 | 0·4 | 0 |
| 240 | 0·4 | 0·6 |

225의 실제 축 높이는43.04, 230은48.08, 240은58.08이었다. 세 칸의 눈금 간격에
24px 글자의 기존 공통 줄 높이1.3em을 적용하면 각각3·2·2칸의 라벨 간격이 된다.
2em을 반올림하는 가설은230에서3칸을 골라 독립 Print와 다르므로 채택하지 않는다.
단순 좌표 이동이나 제목 줄 높이1.25배로 이 숫자 선택 차이를 숨기지 않는다.

## 적용 계획과 소비 경로

`text_style`의 축 글줄 높이 → `automatic_scale`의 축 간격 예산 → 최종 plot 높이와
`render_value_grid`의 라벨 생략 간격 → 실제 숫자 방출을 연결한다. 측정이 이미 사용하는
1.3em을 공통 helper로 만들고 실제 표시도 소비하게 한다. 원점·축 범위·눈금·글꼴·제목과
별도3D 투영 경로는 변경하지 않는다. 수평 축의 기존 숫자 선택과 정상 대조군도 확인한다.
현재 코드·독립 Print·확대 판독·새 head의 Native/fresh 결과를 연결한 뒤 회귀를 추가한다.

## 별도로 배제한 가설과 진행 중인 검증

기본 외곽선은 명시0.75pt Print 대조로 단위 오류를 입증해 `11767e35f3`에서 보정했다.
Native074는268.40초에 성공했고180개 기록 소스가 그 커밋과 같음을1000에 보존했다.
그 제품의180조건 Native 검증은160조건 통과·20조건 미달로 종료했다. 이전 제품 fresh027의176조건은134조건 통과·42조건 미달로 종료했다.
1009의 같은180조건 Native073/074 대조에서0.1%p 이상 하락은 없었다. 서로 다른 소스의 결과를 이 단계의 최종 검증으로 합치지 않는다.

표식 크기를 pt로 곱하는 가설은 적용하지 않는다. 선언 단위는
[OOXML 사양](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.charts.size?view=openxml-3.0.1)
상 pt이나, 기존759/764의 같은 한컴 Print는2/7/14 변경에도600dpi 차트 전체 픽셀이 같았다.
786의 한컴 재저장 XML에도14가 남았으므로 저장과 실제 소비를 구분한다. 해당 증거를
재확인했고 중복 Print 요청이나 파서/표식 반경 변경을 하지 않았다. 자동 삼각 표식의 실제
모양·그리기 순서 차이는 별도로 남아 있으며 점수 통과로 확대 판독 완료로 바꾸지 않는다.

후속 WASM028 자동 빌드는 숫자 간격 보정을 함께 검증하도록 보류했다. Native074 빌드는
중단하지 않고 성공 제품·소스를 보존한 뒤 대기용 orchestration만 종료했다. 해당 종료143은
제품 빌드 실패가 아니다. 실제 빌드 exit0은 receipt로 확인한다. 전체 필수 gate가 끝나기 전
PR·push·remote review를 수행하지 않는다.

현재 근거는 `chart-axis-label-spacing-controls-1003/manifest.json`,
`mcp-chart-axis-label-spacing-controls-1003/*-complete.json`,
`chart-axis-label-layout-1005/measurements.json`, `chart-axis-label-bands-1006/measurements.json`이다.
로그는 `logs/chart-axis-label-spacing-1003-{generate,print}.log`,
`logs/chart-axis-label-layout-1005.log`, `logs/chart-axis-label-bands-1006.log`,
`logs/chart-axis-spacing-native-before-1007.log`다. 최종 whole-page TSV와 직접 판독을 대신하지 않는다.


## 경계 검증과 공통 글줄 높이 구현

1011의 추가 세 입력은226·227·228 높이이며 다른 값·서식·호스트는 보존했다.
세 공식2020 Print 모두0·6을 표시했다. 1012의 실제 Native074 SVG는 모두0만 표시했다.
1013의 독립 숫자 밴드는 축 높이44.08/45.04/46.08과 연결했다. 230의0·4와 함께 확인하면
1.2em/1.25em 또는2em의 단순 반올림으로는 이 경계를 설명하지 못한다. 측정에 이미 쓰는
1.3em의 글줄 점유는 여섯 입력을 모두 설명한다. 인접 입력을 제외하고 단일 점수에 맞추지 않았다.

`text_style::axis_line_height`를 공통 생산자로 만들었다. `automatic_scale`은 기존과 같은
글줄 높이를 이 helper에서 받고, `render_value_grid`도 숫자 생략 간격을 같은 값으로 정한다.
측정의 간격 예산·nice 단위 선택은 변경하지 않았다. 실제 표시에서만 잘못된 별도2em
점유를 제거했다. 빈 예산과 최소0 라벨의 기존 guard 및 수평 축 분기는 유지한다.

1008의 fmt·release-test 차트 crate 빌드·전체 target Clippy는 통과했다. 기존 검사는
157PASS/8FAIL로 같은 여덟 실패다. 새 회귀·기대값·골든은 추가/갱신하지 않았다.
1010의 모듈73조건은 실제 동결 측정 tuple로 실행했고 미등록 tuple의 추정 폭 fallback은 없다.
그 모듈을 다시 컴파일하지 않고1016에서 추가 경계 세 입력에 재사용했다. 총76개의 모듈
실행이며, 이 사실만으로76조건의 한컴 시각 일치를 주장하지 않는다.

1016의 여섯 숫자 계약은 실제 Native074에서 모두 불일치했고, 새 모듈의 실제 방출에서는
모두 독립 Print와 일치했다. 이는 정식 Rust 회귀나 새 Native/fresh 전쪽 gate가 아니다.
1015의 실제 전쪽 Native074 review·standalone overlay를 함께 열어97.05197%여도 PDF의
4가 빠진 모습을 직접 확인했다. 1014의 잘라낸 모듈 진단과 전쪽 증거를 구분한다.

새 원본·독립 PDF 전체 조건은 기존180개와 새 높이6개를 합친186개다. 소스 단계 커밋 뒤
Native075와 같은 소스의 최적화 fresh028에서 한꺼번에 재검증한다. 미달20조건·자동 삼각
표식 판독·기존 Rust 실패와 전체 통합 검증은 계속 해결하며 이 중간 결과로 PR을 제출하지 않는다.

추가 근거는 `chart-axis-label-boundary-controls-1011/manifest.json`,
`mcp-chart-axis-label-boundary-controls-1011/*-complete.json`,
`chart-axis-label-layout-1012/measurements.json`, `chart-axis-label-bands-1013/measurements.json`,
`chart-axis-label-contract-1016/measurements.json`, `chart-axis-native-before-panels-1015/`,
`chart-axis-line-height-1008-validation.json`, `chart-legend-native073-native074-1009.json`이다.
추가 로그는 `logs/chart-axis-label-boundaries-1011-print.log`,
`logs/chart-axis-label-bands-1013.log`, `logs/chart-axis-label-contract-1016.log`,
`logs/chart-axis-native-before-panels-1015.log`, `logs/chart-axis-line-height-1008-*.log`다.
