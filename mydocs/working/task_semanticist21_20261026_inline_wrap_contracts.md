# 어울림 표의 흐름 전진과 양식 개체의 글줄 메트릭

## 독립 근거와 반례

기존 #8045·#8093 회귀의 현재 helper로 입력을 다시 내보내고 같은 바이트를 한컴
2020 MCP의 Print 경로로 출력했다. 실행·입력/PDF 해시는 ignored 증적 루트
`output/pr-review/semanticist21-20261019/`의 `fixture-source-equivalence-4028.json`,
`mcp-current-regression-4029-result.json`에 있다. 과거 fixture와 해시가 달라 과거
PDF를 재사용하지 않았다.

#8045의 수동 LineSeg는 한컴 저장본과 다르다. 같은 재현본을 한컴에서 저장한 HWPX와
그 저장본의 독립 Print를 별도로 확보했다(`mcp-square-normalization-4035-result.json`).
원래 합성 입력을 정상 저장본과 같은 입력이라고 기록하지 않는다. 한컴 저장본의
메모는 `horzpos=20100`, `vertpos=1600`이며, 수동 원본은 각각 21000, 300이다.
현재 Native253/fresh WASM072에서 저장본의 메모 원점은 맞지만 바깥 표가 약 21.33px
높다. 양쪽 raw가 90% 미만이므로 회귀 fixture 교체를 먼저 하지 않는다.

원인 경로는 `unabsorbed_nested_table_paragraph_height`의 어울림 표 전체 높이 가산
→ `compressed_monotonic_ladder`의 압축 판정 → 부모 행 높이 증가다. 실제 배치는
`nested_table_flow_advance`에서 저장 앵커 줄만 전진시킨다. 어울림 표의 보이는 높이와
본문 전진을 같은 상태로 취급한 측정 가정을 제거한다. 앵커 줄이 없는 입력의 기존
표 높이 전진과 자리 차지 표·글자처럼 취급 표의 계약은 유지한다. 시각적 개체 하단은
기존 `cell_nested_controls_bottom`의 합집합으로 계속 보호한다.

#8093의 한컴 저장본은 혼합 `가[단추]나` 줄의 높이 1984HU, 기준선 1686HU를 기록한다
(`mcp-form-normalization-4037-result.json`). 원본 개체의 높이와 85% 기준선이며,
전체 높이를 기준선으로 쓰는 계약이 아니다. 현재 Print 대조는 raw가 90% 이상이어도
옆 글자의 기준선이 약 11px 높다. `compose_paragraph_in_context` → 공통 실제 줄
메트릭 갱신 → 측정·최종 `paragraph_layout`의 기준선 소비를 대조한다. 정상 저장 줄은
덮어쓰지 않고, 저장 줄 부재·합성 줄에서 같은 줄의 글자와 양식 상자 점유를 합친다.

## 구현·검증 계획

어울림 저장 앵커의 전진 질의를 공통 helper로 분리해 측정과 실제 배치가 소비하게 한다.
양식 개체도 공통 인라인 분류와 재생성 글줄의 높이·기준선 합집합에 참여시킨다. 문서별
수치 보정이나 기존 오차 완화는 추가하지 않는다. 기존 99건과 두 재현의 관계·caret
검사, 같은 입력의 Native/fresh WASM Print 대조를 먼저 검증한 뒤 회귀를 교정한다.
그다음 기존 17조건 전쪽과 base 대비 0.1%p, 필수 전체 검증을 진행한다.

sample16·편람 삭제는 유지한다. 해당 입력·검사·이전 파생 참조는 생성하지 않는다.

## Native254 선행 실행과 남은 소비 경로

Native254 빌드는 255.85초·exit 0으로 종료했고 빌드 중 소스 해시는 변하지 않았다.
기존 99건과 #8045·#8093의 4건, 총 103건이 모두 통과했다. 같은 재현의 한컴 저장본은
raw 54.70424%에서 100%가 됐으며, review·overlay에서 부모 표 높이와 메모 원점을
확인했다. 수동 LineSeg 원본은 93.69297% 그대로이며 정상 저장본으로 바꿔 보고하지 않는다.

Form HWPX는 raw 94.52700%에서 100%로 개선됐고 본문 기준선도 직접 확인했다.
HWP는 94.80944%로 그대로다. 원본을 직접 열어 확인한 결과 HWP에는 높이 1000HU,
기준선 850HU, 구현 속성 태그 `2147876864`의 합성 줄이 남아 있고 HWPX는 줄이 없다.
구성 결과만 갱신해도 HWP의 이후 소비자가 원본의 작은 줄을 다시 읽는다. 기존 단일
인라인 host 투영에 Form을 포함해 저장 줄 소비자까지 같은 1984HU 상자를 전달한다.
셀 내부의 해당 공통 투영 진입점도 Form을 인식하게 한다. 정상 높이를 이미 담은 줄은
확장하지 않는다.

어울림 측정의 가산 생략은 정상 저장 줄이 있는 경우로 한정한다. 합성 줄·NO_LS의
기존 흐름 폴백을 저장 근거로 승격하지 않는다. 이 보완과 Form host 투영은 Native254
실행 뒤의 변경이므로 새 Native 및 fresh WASM에서 다시 검증한다.

증적은 `rebase-native254-build-4039-result.json`, `inline-wrap-native254-4040-result.json`,
`logs/inline-wrap-focused-4040.log`, `logs/form-source-metrics-4041.log`와
`inline-wrap-native254-4040/`의 조건별 review·overlay·overlay_metrics.json이다.
