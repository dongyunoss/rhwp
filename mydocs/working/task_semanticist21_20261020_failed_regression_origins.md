# 전체 회귀 실패의 원 해결 PR 추적

## 2026-10-21 전체 재검증과 저장 줄 투영 경계의 보정

`775de4c16fa2cc5e36fc588e7f500625e7a4939c`의 전체 Rust1085는
11,183개 실행 중 11,121통과·62실패·45skip, exit100으로 종료됐다. 실행 중 소스
3,819개의 변경은 없었다. 과거390의57실패와 함수 이름으로 대조하면20개는 통과로
바뀌었고37개는 남았으며25개가 새로 실패했다. 새 실패를 최신 devel의 회귀라고
자동 판정하지 않는다. 원문은 ignored output의 `full-rust-failure-inventory-1092.json`,
`logs/full-rust-regression-1085.log`에 있다.

### #7216의 수정 전 입력과 실제 Print

원 이슈 #7216 → 기여 #7233 → 통합 #7241과 기존 `pr_7233_review.md`를 다시
대조했다. 원 수용 범위는 scaffold의 IR/HWPX/HWP5 나눔 속성과 한컴의60행 보존이다.
수정 전 `tall_table_before.hwpx`는 한컴에서도54~60행이 잘리는 음성 대조군이다.
네 원본을 새 한컴2020 `11.0.0.9136`·PrintMethod0·1-up으로 모두 출력했다.
입력/PDF SHA와 job 영수증은 `mcp-issue7216-current-1091/`에 보존했다.

| 원본 | Native075 최저 raw | fresh028 최저 raw | devel 인쇄 경로 Native 최저 raw |
| --- | ---: | ---: | ---: |
| short_table_before | 97.32824% | 97.29615% | 97.32824% |
| short_table_after | 34.95968% | 35.42245% | 34.95968% |
| tall_table_before | 99.11035% | 99.07258% | 0.95161% |
| tall_table_after | 99.52832% | 99.68207% | 99.52832% |

현재 소스의 최종 판정으로 취급하지 않는다. Native075/fresh028의 실제 생산 소스는
`a19d7c3d164807d69ef85b40a75908957cb0b2d8`이며 최신 HEAD의 최종 gate가 아니다.
현재 셀 밖21행은 수정 전 입력의 정상적인 종이 경계 잘림과 대응하지만, 짧은 after의
위치 차이가 미해결이므로 원장 기대값은 변경하지 않았다. 전체16실행의 원문은
`issue7216-current-print-validation-1094/results.json`에 있다.

### #7751의 실제 회귀와 공통 원인

같은 원본과 독립 KoPub 한컴2024 Print의383쪽을 대조했다. PDF 출처나 기존 검사
383쪽·±2px 기대값을 바꾸지 않는다. base의 dump-pages는383쪽이고 Native075와
현재 default-feature 제품은 모두384쪽이었다. 보존 제품052/054의 실제 소스 SHA를
대조하면 첫 차이는289쪽 section10 para28의 분할이다.052의 컷은 `[1,1,10]`,054는
`[1,1,9]`이고 아래 여백566HU의 추가 예약으로 후속 내용이 이월됐다.

| 대조 | 289쪽 | 290쪽 | 전체 쪽수 |
| --- | ---: | ---: | ---: |
| Native052 | 99.98063% | 98.24914% | 383 |
| Native075 | 89.20221% | 37.89354% | 384 |

`queries/rendering.rs`의 inline frame 정규화는 앞 개체의 높이 차이를 뒤 저장 줄의
vpos에서 빼고 구현 태그를 붙인다. 원본 para27/28의4942/20407HU와 실제 출력
3276/18741HU의 차이는 모두1666HU다. 이 좌표 투영을 `replace_line_segs`로 발행해
`computed_empty_table_host`가 구현 태그만 보고 새로 계산한 빈 줄로 오인했다.
이어 `computed_rowbreak_owns_outer_frame`에서 분할 예산과 부분 배치로 전달되어
원래 저장 프레임이 소유하는 아래 여백을 다시 예약했다.

기존 `publish_object_projection`으로 확정 줄의 소유를 발행하고 공통
`computed_empty_table_host`가 소유된 투영을 새 계산 줄로 취급하지 않게 보정한다.
원본·저장 출력은 바꾸지 않으며 문서명·임의 좌표에 따른 예외도 추가하지 않는다.
이 절은 구현 전 독립 근거와 적용 계획이며 보정 뒤 빌드·기존 회귀·Native/fresh 전쪽
검증·확대 판독은 계속 필요하다. 실행 자료는1096~1103에 보존했다.

