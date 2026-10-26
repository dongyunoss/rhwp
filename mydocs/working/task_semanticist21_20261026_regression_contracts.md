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

## 당시 남은 검토와 삭제 유지

#8045의 수동 LineSeg 재현본과 한컴 저장본은 같은 입력으로 취급하지 않는다. 같은
재현본의 한컴 저장본에서 드러난 표 높이 차이와 #8093의 혼합 텍스트·양식 개체 기준선
차이는 공통 측정·배치 경로에서 분석 중이다. 기존 재현본의 raw가 90%를 넘더라도
직접 판독에서 확인한 차이가 남아 있으므로 완료로 판정하지 않는다.

sample16·편람의 기존 삭제를 유지한다. 이전 파생 참조를 되살리거나 제외 입력을
재생성하지 않는다. `retired-reference-readonly-4038.json`은 manifest를 저장하지 않고
현재 1,767개 수집 소스와 전용 검증 worktree의 실제 1,767개 참조를 대조했다.
누락·제외 대상 참조는 0개이며 주 작업공간의 generated 파일은 변경하지 않았다.

## 후속 보정과 최종 Rust 검증 — 2026-10-26

위 #8045·#8093의 당시 미완료 상태는 공통 보정과 독립 Print 대조로 해소했다.
[Form·정상 한컴 중첩 표 증거](../pr/assets/inline_wrap_qualified_20261026/README.md)의
세 조건은 전쪽 Native255·최적화 fresh WASM073 모두 raw 100%다. 수동 LineSeg 입력과
정상 한컴 저장본을 구별하며, 정상 저장본의 통과를 수동 입력의 완전 일치로 바꾸어 쓰지 않는다.
코드·회귀 후보는 `61528013511c87c732524f91c6f224286a055337`, 실제 runtime은
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`다. 이후 문서·증거 커밋과 해당 코드·검사
파일이 동일함을 대조했다. 새 회귀의 수정 전 실패·수정 후 통과는 4057·4059 기록에 있다.

| 필수 Rust 검증 | 실제 최종 결과 | ignored output의 로그·영수증 |
| --- | --- | --- |
| 준비·fmt·suite 정책·unit tier 정책·Release build | 각각 exit 0 | `final-full-rust-4052-result.json`의 완료 단계 |
| Release 라이브러리 | 3,983 PASS·0 FAIL·12 ignored | `logs/final-release-lib-4052.log` |
| 전체 nextest | 11,708 PASS·0 FAIL·45 skipped | `logs/final-full-nextest-4052.log`, `full-nextest-terminal-audit-4092.json` |
| Native Skia 라이브러리 | 4,040 PASS·0 FAIL·12 ignored | `logs/final-native-skia-lib-4093.log` |
| Native Skia placeholder·직접 PDF 검사 | 각각 exit 0 | `remaining-rust-4093-result.json` |
| Native·WASM32 Clippy | 각각 exit 0 | `logs/final-native-clippy-4093.log`, `logs/final-wasm-clippy-4093.log` |
| workspace build·all-target Clippy | 각각 exit 0 | `logs/final-workspace-build-4100.log`, `logs/final-workspace-clippy-4100.log` |
| 문서 테스트 | 8 PASS·0 FAIL·3 ignored, exit 0 | `logs/final-doctests-4108.log` |

표의 공통 루트는 `output/pr-review/semanticist21-20261019/`다.
최종 집계 `final-rust-validation-4111.json`은 각 로그의 SHA-256과 실제 종료 결과를 연결한다.
전체 nextest의 바깥 driver는 SIGTERM으로 끝났고 child 종료 코드는 수집되지 않았다.
따라서 driver를 성공으로 표시하지 않고, 별도 감사로 확인한 명시적 terminal summary와
연속된 11,708개 PASS를 전체 nextest의 완료 근거로 쓴다. 이후 미실행 단계는 따로 완료했다.

workspace query cache 쓰기는 ENOSPC로, 첫 문서 테스트의 다섯 항목은 같은 시점의
링커 SIGBUS로 실패했다. 종료된 정확한 증분 캐시와 사용하지 않는 output 실행 파일 사본을
정리하고 같은 소스로 재실행했다. 문서 테스트는 임시 실행 파일 링크가 겹치지 않도록
한 개씩 실행했으며 빌드 잡은 8개, 공유 target은 `target/pr-review`로 유지했다.
기대값·golden·검사 항목은 바꾸지 않았다. 실패 기록을 삭제하거나 이전 driver의 상태를
성공으로 덮어쓰지 않는다.

현재 원 PR 재현 입력 62조건의 후보 Native·fresh WASM 캡처 124개는 모두 완료됐다.
base 대조는 디스크 부족으로 중단된 뒤, 출처가 일치하는 완료 캡처는 canonical
`--png-pair`로 다시 측정하고 누락된 캡처만 실제 export해 계속한다. 일반 PNG 모드의
`--resume`을 실루엣 전용 모드에 적용했던 4109는 인자 오류이며 통과 증거에서 제외한다.
이 대조와 대표 경계의 직접 판독이 완료되기 전에는 전체 PR 준비 완료로 판정하지 않는다.

sample16·편람의 삭제된 회귀·fixture·보관본·파생 참조는 생성하지 않았다.
`retired-reference-preservation-4103.json`은 manifest를 쓰지 않고 현재 수집 소스와
주 작업공간·검증 worktree의 generated 참조를 대조했다. 삭제 파일을 가리키는 참조와
수집 소스의 누락은 0개다. OLE 원본의
[현재 Native·WASM 증거](../pr/assets/legacy_chart_physical_layout_20261020/current-20261026/README.md)는
기존 HWP·추적 중인 동일 Print PDF를 재사용했으며 제외 문서의 복원과 무관하다.


## 원 PR별 실제 실행 대조와 삭제 유지 재확인 — 2026-10-26

[기존 재현본 62조건의 최종 증거](../pr/assets/source_pr_qualified_20261026/README.md)를
보존했다. 전쪽 Native·fresh WASM 후보 각각 62쪽의 최저 raw는 99.98107%이고,
독립 Print 쪽수가 같으며 base 대비 0.1%p 초과 하락은 0쪽이다. 대표 23조건의
review·standalone overlay 46쌍은 기존 캡처의 입력/PDF·producer·실제 PNG 해시를
확인한 뒤 합성했다. 원본 RGB가 같은 경우의 판독 재사용과 새 캡처를 구분했다.
원문 TSV 248개·비교표 2개·PNG 92개의 해시를 재확인했다.

원 PR 45개의 현재 검사 함수 196개를 실제 PASS 행과 연결했다. 이는 PR 간 공용
함수의 중복을 포함한 연결 수이며 서로 다른 검사 196개를 새로 만들었다는 뜻이 아니다.
기본 nextest에서 조건부로 빠진 Native font trace 두 함수는 별도 native-skia 모듈
실행의 12 PASS·0 FAIL·exit 0으로 확인했다. Studio 네 소스는 전체 Node 명령에
명시된 실제 검사 파일과 그 파일이 import한 runner를 구분했다.

| 원 PR | 실제 PASS와 연결한 Rust 함수 수 | Studio 검사/runner 소스 수 |
| --- | ---: | ---: |
| [#7559](../pr/archives/pr_7559_review.md) | 2 | 0 |
| [#7572](../pr/archives/pr_7572_review.md) | 3 | 0 |
| [#7573](../pr/archives/pr_7573_review.md) | 3 | 0 |
| [#7588](../pr/archives/pr_7588_review.md) | 11 | 0 |
| [#7589](../pr/archives/pr_7589_review.md) | 2 | 0 |
| [#7662](../pr/archives/pr_7662_review.md) | 6 | 0 |
| [#7694](../pr/archives/pr_7694_review.md) | 19 | 0 |
| [#7819](../pr/archives/pr_7819_review.md) | 14 | 0 |
| [#7833](../pr/archives/pr_7833_review.md) | 38 | 0 |
| [#7904](../pr/archives/pr_7904_review.md) | 0 | 4 |
| [#8014](../pr/archives/pr_8014_review.md) | 1 | 0 |
| [#8025](../pr/archives/pr_8025_review.md) | 1 | 0 |
| [#8027](../pr/archives/pr_8027_review.md) | 4 | 0 |
| [#8029](../pr/archives/pr_8029_review.md) | 1 | 0 |
| [#8030](../pr/archives/pr_8030_review.md) | 1 | 0 |
| [#8031](../pr/archives/pr_8031_review.md) | 2 | 0 |
| [#8032](../pr/archives/pr_8032_review.md) | 2 | 0 |
| [#8033](../pr/archives/pr_8033_review.md) | 2 | 0 |
| [#8035](../pr/archives/pr_8035_review.md) | 3 | 0 |
| [#8036](../pr/archives/pr_8036_review.md) | 2 | 0 |
| [#8037](../pr/archives/pr_8037_review.md) | 2 | 0 |
| [#8038](../pr/archives/pr_8038_review.md) | 2 | 0 |
| [#8039](../pr/archives/pr_8039_review.md) | 1 | 0 |
| [#8041](../pr/archives/pr_8041_review.md) | 7 | 0 |
| [#8042](../pr/archives/pr_8042_review.md) | 11 | 0 |
| [#8043](../pr/archives/pr_8043_review.md) | 9 | 0 |
| [#8045](../pr/archives/pr_8045_review.md) | 1 | 0 |
| [#8049](../pr/archives/pr_8049_review.md) | 2 | 0 |
| [#8050](../pr/archives/pr_8050_review.md) | 1 | 0 |
| [#8051](../pr/archives/pr_8051_review.md) | 1 | 0 |
| [#8052](../pr/archives/pr_8052_review.md) | 2 | 0 |
| [#8053](../pr/archives/pr_8053_review.md) | 1 | 0 |
| [#8054](../pr/archives/pr_8054_review.md) | 1 | 0 |
| [#8055](../pr/archives/pr_8055_review.md) | 1 | 0 |
| [#8056](../pr/archives/pr_8056_review.md) | 1 | 0 |
| [#8057](../pr/archives/pr_8057_review.md) | 3 | 0 |
| [#8058](../pr/archives/pr_8058_review.md) | 4 | 0 |
| [#8061](../pr/archives/pr_8061_review.md) | 1 | 0 |
| [#8062](../pr/archives/pr_8062_review.md) | 1 | 0 |
| [#8065](../pr/archives/pr_8065_review.md) | 9 | 0 |
| [#8066](../pr/archives/pr_8066_review.md) | 1 | 0 |
| [#8090](../pr/archives/pr_8090_review.md) | 1 | 0 |
| [#8093](../pr/archives/pr_8093_review.md) | 4 | 0 |
| [#8094](../pr/archives/pr_8094_review.md) | 11 | 0 |
| [#8096](../pr/archives/pr_8096_review.md) | 1 | 0 |

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
`individual-pr-execution-bindings-4125.json`에는 현재 검사 파일 SHA-256, 모듈·함수의
실제 PASS 행과 로그 SHA-256을 기록했다. `native-trace-contracts-4119-result.json`과
`logs/native-trace-contracts-4119.log`는 Native 조건부 검사 12건의 실제 종료 근거다.
개별 리뷰의 검사 통과는 독립 Print 일치나 PR의 전체 수용 판정을 대신하지 않는다.
아직 게시하지 않은 통합 PR의 CI·merge·후속 처리를 완료했다고 기록하지 않았다.

`retired-reference-preservation-4123.json`은 `deriveManifest`의 persist=false 경로로
현재 수집 소스 1,787개를 읽었다. 누락 0개·삭제 대상 참조 0개이며 현재 generated
harness의 삭제 대상 참조도 0개다. manifest·fixture·보관본·삭제된 파생 파일을
재생성하지 않았다. 텍스트 검색의 과거 사유 주석과 제외 필터는 실제 원본을 여는
활성 참조와 구분해 유지했다. 검사 대상과 실제 실행 코드는 변경하지 않았다.

## 삭제된 파생 참조의 재생성 금지 재확인 — 2026-10-26

사용자 지시에 따라 sample16·편람의 삭제를 유지한다. HEAD
`588130c50b2e5a5a6f14cded810f68198880c988`에서 `deriveManifest`의
`persist=false` 경로로 수집한 소스 1,787개와 현재 generated harness를 읽고,
새 harness 문자열도 메모리 안에서만 대조했다. 누락·삭제 대상 소스·현재 파생 참조·
재생성 예정 참조가 모두 0건이다. `--prepare`나 fixture·보관본 생성은 실행하지 않았다.
결과는 ignored `output/pr-review/semanticist21-20261019/retired-reference-preservation-4148.json`이다.
텍스트 검색의 과거 근거 주석·정상 합성 계약과 실제 파일을 여는 활성 참조는 구분했다.

같은 범위의 [#8061 개별 검토](../pr/archives/pr_8061_review_impl.md#독립-print의-레이어-반례--2026-10-26)에서
불완전한 초기 기하 정보와 레이어 의미의 반례를 분리했다. 같은 재현의 기하 보완본·
단일 속성 대조 4조건은 후보/base × Native/fresh WASM 16회 모두 raw 100%·1쪽이지만,
`flowWithText=0, zOrder=0` 그림은 Print와 달리 쪽 도형 위에 보였다.
같은 불투명 영역의 색상 차이는 실루엣만으로 판정하지 않는다. 해당 PR의 수용 판정은
보류를 유지하고 공통 레이어 소비 조건의 보정 대상으로 기록했다.

`pr8061-controls-panels-4150-result.json`의 compare·standalone overlay·review 8쌍은
현재 source와 sweep·입력/PDF·원 PNG 해시를 확인해 기존 캡처에서 정본 함수로 합성했다.
그중 Native의 첫 조건과 `flowWithText=1` 조건의 review·overlay 4개를 실제로 열어
서로 다른 그림 표시와 같은 셀 본문 순서를 직접 대조했다. 나머지 생성 이미지를 모두
판독했다고 기록하지 않는다. source·검사·지침·Visual Sweep 도구는 바꾸지 않았다.

## 셀 그림의 쪽 순서와 편집 소유 분리 보정 — 2026-10-26

앞 절 이후 #8061의 필수 반례에 따라 공통 renderer를 보정 중이다. 기존 검증 코드
후보 `61528013511c87c732524f91c6f224286a055337`와 렌더 소스
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`의 결과를 이 새 소스의 완료 결과로
재사용하지 않는다. source·검사·지침을 바꾸지 않았다는 앞 절은 그 단계 당시의 기록이다.

