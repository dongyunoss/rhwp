# #7559 현재 API의 계산 빈 호스트와 표 프레임

## 원 해결 추적과 독립 근거

현재 API의 새 HWP/HWPX4개를 같은 한컴2020·11.0.0.9136·PrintMethod0·1-up으로 인쇄했다.
영수증은 ignored `output/pr-review/semanticist21-20261019/mcp-save-origin-originals-475`에 있다.
원 API 테스트의 입력 구성은 바꾸지 않았으며 native HWP와 adapter HWP 바이트가 같음을 확인했다.
현재 HWP의 live/재열기 table top은176.2666667, HWPX는180.04다. 독립 Print는 두 형식의
표 테두리가 같은 위치다. Native049의 HWP raw5.24242%, fresh023의 HWP5.27488%이며
같은 입력 HWPX와 padding 대조는 양쪽100%다. 이 수치는 보정 전 결과다.

원 #7559는 #7265를 참조하며 실제 메인터너 통합 #7430의 위 여백 계약을 유지해야 한다.
#7538의 원점 논의와 실제 후속 보정도 읽었다. #7538 자체는 closed·unmerged다.
추가 원인인 `reflow_empty_table_host`의 LineSeg 계약은 원 #7518→통합 #7570의
`3d9239eeec`→`a9dfc4e664` 보정까지 추적했다. 계산 줄을 저장 줄 부재로 간주해 없애면
원래 보정의 줄 점유 계약을 깨뜨린다. #7601의 `0a305d51a8`은 입증된 저장 원점만 수용한다.
`generated-host-original-pr-trace-481.json`과 기존 `pr_7518_review_impl.md`에 근거가 있다.

## 값의 생산과 소비

- 입력의 계산 LineSeg는 synthetic tag를 유지한다. HWPX writer는 이 줄을 저장 사다리로 내보내지 않는다.
  이전 fixture012와 현재 public API 모델의 차이도 이 tag다. 이전 저장본100%를 현재 실패의
  해결 증거로 재사용하지 않는다.
- `host_spacing::resolve`가 이미 위·아래 바깥 여백을 산출한다. `FormattedParagraph.total_height`는
  계산 줄의 실제 format 결과다. 줄이 보이지 않는다고 공간을0으로 취급하지 않는다.
- `block::computed_host_frame::resolve`는 빈 글줄과 표 하나가 같은 호스트에 속한 경우만,
  같은 원점에서 표 앞 여백·양수 오프셋·표 높이·뒤 여백과 계산 줄 점유의 합집합을 만든다.
- `block::entry`의 기존 명시 저장 프레임과 NO_LS 계획이 없을 때 이 결과를 수용한다.
  기존 HostPlacementConstraint가 앞 형제·점유 띠를 적용한 뒤 같은 상자를 fit에 사용하고 기록한다.
- `place_table_with_text`는 기록한 occupied_bottom으로 흐름을 전진시키고,
  layout의 resolved_table_origin은 같은 table_top을 쓰며 별도 offset·여백을 재계산하지 않는다.
  뒤에서 원점 덮어쓰기나 paint-only 이동을 추가하지 않는다.

실제 저장 줄, visible/공백 호스트, 둘 이상의 줄 참여 개체, 음수 오프셋·절대 기준·다른 세로 정렬은
이 새 경로에 들어오지 않는다. 구역·단 설정과 줄에 참여하지 않는 그림·도형은 기존 공통
`control_leaves_table_anchor_empty` 판정을 공유한다. 문서 ID·좌표 상수·row 수 조건은 없다.
계산 줄의 공간을 보존하므로 #7518의 `reflow_empty_table_host` 수용 조건은 바꾸지 않는다.

## 검증 순서

1. Native050 동결 빌드 뒤 실제 원 API2검사와 현재 API4입력의 전쪽 Print 대조를 먼저 확인한다.
2. 같은 입력의 fresh WASM 전체 페이지와 직접 review/overlay 판독을 완료한다.
3. 양쪽90% 이상이 확인된 뒤 원 API 저장 전후 불변 검사와 줄·표·다음 내용의 관계 검사를 보강한다.
4. 원 #7518 계산 줄, #7430 위 여백, 기존 형제 표·분할·positive offset 대조와 #8064를 함께 검증한다.
   원본 저장 줄의 처리는 변경하지 않는다. 넓은 전체 회귀는 영향 경계 확인 이후 수행한다.

아직 수정 후 빌드·시각 검증 전이며 PR 제출 조건이 충족된 상태가 아니다.

## Native050 관측

공유 target·잡8개 Native 빌드는236.07초에 exit0으로 끝났다. 원 API 입력 구성과 원점 기대값을
바꾸지 않은 `table_hwpx_save_origin_contract`의2검사가 모두 통과했다.
현재 API의 HWP adapter/HWPX·offset/padding4개 새 Print 대조도 각 전체1쪽 Native100%다.
수정 전 HWP offset5.24242%에서100%로 개선됐다.

strict rustc로 실제70검사를 실행해0FAIL을 확인했다: 원점2, 보충 측정21, 세션11, 강조점3,
형제 표7, 원 #7518 계산 줄·물리 프레임22, 저장 도형 호스트4다. 원 #7518의 LineSeg 수용 조건과
검사 기대값은 변경하지 않았다. 전체 Cargo 회귀 완료로 표현하지 않는다.
`char-overlap-native-build-050-result.json`, `emphasis-metric-current-failure-focus-484.json`,
`save-origin-current-api-488-computed-host-fixed-native/results.json`에 실제 명령·SHA·결과를 보존했다.
fresh WASM, 직접 판독과 넓은 대조는 다음 검증 단계이며 아직 전체 PR gate가 통과한 상태는 아니다.