### 보정 commit96038d8647의 실제 재검증

Native076은 공용 `target/pr-review`, build jobs8로237.32초에 빌드됐다.
실제 생산 소스는 `96038d8647f4e4c47e38d43b6f99bf995234bfd5`이며 파일별 SHA와
binary/rlib 해시는 `char-overlap-native-build-076-result.json`에 있다.
기존17모듈104검사는 모두 통과했다. 여기에는 편람의 완료 페이지6검사, 원 선언-fit,
표 조각·미주, 보충 메트릭, 저장 원점, 계산 빈 호스트, 양식 두 대조가 포함된다.
검사·기대값·golden은 바꾸지 않았다. 전체 Rust1085의62실패를 전부 해결했다는 뜻이 아니다.

편람은383쪽으로 복구됐고289쪽 컷도 `[1,1,10]`이다. 같은 독립 PDF와 새 Native PNG의
289/290쪽 raw는99.94445%/98.24914%다. `handbook-native076-panels-1111/`의
review와 standalone overlay를 직접 열어 Q14의 중첩 표·화살표·뒤 조각 소유를 확인했다.
글자·얇은 선의 농도 잔차는 실제 패널에 보존하며 완전 픽셀 일치로 보고하지 않는다.

편람 전383쪽 Native TSV에서는381쪽이90% 이상이고111쪽81.91890%,242쪽64.34522%가
남았다. 전체 쪽수는 PDF와383=383이다. 따라서 문서 전체는 아직 통과하지 않았다.
미달 두 쪽도 새 review/overlay로 직접 확인했다.242쪽은 제목 표와 뒤 본문 시작점의
세로 차이가 보인다. 최신 고정 devel의 같은 전383쪽·글꼴·PDF 대조1113을 시작했으며,
base의 같은 미달인지 현재 수정의 회귀인지는 그 결과로 구분한다. 기대값은 완화하지 않는다.

Native39조건449쪽 재출력1109와 같은 소스의 최적화 fresh029 빌드·39조건 재검증1115를
진행 중이다. 전쪽·fresh·최신 devel 무회귀·직접 판독을 모두 충족한 뒤 판정을 갱신한다.
로그는 `logs/stored-projection-focused-1107.log`,
`logs/projection-native076-1109-handbook-full.log`,
`logs/char-overlap-fresh-wasm-build-029.log`에 보존했다.

### 저장 TAC 원점과 앞뒤 문단 간격의 독립 근거

실제 해결 계보는 원 #7494의 직접 PDF 기여 → 통합 #7964이며, #7751의 최종 댓글은
같은 한컴2024+KoPub383쪽 Print를 기준으로 종료했음을 확인한다. 옛384쪽 본문 기록과
혼동하지 않는다. 현재 Native base도111/242쪽이81.91890%/64.34557%로 미달이고
Native076의 전383쪽은 base 대비0.1%p 하락0이다. 결과는1118에 있다.

1124의 원본 파서/DocumentCore 직접 조회에서 두 쪽의 `source_line_seg_vertical_pos`는
없고 원본 vpos가 그대로 보존된다. 따라서 원본 좌표 손실 가설은 채택하지 않는다.
1125의 실제 호출 진단은242쪽 제목 표의 원본 원점353.92px를, 직전 빈 줄의 흐름 끝에서
역산한343.94667px로 덮어쓰는 경로를 보인다. 두 값의 차이9.97333px는 독립 PDF의
제목·뒤 문단 위치 차이와 대응한다. 원본 높이와 실제 표 측정은 모두3118HU로 일치한다.

기존 `95ab7fdb23`과 그 작업 기록187을 읽었다. 이 변경은 실제 직전 내용 끝과 원본 상대
간격의 소유를 공유하려는 것이다. 상대 간격은 실제 흐름의 하한을 정하지만, 그것이 더
위라는 이유로 별도로 소유된 원본 개체 원점을 삭제할 근거는 아니다. 원본의 옛 vpos가
새 단을 가리키지 않는 경우는 기존 `restarted_column_line`/`continued_column_line`의
독립적인 후속 줄 등식으로 먼저 해소한다. 그 뒤 남은 원본 원점과 실제 점유 하한을
함께 소비하며 새 문서별 예외나 목표 y 보정은 넣지 않는다.