`layout/anchored_cell_paint.rs`로 기존 그림 wrapper·셀 순서 조립을 옮겼다.
`flow_with_text=0`의 일반 셀 그림은 셀의 편집 주소와 절대 좌표를 보존하면서 쪽
개체들과 정렬한다. `flow_with_text=1`과 이미 증명된 셀 프레임 paint는 셀의 그리기
소유를 유지한다. 쪽 순서를 따르는 한 그림의 bitmap·테두리·캡션은 같은 묶음으로
이동하며 임시 NodeId를 순서 비교에 쓰지 않는다. 원본 주소가 없는 테두리가 bitmap
앞으로 재정렬되지 않도록 원본 그림의 정렬키와 내부 부분 순서를 함께 보존한다.
공개 JSON 계약에 내부 묶음 필드를 추가하지 않는다.

같은 원 재현을 떠 있는 표로 바꾸고 겹침 영역을 명시한 Print 3조건에서도
`flowWithText`와 z-order의 차이가 확인됐다. 별도의 2px 파란 테두리 대조군은 같은
원 재현에서 테두리만 추가했다. 다른 실물 HWP·이관 문서·sample16·편람을 추가하거나
이전 파생 참조를 재생성하지 않았다.

| 단계 | 실제 결과와 남은 검증 |
| --- | --- |
| Native 256 | 초기 공통 분리 보정, release-test 빌드 exit 0·4분 39초·source 해시 고정. 8조건·9쪽의 수정 전후를 확인했다. |
| 집계 오류 복구 | 4158 집계기는 시스템 Pillow API에서 실패했다. 완료된 PNG·producer·입력/PDF 해시를 검증하고 4159의 정본 PNG-pair TSV로 재집계했다. 재렌더하지 않았다. 후보 8조건 raw 100%, 두 레이어 반례는 수정 전 FAIL·수정 후 PASS다. |
| Native 257 | 한 개체 내부의 bitmap·테두리 순서까지 보존한 보완본, release-test 빌드 exit 0·4분 17초. 4163의 9조건·10쪽 Native 대조는 모두 raw 100%·쪽수 일치·색상 존재 진단 통과다. |
| fresh WASM 074 | 같은 공통 소스의 표준 release·실제 wasm-opt 빌드 exit 0·17분 33초. 아래 신규 양쪽 시각 검증과 focused 회귀를 완료했다. |

