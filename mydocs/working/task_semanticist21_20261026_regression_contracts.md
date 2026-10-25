# 실제 글꼴 공급자와 회귀 검사의 독립 계약

## 검증 대상과 결과

런타임 소스는 `11dc162752ebcd30084757a4f9e20d18d0d8189a`다. Native253과
fresh WASM072로 기존 17조건·각 213쪽을 전체 비교했다. 최저 raw 일치율은 각각
92.04708%, 91.53306%이며, 모든 쪽이 90% 이상이고 독립 Print PDF와 쪽수가 일치했다.
고정 base의 같은 입력·PDF와 대조한 페이지별 0.1%p 초과 하락은 양쪽 모두 0쪽이다.
#6133의 base는 쪽수가 달라 raw 비교값을 만들지 않았고, 현재의 2쪽 내용 보존을 별도로
확인했다. 이 결과를 전체 Rust 검증 완료로 기록하지 않는다.

## #5846 기호 선택 조건의 교정

기존 검사는 원본이 요청한 `휴먼명조` 문자열로 실제 U+2024 기호를 찾았다. 독립
Print PDF와 현재 SVG의 실제 공급자는 한컴바탕이며 첫 paint family는
`Haansoft Batang`이다. 요청 face와 실제 공급자를 구분하도록 선택 조건만 교정했다.
기존 좌표·±2px 허용 오차·소유·기호 폭과 다음 글자의 관계 검사는 유지했다.

같은 원본 66쪽의 Native/fresh 최저는 97.15069/97.06823%다. 중요한 65쪽은 양쪽
raw 100%이며 canonical review와 standalone overlay에서 기호·다음 글자·간격을
직접 확대 판독했다. 집중 회귀는 교정 전 98 PASS / 1 FAIL에서 교정 후
99 PASS / 0 FAIL로 종료했다. 이 선택 조건 교정은 렌더링 기대 위치를 바꾸지 않는다.

## 증적

아래 경로의 공통 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

| 항목 | 증적 |
| --- | --- |
| Native 전체 비교 | `affected-current-full-4019-result.json`, `remaining-active-native253-4022-result.json` |
| fresh WASM 전체 비교 | `affected-current-wasm072-4026-result.json`, `remaining-active-wasm072-4027-result.json` |
| base 대비 페이지별 비교 | `native-page-preservation-4025.json`, `wasm-page-preservation-4033.json` |
| 집중 99건 종료 | `verified-boundaries-4032-result.json`, `logs/verified-boundaries-4032.log` |
| Native 65쪽 직접 판독 | `selected-native253-4024/5846-native253/review/review_065.png`, 같은 조건의 `overlay/overlay_065.png` |
| fresh WASM 65쪽 직접 판독 | `selected-wasm072-4030/5846-wasm072/review/review_065.png`, 같은 조건의 `overlay/overlay_065.png` |

## 남은 검토와 삭제 유지

#8045의 수동 LineSeg 재현본과 한컴 저장본은 같은 입력으로 취급하지 않는다. 같은
재현본의 한컴 저장본에서 드러난 표 높이 차이와 #8093의 혼합 텍스트·양식 개체 기준선
차이는 공통 측정·배치 경로에서 분석 중이다. 기존 재현본의 raw가 90%를 넘더라도
직접 판독에서 확인한 차이가 남아 있으므로 완료로 판정하지 않는다.

sample16·편람의 기존 삭제를 유지한다. 이전 파생 참조를 되살리거나 제외 입력을
재생성하지 않는다. `retired-reference-readonly-4038.json`은 manifest를 저장하지 않고
현재 1,767개 수집 소스와 전용 검증 worktree의 실제 1,767개 참조를 대조했다.
누락·제외 대상 참조는 0개이며 주 작업공간의 generated 파일은 변경하지 않았다.