111쪽 첫 표는 비흐름 장식과 같은 문단의 한 TAC 줄이다. 원본 줄은5208HU로 표 본체
4928HU와 위·아래 여백각140HU를 정확히 소유한다. 다음 줄 vpos37944는 현재29996+
높이5208+뒤 줄간격1540+다음 문단 앞 간격1200HU로 닫힌다. 기존 장식 경로는 다음
문단 앞 간격이0이어야 한다는 조건 때문에 이 유효한 등식을 버렸다. 비흐름 장식을
흐름 소유자로 만들지 않고 같은 닫힌 등식에 문단 간격도 포함해 수용한다. fit과 실제
배치는 같은 원점·표 여백·줄 끝을 계속 소비한다.

이 장식 동반 줄은 단 첫 저장 줄의 영점까지 검증했으므로 임시 lazy 텍스트 기준축을
개체 원점에 빼지 않는다. 같은 `StoredColumnFrame`을 inline placement에 실어 뒤 문단의
축 확정과 실제 paint도 영점을 재사용하게 한다. 계산 helper 뒤의 별도 원점 선택으로
검증 결과가 덮이지 않는지 기존104검사와 전쪽 출력에서 함께 확인한다.

이 두 보정은 Native077에서 기존 집중104검사, 편람 전383쪽과 같은 표·양식 대조,
실제 변경 전에 이미 검증한 언어 원본과 #8064 보존 범위를 대조한다. fresh029는
96038d8647 제품의 기존39조건을 계속 확인 중이며 이 새 소스의 결과로 바꾸어 쓰지 않는다.
새 회귀·기대값 변경은 전쪽 Native/fresh gate와 직접 판독 뒤에만 판단한다.

### Native077의 독립 좌표 대조와 위 여백의 단일 소비

Native077의 기존 집중 17모듈 104검사는 기대값 변경 없이 모두 통과했다.
`logs/stored-tac-frame-focused-1129.log`와 `emphasis-metric-current-failure-focus-1129.json`에
정식 검사 소스 SHA·명령·결과를 보존했다. 전쪽 Native40조건과 앞 제품 fresh029의
39조건 검증은 별도이며 이 집중 통과를 전체 완료로 취급하지 않는다.

1132의 실제 render tree와 독립 PDF 선 좌표를 같은 96DPI 종이 원점으로 대조했다.
242쪽 제목 표는 y491.3600에서501.3333으로 이동해 PDF501.2310에 대응했고,
111쪽 뒤 두 표는695.4667/804.1467로 PDF695.2920/804.0814에 대응했다.
첫 표는551.0933 대 PDF549.0667로 위 바깥 여백140HU가 중복 소비된 차이가 남았다.
원본 첫 줄29996HU·body147.4133·위 여백140HU의 독립 합은549.2267이다.
진단 원문은 `tac-frame-output-comparison-1132.json` 및 두 제품 실행 로그에 있다.

실제 소비 경로는 `prepare_coanchored_first_line → InlineBoxPlacement.y →
layout_table_control_block`이다. 마지막 소비자가 위 바깥 여백을 한 번 더해 테두리
좌표를 만든다. 일반 물리 TAC 생산자와 원 통합 #7964도 pen만 전달한다.
따라서 공통 coanchor 생산자의 y를 원본 줄 pen으로 전달하고 top margin을 미리 더하지
않도록 맞춘다. 문서 ID·좌표 상수·허용치 추가는 없다. 같은 metadata를 소비하는
장식도 같은 pen을 받으며, 후속 end와 저장 축은 그대로 유지한다.
Native078에서 기존104검사·독립 좌표·영향 페이지 및 전쪽 대조를 확인하고,
같은 제품의 fresh WASM을 산출한다. 새 테스트·fixture·golden은 아직 추가하지 않는다.

### Native078 결과와 확대 판독에서 남은 화살표