기존 #8061 합성 검사는 `flowWithText=0` 그림도 무조건 쪽 도형 위에 두어야 한다고
고정하므로 독립 Print와 맞지 않는다. 실제 원본·Print·양쪽 시각 확인을 마친 뒤
새 이름의 관계 검사와 자격을 충족한 페이지의 중요 좌표 검사로 교체한다. 기존의
전체 Rust·Studio PASS는 이전 코드 후보의 결과이며 새 공통 소스에서 필요한 필수
게이트를 다시 완료해야 한다. PR은 아직 제출하지 않았다.

로그·영수증은 ignored `output/pr-review/semanticist21-20261019/` 아래
`pr8061-runtime-overlay-4156.json`, `pr8061-runtime-overlay-4161.json`,
`rebase-native256-build-4157-result.json`, `cell-layer-boundaries-audit-4159-result.json`,
`pr8061-floating-controls-print-4154-result.json`, `pr8061-border-control-print-4160-result.json`,
`rebase-native257-build-4162-result.json`, `cell-layer-boundaries-validation-4163-result.json`,
`rebase-candidate-wasm074-build-4164-result.json`에 구분했다. 임시 source overlay와
Git head를 같은 것으로 보고하지 않으며 실제 파일별 SHA-256을 보존했다.


## 셀 그림 보정의 양쪽 검증과 파생 참조 보존 — 2026-10-26

