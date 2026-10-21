# 묶음 그림 외곽선의 공통 프레임과 독립 Print

`e787be830763d871703e025aa7adae499f954618`의 Native082·최적화 fresh WASM033에서
같은 원본·독립 PDF를 검증했다. 입력·PDF·TSV·PNG SHA와 실제 명령은
[provenance.json](provenance.json)에 있다. 원본과 PDF는 이미 추적하는 아래 파일을 재사용했다.

- [원본 HWP](../../../../samples/2025%20행정업무운영%20편람(최종).hwp)
- [한컴2024+KoPub Print PDF](../../../../pdf/2025%20행정업무운영%20편람(최종)-hwp-kopub-2024.pdf)
- [Native 전체383쪽 TSV](native_silhouette.tsv) / [fresh 전체383쪽 TSV](wasm_silhouette.tsv)

양쪽 전체383쪽의 PDF 쪽수가 같고 누락·90% 미만 쪽은 없다. 전쪽 raw 최저는
Native92.66367%, fresh92.65487%다. 94쪽 raw는Native98.64499%, fresh98.64029%이며
색상 경계 보정 전2px 실루엣 값이다. 엄격 RGB 일치율로 해석하지 않는다.

| 실제 출력 | review | standalone overlay |
| --- | --- | --- |
| Native082 94쪽 | [review](native_review_094.png) | [overlay](native_overlay_094.png) |
| fresh033 94쪽 | [review](wasm_review_094.png) | [overlay](wasm_overlay_094.png) |

원문 구역2 문단657·661의 묶음 자식 Picture는 실선을 갖지만 이전 경로는 bitmap만
추가했다. 공통 그림 테두리 모듈이 최종 ImageNode의 좌표·변환 프레임을 소비하도록
연결했다. 그룹에 이미 적용한 원본 회전·대칭을 다시 적용하지 않는다. crop·선 종류·
기본 물리 두께·쪽 수용 조건은 바꾸지 않았고 문서별 예외나1px 최소선은 추가하지 않았다.

같은 배율의 독립 Print·Native·fresh에서 두 그림의 닫힌 외곽, 기존 빨간 주석 상자의
쌓기 순서와 주변 본문을 직접 판독했다. 얇은 선의 raster 농도 차이는 잔여 차이로 기록한다.
독립 Print의 실제 vector 테두리 경로 bounds를72pt→96DPI로 환산한
[independent_coordinates.json](independent_coordinates.json)을 사용한다. 엔진 좌표를
기대값으로 복사하지 않았다. 94쪽의 양쪽98%·직접 판독 조건에서만 ±2px 좌표를 고정한다.
문서 전체98% 조건으로 확대하지 않으며, 이후에도 이쪽은 양쪽98% 이상을 유지해야 한다.

새 [정식 회귀2함수](../../../../tests/cases/group_picture_outline_frames.rs)는 원문 bin-data
소유로 그림과 묶음을 찾는다. 외곽선 한 개·그림과 같은 틀·bitmap 바로 위의 순서·기존
주석의 보존과 뒤 쌓기 순서를 검사하고 독립 PDF bounds의 ±2px를 함께 보호한다.
같은 최종 정식 소스를 Native081/082에 strict 컴파일해 이전2FAIL·보정 뒤2PASS를
확인했다. 두 build exit0이고 이전 실패는 외곽선 누락이다. 이전1255의 runner 인자
오류를 제품 재현으로 세지 않았으며 수정한1257의 실행과 test source SHA를 보존했다.
새 source의 자동 suite 수집·고정 devel 기준 manifest·unit-tier 검사도 통과했다.
정식 자동 수집 경로 `regression_suite_014`의 nextest 실행도 2PASS·0FAIL이다.
전체62조건860쪽 Native/fresh 비교에서 고정 devel 대비0.1%p 이상 하락은 양쪽 모두0쪽이다.

[기존 TAC·선 경계 증거](../stored_tac_source_frame_20261021/README.md)와 함께 읽는다.
111/242쪽의 기존 독립 좌표·내용 관계 검사와 crop·회전·그룹 해제·활성 셀·편람 경계
24검사는 Native082에서도 통과했다. 이 폴더의 완료를 전체45개 PR 통합 검증·CI·승인으로
표현하지 않는다. 전체 통합의 다른 실패와 차트·원 PR별 검토는 계속 진행한다.