제품 `9810f26ccfbf9073679a7b79c3cfa5ece18905b6`의 Native078 빌드는226.18초·exit0이다.
동일한 정식 검사17모듈104개가 모두 통과했다. 1137 실제 render tree는111쪽 첫 표
원점549.2267, 뒤 표695.4667/804.1467,242쪽 제목501.3333을 확인했다.
독립 PDF와의 차이는 각각 약0.16/0.17/0.07/0.10px다. 기존 기대값은 바꾸지 않았다.
Native078 편람 전383쪽은 최저92.44708%,111쪽99.86423%,242쪽98.84506%다.
같은 제품의 표·양식·언어 원본을 합친40조건과 fresh030의 최종 검증은 별도로 진행한다.
Native077의 전383쪽 devel 대조1140에서는0.1%p 이상 하락0쪽이었다.

1138에서 Native078의111·242쪽 review와 standalone overlay를 모두 직접 열었다.
표와 제목의 원점은 개선됐지만111쪽 마지막 상자의 화살표는 독립 Print의 위 방향과 달리
오른쪽을 향한다. 99% 점수로 이 차이를 면제하거나 해당 쪽의 확대 판독 완료를 선언하지 않는다.
새 정식 좌표 회귀는 아직 추가하지 않는다.

1143의 실제 SVG 진단은 셀 안 선 끝점 `(232.9533,828.0933) → (234.2867,849.6933)`에
추가 `rotate(90,233.62,838.8933)`가 적용됨을 확인했다. 원 처리 #7333 → #7377의
`c84a8f8e1a9`와 기존 개별 review의 연결선 방향·marker·선택 계약을 읽었다.
원본2.841 첫 셀의 직선이며1144 원문 dump와 조회용 RHWP HWPX export를 보존했다.
이 export는 원본 속성을 조사하는 진단 자료이며 한컴 원본이나 독립 Print를 대체하지 않는다.
최초 CLI export의 `-o` 옵션 오류는 exit2로 보존했고 positional 출력 경로로 다시 실행해
성공했다. 진단 export의 rotation90, current100×1620, 끝점0/0→100/100과 축척 행렬은
독립 Print의 실제 방향과 함께 대조하며, 전체 선 회전을 일괄 끄는 식으로 보정하지 않는다.

## 범위와 증거

전체 Rust390의 `5d736f74ca` 실행은 11,098통과·57실패·45skip으로 종료됐다.
실패 요약에 같은 이름이 다시 출력된 114줄을 114건으로 세지 않는다. 57검사·29모듈이다.
현재 작업은 `d26f6aef06`의 공통 보정까지 진행됐으며 전체 회귀를 다시 통과한 상태가 아니다.

ignored `output/pr-review/semanticist21-20261019/`에 다음 원문을 보존한다.

- `failed-test-origins-419.json`: 57검사의 실제 소스, 도입 커밋 검색, GitHub 연결 PR 응답.
- `original-cherry-pick-prs-423.json`: 도입 커밋 본문의 원 cherry-pick SHA를 별도로 추적.
- `pr*-original-history-420.json`: 아래 원 PR의 본문·comment·review·실제 변경 파일.
- `issue-*-timeline-404.json`, `pr-*-history-407.json`, `pr-*-history-408.json`:
  앞 단계에서 확인한 원 이슈와 해결·통합 PR.

GitHub의 commits/pulls 연결은 해당 커밋을 나중에 포함한 통합 PR도 반환할 수 있다.
또 fork 원 SHA나 로컬 cherry-pick SHA에는 연결 PR이 비어 있을 수 있다. 연결 목록만으로
직접 해결 PR을 단정하지 않고 원 본문·diff·통합 기록과 대조한다. #7474 같은 포괄 연결을
개별 결함의 해결 근거로 대신하지 않는다.

## 확인한 해결 의도와 현재 처리