[정식 증거](../pr/assets/cell_picture_layer_qualified_20261026/README.md)에 같은 #8061
재현의 HWPX 8개·독립 MCP Print PDF 8개·Native/fresh WASM TSV·대표 review/overlay를
등록했다. 8조건은 양쪽 전쪽 raw 100%이며, 정상 RowBreak 2쪽도 양쪽 100%다.
기존 영향 문서 6조건·94쪽을 같은 새 런타임으로 재출력했다. 선행 검증 합계는
15조건·104쪽/backend, 전쪽 90% 이상·독립 PDF 쪽수 일치다.
base 대비 페이지별 비교 208개에서 0.1%p 초과 하락 0쪽을 확인했다.

새 이름의 셀 그림 검사 9개와 RowBreak 검사 1개는 focused nextest 10 PASS·0 FAIL이다.
첫 실행의 8개 실패는 같은 원본 좌표의 약 1e-14px f64 차이였고, 정수 HWPUNIT의
동등·포함 관계로 수정했다. 독립 PDF ±2px 조건은 유지했다. 별도 순서 증거 3조건은
수정 전 FAIL / 수정 후 PASS다. raw만으로 드러나지 않는 잘못된 색상 가림도 실제 PNG로
검사했다. PHOTO의 래스터 경계와 일부 약 1.12px 원점 차이는 숨기지 않고 기록했다.

삭제 이력에서 관련 검사·fixture를 추적하고 `deriveManifest(persist=false)` 및 메모리의
harness로 두 작업공간의 소스 1,788개를 대조했다. 누락·삭제 대상 source·새로 생성될
삭제 대상 참조는 0건이다. 준비한 검증 worktree의 실행 harness도 대상 참조 0건이다.
주 작업 트리의 과거 ignored generated 파일에는 삭제된 `diag_1042_version_check.rs`
참조 1건이 남아 있으나 복원·재생성·실행·stage하지 않는다. 과거 사유 주석·공용 제외
필터·제품 catalog enum은 실제 파일을 여는 활성 검사와 구분한다. sample16·편람의
삭제된 파생 참조를 다시 생성하는 변경은 추가하지 않았다.

ignored 영수증은 `cell-scope-focused-4175-result.json`,
`cell-scope-negative-proof-4176.json`, `retired-reference-preservation-4177.json`,
`cell-scope-finalization-4178.json`, `cell-scope-base-comparison-4179.json`이다.
새 코드의 전체 Rust·lint·Studio 및 최종 제출 범위 검증은 별도로 완료해야 하며,
이전 런타임의 전체 PASS를 재사용하지 않는다. 아직 PR을 제출하지 않았다.


## 사용자 지시에 따른 손상된 이동 HWP 삭제 — 2026-10-26

기존 일반 이동 `before.hwp`·`after.hwp`의 한컴 2020 손상 경고를 확인한 뒤 삭제했다.
두 파일의 SHA 목록과 재열기 경로도 정리했다. HWPX 두 파일 및 차트 HWP/HWPX 네 파일의
모델·저장 계약은 별도 `valid_inline_move_fixture_contract`에 유지하고, 공용 helper를
분리해 기존 원본은 983줄로 줄였다. 새 HWP로 대체하거나 삭제 파일을 재생성하지 않는다.

현재 실행 중인 전체 nextest·Native/Studio 후속 작업은 삭제 전 고정 source의 결과로
구분한다. 종료 뒤 검증 worktree에도 삭제를 반영하고 변경한 검사의 집중 회귀·lint를
완료한다. 새 source에 과거 PASS를 적용하지 않는다. 실제 삭제와 근거는 #7694 개별
구현 기록 및 ignored `damaged-move-removal-4195.json`에 연결한다.