| 대상 | 원 해결·통합 | 실제 해결 의도 | 현재 처리 |
| --- | --- | --- | --- |
| #7084 | [#7119](https://github.com/edwardkim/rhwp/pull/7119), merge `922946438f` | 같은 세대·서식의 실제 backend 전진을 사용하고 다중 scalar 자소의 부분 적용·다른 출력 경로 누출을 막음 | 수집과 소비의 자격 조건을 공유해 다른 face의 임시 폭보다 유효한 실제 측정을 우선. `d26f6aef06`, 기존 실패16건 중14건 해소. 나머지2건 기대값은 변경하지 않음 |
| #7158 | [#7163](https://github.com/edwardkim/rhwp/pull/7163), merge `da7ec5c090` | 표 옆 이어지는 같은 문단의 줄과 뒤 문단을 동일한 이동 앵커로 배치 | 실제 원본·Print와 현재 줄 소유·앵커를 대조한 뒤 구현 판단. 저장값 assertion 삭제로 끝내지 않음 |
| #6448 | #6472 → [통합 #6480](https://github.com/edwardkim/rhwp/pull/6480), merge `19b89d967b` | 통합 보정에서 일반 경로를 복원하고 검증한 빈 host·3행1열의 물리 밴드와 뒤 간격을 처리 | 원 contributor 설명만 재적용하지 않고 통합 보정까지 유지·대조 |
| #5819 | [#7068](https://github.com/edwardkim/rhwp/pull/7068), merge `3f34869b9c` | 열 너비·정렬·field 위치 옵션과 한컴 표 생성 기본값, 두 형식의 속성 보존 | 현재 CLI --verify의 차이12건을 모델·저장 경로에서 조사. 검증 허용치 완화하지 않음 |
| #6624·현대 OOXML 차트 | [#6626](https://github.com/edwardkim/rhwp/pull/6626) → [통합 #6645](https://github.com/edwardkim/rhwp/pull/6645), merge `2edbe62e5dc74db58c33df2c006ae587f86a1a71` | XML에 명시한 글꼴·선·축·gapWidth를 읽고, 없는 제목14pt·라벨10pt 및 자동 배치를 보충. 통합은 wrapper padding의 저장 높이 계약도 보정 |575~576에서 원 이슈·PR·통합 본문과 실제 cherry `b1091864dffee8f8041e5b235d9ab9491ea41c04`를 재대조. 당시 IoU·프로파일·fidelity 차이 지표는 현재 raw90% 증거가 아님. #8064의 legacy OLE/WMF 차트와 구별하며 현재 제목·플롯 위치 미달을 직접 Print와 보정 |
| #4085 | [#4096](https://github.com/edwardkim/rhwp/pull/4096), merge `3f7d875428` | 실제 테두리가 없는 글자겹침을 charSz로 축소하지 않음. 반전 사각형 -2는0.8 유지 | 두 계약을 유지. 양수50을0.5로 보는 별도 가정은 새 독립 Print의 signed10% 단계 근거와 대조. 테두리 게이트를 되돌리지 않음 |
| #4962 | [#5857](https://github.com/edwardkim/rhwp/pull/5857), merge `4bd9c5d60e` | 실제 layout 결정의 문자별 coverage를 비식별 aggregate로 계측. 당시 metric DB·fallback·paint 변경은 범위 밖 | 새 fallback 출처를 기존 face exact hit로 가장하지 않고 분류·원 face/실제 측정 provenance를 검토. 7분류 합계와 W2/W3 일치 유지 |
| #5874 | [#6775](https://github.com/edwardkim/rhwp/pull/6775), merge `1f861362ab` | SVG 이후 기본 PDF 변환에서 정규 fallback face의 기울임 누락을 합성 shear로 복원. 실제 italic·oblique는 이중 합성 금지 | 현재 독립 Print의 기울임과 대조. 실제 선택 face, baseline·advance 보존과 이중 합성 방지를 유지. 옛0.25 계수만 복원하지 않음 |
| #975·RealPic | [#1019](https://github.com/edwardkim/rhwp/pull/1019), merge `e26f159b60`(comment의 `f4d4fa34`는 별도의 반영 SHA) | SVG·Canvas의 fill mode와 watermark 톤을 공통 helper에서 맞춤. 원 값은 당시 한컴 화면 근사이며 공식 알고리즘을 확정하지 않음 | 새 Print로 검증한 RGB bake와 배치를 보존. opacity 문자열 존재를 시각 정확성의 대용으로 삼지 않음. 기존9개 신규 관계·물리 배치 검사와 대조 |
| 캡션 번호 | [#8049](https://github.com/edwardkim/rhwp/pull/8049), 열린 원 PR | 자동 번호 원문 자리표를 찾아 실제 표시 번호로 대체 | 실제 paint의 display 문자열과 원문 주소를 구분. 원본·공백 없는 반례는 양쪽99.98%대지만 최종 표시·위치 확인 후 검사 소비값을 판단 |
| 저장 원점 | [#7559](https://github.com/edwardkim/rhwp/pull/7559), 열린 Draft | 편집/HWP/HWPX 원점 차이의 미해결 재현. #7265 한컴 정본 여백 계약을 되돌려 통과시키면 안 됨 | 공개 API의 저장 형식별3.77px 차이 원인을 수정해야 함. 단순 허용 오차 확대나 정본 여백 제거 금지 |
| 자소·캐럿 | [#7819](https://github.com/edwardkim/rhwp/pull/7819), 열린 원 PR | 글자겹침 한 logical slot과 payload 표시 cursor를 구분. note 순서·문단 끝 hit·선택 경계 보존 | 실제 non-BMP·공유 앵커·문단 끝 반례와 논리/표시 축을 함께 대조. 현재 합성 출력 미달도 보정 대상 |
| 장식 clip | [#8038](https://github.com/edwardkim/rhwp/pull/8038), 열린 원 PR | 실제 장식 잉크를 본문 clip에 포함하되 원 글자 bbox·캐럿과 셀 clip은 보존 | 현재 공통 강조점 glyph의 잉크 범위와 실제 body clip을 확인. 옛 점 모양 selector만 기계적으로 복원하지 않음 |
| 이어받기 | [#8055](https://github.com/edwardkim/rhwp/pull/8055), 열린 원 PR | 분할한 위아래 배치 표의 뒤 간격과 쪽 원점을 한 번만 소비 | 원문은 실제 문서에 한컴 PDF가 없었다고 명시. 제출 합성은 pageBreak=NONE·빈 LineSeg 가정이 있어 한컴 출력과 다름. 저장 줄을 임의 생성하거나 기대값만 갱신하지 않고 독립 출력으로 전제부터 검토 |

## 새 Print에서 확인한 별도 표 배치 차이

#7084의 같은 원본 HWP/HWPX를 MCP engine2020·한컴11.0.0.9136·PrintMethod0·1-up으로
다시 출력했다. HWP 응답의 input_preprocess는 `scripts_disabled`, HWPX는 `none`이다.
이 차이를 숨기지 않고 영수증과 입력 SHA를 보존한다. 문서 스크립트를 실행하거나 승인하지 않았다.

- 두 형식 모두2쪽이며1쪽 Native는100%다.2쪽은14.18126%이고 표 사이의 세로 여백이 빠진다.
- 고정 devel `5e2b9bfbf0`의 같은 입력·새 Print 대조도2쪽14.08322%다.
  따라서 이 미달은 최신 보충 메트릭 보정에서 새로 발생한 것으로 단정할 수 없다.
- `supplemental-originals-base-comparison-422.json`에 base/head 전쪽 TSV·명령을 연결했다.
- 2쪽의 여러 표는 같은 문단에 있으며 각 바깥 위·아래 여백283HU가 있다. 앞 표 테두리
  끝과 다음 표 시작 사이의 물리 공간을 실제 측정·배치 양쪽에서 추적한다. 문서명 분기나
  최종 좌표 clamp로 차이를 감추지 않는다. 원본과 정상1쪽, 독립 Print를 모두 보존한다.
- 미달을 이유로 임의 이슈 이관·회귀 삭제·완료 처리하지 않는다. 현재 필수 보정으로 남는다.

## 현재 보정의 일괄 대조

Native048의88조건은 전부90% 이상이며 최저92.67157%다. 이전 통합 후보 Native047과
같은 조건으로 비교한86개 기존 조건은0.1%p 이상 하락0쪽, 새 독립 대조군2개는100%다.
강조점 split94.27793→100%, note-leading98.27445→100%, 원문 경계2쪽 입력의 최저는
99.18718→99.43103%다. 기록은 `native048-vs047-88-comparison-425.json`에 있다.
이는 직전 후보와의 대조이며 최신 devel의 전체 gate를 대신하지 않는다. fresh WASM022는
같은 source snapshot으로 빌드 중이다. 전체 Rust·시각 gate·직접 판독 완료 전에는 PR을 제출하지 않는다.

## 같은 host의 여백 프레임 원 처리 추적

현재 측정의 `empty_float::prepare` → `commit_empty_float_table` → 저장한
`paragraph_float_placements` → 실제 `layout_table_control_block`의 원점 소비를 확인했다.
렌더 fallback은 별도 lane을 계산하므로 실제 확정 결과 사용 여부도 대조해야 한다.

- #7270의 `5b481c8b2c`와 메인터너 보정 `0532933b9`는 저장 사다리가 증명하는
  흐름 높이를 한 번만 소비하도록 통합했다. 그 계약을 되돌리지 않는다.
- `5a36cf37267`은 #7505의 그림 표 형제 프레임 공유다. 일반 위아래 배치 표의
  resolved branch는 host_spacing과 effective_height를 소비하고 바깥 위·아래 여백과
  실제 paint inset의 소비 경계를 확인해야 한다. #7505는 원 #7382에 대한 메인터너
  보정 통합이므로 원 contributor 코드만으로 설명하지 않는다.
- 원본2쪽의 세 표는 같은 pi=12, ci=0/1/2이며 현재 테두리 y=41.6/128.6/980.1이다.
  원본을 바꾸지 않고 Print와 대조한다. 진단용 HWPX3개는 같은 host의 바깥 여백만
  0/0,283/0,0/283HU로 바꿨다. `float-outer-controls-428/manifest.json`에 차이를
  명시했고 별도 MCP Print를 요청했다. 이 진단 대조군을 원본 완료 증거로 쓰지 않는다.

## Native HWP의 형제 표 공유 경로 보정 착수

원본의 바깥 여백만 바꾼 독립 Print 대조에서 Native048의2쪽 raw는
0/0HU=100%,283/0HU=27.56303%,0/283HU=24.10151%다. 모든 입력은2쪽이다.
`float-outer-controls-432`와 MCP job 원문을 보존한다. 임의의 고정 gap을 더하지 않는다.

`host_spacing::resolve`는 위·아래 바깥 여백을 이미 `before`·`after`에 계산하고
`after_for_fit`에서 마지막 아래 여백의 fit 면제만 분리한다. 그런데 같은 host의
형제 표를 확정하는 `empty_float::prepare`의 공통 resolved 결과는 HWPX 저장 프로필만
선택했다. Native HWP 및 HWP 유래 HWPX는 이 결과를 소비하지 않고 렌더의 별도 lane으로
가서 앞 표의 아래 여백과 다음 표의 위 여백이 빠진다.

프로필 제한을 제거해 기존 `before → table_top → effective_height + after → occupied_bottom`
결과를 두 형식이 함께 소비하도록 한다. `commit_empty_float_table`의 같은 lane 예약과
`layout_table_control_block`의 확정 원점 우선 소비는 유지한다. Square 표·TAC·단일 표와
저장 사다리로 입증한 별도 outer-box 결과는 기존 분기를 유지한다. Native049에서 원본 두 형식,
여백 대조군과 기존 분할·형제 표·본문 간격 검사를 함께 실행한 뒤 판정한다.
이 문단은 보정 방향이며 아직 수정 후 실행으로 통과를 입증한 결과가 아니다.

#7559가 참조한 #7265는 셀 글자 서식 갱신 이슈이며 원 해결은 #7400, 실제 통합은
#7430이다. #7430은 HWP5 계보 HWPX 첫 표의 outer_top 소비를 메인터너가 보정했다.
#7538은 #4068의 단 맨 위 paint 원점 보정 PR이며, 뒤 문단 흐름에서는 inset을
되돌리고 같은 host 형제 표는 실물 아래에 쌓는 계약을 명시했다. 당시 labnote4건은
MCP 실패로 정본 없이13.2px 움직였다고 원문에 남았다. 이번에는 새 Print가 있으므로
그 기록을 통과 근거로 대신하지 않고 원본과 공통 프레임을 실제 재검증한다.

## Native049 원본 재검증

잡8개·공유 target의 빌드는263초에 exit0으로 끝났다. 변경한 제품 소스는
`empty_float.rs`의 형제 표 공유 경로 프로필 제한뿐이다.

- 같은 원본 HWP·HWPX의 전체2쪽 Native raw가 각각100%다.2쪽은14.18126→100%다.
- 여백0/0,283/0,0/283HU의 세 대조군도 각각 전쪽100%다.
- 새 강조점 관계 검사3건은 동결 Native048/049 양쪽에서3PASS로 유지됐다.
  `emphasis-metric-before-after-444.json`에 정식 테스트 파일 SHA와 strict build를 기록했다.
- `supplemental-originals-panels-443`의 원본2쪽 review와 standalone overlay를 직접 열었다.
  앞 표·34행 표·서명 표의 여백과 격자·내용 원점을 같은 독립 Print에 대조했다.
  컬러 이모지와 raster 경계 농도 차이는 RGB 완전 일치로 바꾸어 주장하지 않는다.
-101조건 Native 일괄 대조는 계속 실행 중이다. #7819 원본89.02659%, #8055 원본1.22674%의
  기존 미달도 그대로 기록하며 이 보정의 통과로 전체 검증 완료를 선언하지 않는다.
- fresh WASM023에서 같은 원본과 대조군을 다시 확인해야 한다. 새 형제 표 회귀는
  양쪽 전쪽90% 이상·직접 판독 뒤에만 정식 추가한다. 기존 기대값은 아직 바꾸지 않았다.

강조점 전 단계의 Native048/fresh WASM022는88조건 모두90% 이상으로 완료됐다.
정식 강조점 회귀3건과 입력·독립 PDF·원문 TSV·review/overlay는 `b0a85b6b88`에 있다.
현재의 새 형제 표 보정은 그 이후의 별도 제품 단계다.

## 원 PR 대조 후 형제 표와 보충 측정 검증 완료

GitHub API로 #7119의 merge `922946438f`, #7430의 실제 merge `eb321165ec`를 다시 확인했다.
#7538은 closed·unmerged이며, 그 설명과 실제 반영된 통합·보정 커밋을 구분한다.
`original-pr-recheck-474.json`에 본문과 조회 결과를 보존했다.

제품 `345d0b5202`의 Native049/fresh WASM023 실제 소스145개를 커밋 바이트와 대조했다.
Native 빌드 전 HEAD는 `b0a85b6b88`·dirty이며 이를 clean HEAD 빌드로 오기하지 않는다.
WASM 최적화·Studio public 동기화가 완료됐고 실행 artifact SHA도 검증했다.

- 원본 HWP/HWPX와 여백3대조군은 각 전체2쪽 Native/fresh 모두100%다.
- 같은 입력·Print·글꼴을 고정 devel `5e2b9bfbf0`과 대조한20쪽에서0.1%p 이상 하락0쪽이다.
  기준 원본2쪽은 Native14.08322%, fresh14.13750%다. 기준 미달의 exit1과 gate를 그대로 보존했다.
- 원본2쪽 전체와 표 사이 위·아래 여백을 양쪽 review/standalone overlay로 직접 판독했다.
  컬러 이모지와 raster 농도 차이는 남으며 RGB 완전 일치를 주장하지 않는다.
- 새 `coanchored_float_outer_frame.rs`7건은 원문 소유·행열·내용 순서/보존·2쪽 소속·여백 관계를 검사한다.
  원본 완료2쪽은 양쪽100%이므로 독립 PDF 외곽 좌표±2px를 관계 검사와 함께 추가했다.
  정식 소스 strict rustc로 수정 전 Native048은6FAIL·정상1PASS, Native049는7PASS다.
  최초 진단의 page 인덱스 타입 오류와 부동소수점 동등 경계 검사 오류도 로그467/468에 남겼다.
- #7084의32검사는 같은 소스에서 수정 전 Native047은16FAIL·16PASS, Native049는32PASS다.
  마지막2건은 보편0.5em 폭 핀·특정 fallback 문자열 대신 독립 provider 폭의 적용,
  뒤 문자 상대 간격·원문 경계 보존과 다중 scalar 자소의 측정 출처 격리를 검사한다.
  IR 불변·실제 backend 적용·서식 격리 검사를 유지한다. 제어된 provider 값27.5를
  한컴 이모지의 실제 폭으로 주장하지 않는다.

최종 입력·독립 Print PDF·전쪽 원문 TSV·대표 PNG·출처는
`mydocs/pr/assets/coanchored_float_outer_frame_20261020/README.md`에 연결했다.
실행 원문은 ignored `coanchored-base-comparison-471.json`,
`emphasis-metric-coanchored-before-after-476.json`,
`emphasis-metric-current-failure-focus-470.json`에 있다.
이는 전체 Rust 재실행 완료가 아니다. #7559 live API 저장 원점과 #8055 등의 필수 보정은 계속한다.

#7559의 현재 공개 API 입력을 Native049에서 그대로 생성해 live/native HWP/adapter HWP/HWPX를
대조했다. 두 HWP export 바이트는 동일하다. offset의 live/HWP는176.2666667,
HWPX 재열기는180.04다. 이전 저장 fixture와의 차이는 생성 LineSeg의 synthetic tag다.
따라서 옛 fixture의100%만으로 현재 API의 실패를 해소했다고 판정하지 않는다.
현재 입력4개는 새 독립 MCP Print를 요청했고 `save-origin-current-api-print-475.log`에 기록한다.