## 손상 표본 삭제 뒤 자동 수집 유지 — 2026-10-26

손상 HWP 두 개를 삭제한 `5e71af95aa` 뒤 일반 HWPX의 기존 배치 결함은
`a1fe7fb50f`와 #8266으로 구분했다. 모델·저장 계약과 정상 차트 계약은 유지한다.
검사 분리 과정에서 추가한 `#[path] mod` 두 개가 자동 수집기의 singleton 예외로
분류되어 integration target 예산이 48에서 50으로 늘어나는 문제를 확인했다.
기존 `tests/cases/`의 공유 helper 포함 방식처럼 명시적인 내부 모듈에서 `include!`하도록
수정했다. target 예산·정책·CI 호출을 완화하지 않으며, 같은 helper를 실제 suite 안에서
컴파일·실행해 보존 여부를 확인한다. `.rs.txt` 보존본은 제거 전 source 바이트 그대로 둔다.

`current-full-nextest-4184.log`는 생산 코드 `a90a2057a7`에서 11,716 PASS·0 FAIL·45 skipped,
전체 Native Skia 3종과 문서 테스트도 exit 0이다. Studio의 TypeScript·전체 Node·fresh WASM
실브라우저·production build·오류 표시 검증은 4186에서 모두 exit 0이다. 이 결과를 삭제 후
변경한 두 검사 source의 실행으로 바꾸어 기록하지 않는다. 변경된 검사와 필수 lint는
`logs/current-*-4205.log`에서 별도로 검증하고 terminal 결과를 추가 기록한다.

## 생산 코드의 최종 실행과 정리 후 검사 — 2026-10-26

| 범위 | 실제 terminal 결과 | ignored 로그 (`output/pr-review/semanticist21-20261019/`) |
| --- | --- | --- |
| `a90a2057a7` 전체 Rust nextest | 11,716 PASS·0 FAIL·45 skipped, exit 0 | `logs/current-full-nextest-4184.log` |
| 같은 생산 코드 Native Skia lib | 3,858 + 15 + 165 + 2 = 4,040 PASS·0 FAIL, exit 0 | `logs/current-native-skia-lib-4184.log` |
| Native placeholder / direct PDF | 2 PASS / 14 PASS, 각각 exit 0 | `logs/current-native-skia-{placeholder,direct-pdf}-4184.log` |
| 문서 테스트 | 8 PASS·0 FAIL·3 ignored, exit 0 | `logs/current-doctests-4184.log` |
| Native font trace | 12 PASS·0 FAIL, exit 0 | `logs/current-native-trace-4191.log` |
| Studio TypeScript·Node·실브라우저·production build | Node 2,597 PASS·0 FAIL, 5단계 모두 exit 0 | `logs/current-studio-*-4186.log` |
| `116735e8df` 삭제·분리 뒤 이동 계약 | 19 PASS·0 FAIL, exit 0·source 불변 | `logs/move-cleanup-focused-4207.log` |
| 같은 정리 뒤 전체 필수 lint | fmt·Native/WASM/workspace Clippy·workspace build·정책 모두 exit 0 | `logs/current-*-4205.log` |

전체 실행 뒤 바뀐 것은 손상 표본 삭제와 이동 계약 검사 정리다. 생산 Rust 코드는
`a90a2057a7`과 동일함을 확인했고, 변경된 검사 19개는 새 source로 다시 실행했다.
전체 회귀 결과와 새 focused 결과를 서로 다른 source 경계로 기록한다.

[최종 전쪽 TSV와 대표 review·overlay](../pr/assets/semanticist21_final_20261026/README.md)에
129조건·각 344쪽을 등록했다. 전쪽 최저는 Native 92.04708%, fresh WASM 91.53306%이며
90% 미만·누락·독립 PDF 쪽수 불일치는 0쪽이다. 수치 대응 684행에서 base 대비 0.1%p
초과 하락 0행이며 #6133의 1→2쪽 복원은 내용·쪽 소속의 구조 개선으로 따로 기록했다.
나머지 108조건의 새 Native 캡처는 이전 직접 판독 대상과 전쪽 픽셀이 같음을 확인했다.
새로 출력한 것을 이전 캡처의 재사용으로 대신하지 않았다. HWPX 차트 네 경로도 별도로
출력했으며 판독한 HWP 차트와 RHWP/PDF 픽셀이 각각 동일함을 확인했다.
Print 증거·입력·원문 TSV를 보존하고 font 목록만 동일 해시별 한 번 저장했다.
