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

## #8240 문서 전용 리베이스와 원 PR별 실행 대조 — 2026-10-26

최신 base `4e690c930105fa09fef484787ed0a2e5e182d444` 위로 리베이스했다.
새 base의 변경은 `THIRD_PARTY.md` 41줄뿐이다. 이전 후보 `53ef4bcfc6`과 새 후보
`d6ea108372`의 src·crates·tests·scripts·Studio·npm·Cargo tree ID가 모두 같고,
replay한 356개 commit은 range-diff에서 전부 같은 patch다. #8228의 조상 관계와
삭제한 표본을 유지했다. 새 base의 suite·unit-tier 정책 검사도 통과했다.
[리베이스 증거](../pr/assets/semanticist21_final_20261026/rebase_equivalence.json)와
[원 PR별 현재 commit 대응](../pr/assets/semanticist21_final_20261026/source_commit_map.json)을
등록했다. 실제 생산 코드는 리베이스 뒤 `e436d8aeefb5e46265f5f15f71ccfb0925742197`이며,
실행 당시 `a90a2057a7`의 코드와 같다. 이를 새로 빌드·실행한 결과로 재표기하지 않았다.

45개 개별 review·impl의 실제 실행 절을 현재 소스에 맞춰 갱신했다.
[실제 실행 대응](../pr/assets/semanticist21_final_20261026/source_execution_map.json)의
206개 함수 대응과 Studio 소스 4개에 누락이 없다. 94개 문서의 내부 Markdown 링크도
검사했다. 이 작업으로 최종 승인 판정을 일괄 변경하지 않았다.

## 원 PR 재현의 잔여 게이트 — 진행 중

129조건·344쪽/backend는 **완료한 통과 자료의 부분집합**이다. 별도의 원 PR 재현
59조건은 과거 Native 미달로 실행이 중단됐던 기록이 있었다. 그중 정상 RowBreak는
현재 같은 입력·Print의 완료 증거를 재사용하고, 초기 #8061의 불완전한 기하 입력은
독립 Print로 검증한 같은 재현 8조건으로 교체한 경계를 유지한다. 나머지 57조건은 기존
원본과 PDF의 바이트를 그대로 사용해 현재 후보/base × Native/fresh WASM을 다시 관측한다.
새 문서 수집·입력 생성·Cargo 빌드는 하지 않는다. `remaining-contracts-current-4213-result.json`과
`logs/remaining-contracts-current-supervisor-4213.log`에 실제 상태·명령·종료를 기록한다.
Native 후보 57회 중 36조건이 통과했고, 20조건은 raw 미달, 1조건은 쪽수 불일치다.
전 경로의 측정과 원인 판정 전에는 통합 PR을 제출하지 않는다.

### 확인한 실제 회귀: #8036

`pr8036-saved-vpos-0.hwpx`는 같은 원본의 독립 Hancom Print와 새 base가 모두 1쪽이며
base Native raw는 100%다. 현재 후보만 2쪽이라 TSV 산출이 쪽수 불일치로 중단된다.
`stored_reset_after_tall_line`의 2쪽 기대값과
`typeset/section/stored_reset.rs`에서 `pv > 5000`을
`pv > 0 && prev_vpos_end > 5000`으로 바꾼 강제 분기부터 재검토한다.
수동으로 만든 저장 줄의 큰 높이만으로 쪽 경계 수용을 넓힌 가정이 독립 Print와 다르다.
그대로 승인하거나 base 미달 문서로 이관할 수 없다. 원본 입력·Print는 보존하고,
공통 경계 판단을 보정한 뒤 두 vpos 대조군의 전쪽 Native/fresh WASM을 다시 확인한다.
잘못된 2쪽 기대를 새 정상 baseline으로 고정하지 않으며 새 이름의 관계 검사로 보호한다.

### 별도로 구분한 기존 중첩 표·문자 전진

#7833의 첫 nested 표 재현은 base raw 18.58277%, 후보 18.37031%다. 직접 Native 재출력의
모든 픽셀 차이는 `(175,367)..(247,387)`의 수달 문자와 후속 글자 구간에 한정된다.
노드에서는 문자 폭 7→18px와 뒤 세 요소의 x 이동만 달랐고, 표의 좌표·높이는 같았다.
같은 Print에서 해당 기호(F000으로 매핑)와 다음 `나`의 원점을 직접 읽은 전진은
18.23011px다. 따라서 raw 하락과 실제 문자 전진의 정확성을 구분해야 한다. 이 사실로
기존 표의 배치 미달을 통과 처리하거나 px 고정을 정당화하지 않는다.

`cell_boundary_object_fixture`는 본문 문단·호스트를 복제해 중첩 셀로 옮기면서 저장
LineSeg를 함께 가져오는 구조다. 수동 이동 뒤 저장 좌표가 유효한지 먼저 확인하고,
정상 저장본의 수용 조건을 완화하는 대신 입력 생성 계약과 실제 renderer 문제를 구분한다.
이 원인 가설은 아직 완료 판정이 아니며, 고정 원본의 대조 결과를 보존한 뒤 검증한다.
21개 Native 실패는 같은 producer로 묶어 노드·PNG 차이를 추가 수집했다.
`failed-tree-comparison-4221-result.json`과 독립 문자 원점의
`pr7833-base-candidate-diagnosis-4219.json`은 ignored output에 있다.
잔여 판정과 필요한 변경을 모은 뒤 한 번에 보정·빌드·재검증한다.

## 손상된 #5715 HWP 보관본 삭제 — 2026-10-26

작업지시자의 “손상된 파일 삭제” 지시에 따라
`mydocs/pr/assets/issue_5715_corrupt_sample_20261025/float_chart_ghost_ladder_gap.hwp`
보관본도 삭제했다. 삭제 전 21,504바이트와 SHA-256
`fe6570871a907d8cff07f9a672286c26ff15a42c665c315ee634a44e39ac8282`를 확인했다.
별도 보관본이나 대체 입력을 만들지 않았다. 기존 `before.hwp`·`after.hwp` 삭제도 유지한다.
이 단계에서는 활성 검사나 제품 코드를 변경하지 않았으며 #5715의 해결을 주장하지 않는다.

검증: Git 추적 대상 코드·검사·스크립트·baseline 4,664개 파일에서 삭제 대상의
활성 참조 0건을 확인했다. 변경한 README의 내부 링크 3개와 `git diff --check`도
통과했다. 로그는 ignored `output/pr-review/semanticist21-20261019/logs/`의
`corrupt-file-deletion-references-4238.log`와 `corrupt-file-deletion-links-4238.log`다.
기본 링크 검사기의 전체 탐색은 파일 I/O 지연으로 중단하고 해당 README의 링크만
한정 검사했다. 코드 변경이 없어 이 삭제 단계에서 Rust 재빌드를 실행하지 않았다.

## 남은 원 PR 재현의 공통 원인 보정 — 2026-10-26

기존 #8032의 현재 public API 산출물과 새 독립 Print를 대조했다. 큰 TAC 그림은
다음 줄에 정상 배치되지만, 앞 텍스트의 자동 줄바꿈을 확정하는 `inline_flow::plan`
이 줄 종료 이유를 보존하지 않아 양쪽 정렬 전진을 발행하지 않았다. 측정의
`InlineFlowPlan.rows/boxes`에서 한 번 확정하고 `layout_inline_flow_plan`이 같은
원점을 소비하도록 수정한다. 명시 개행·문단 끝·좁은 그림 대조군은 별도로 확인한다.
이미지 nearest 보간 진단은 오히려 85%대로 낮아져 적용하지 않았다.

#8036은 동일 원본의 base와 독립 Print가 1쪽인데 후보만 2쪽인 실제 회귀다.
이전 줄 시작이 아니라 큰 개체의 끝만으로 새 쪽을 추정하던 조건을 보정한다.
잘못된 2쪽 기대 검사는 아직 변경하지 않았으며, 새 Native/fresh WASM 시각 검증
선행 조건을 만족한 뒤 새 이름의 관계 검사로 교체한다.

#7833은 수동으로 본문 문단을 셀에 복제하면서 원래 저장 줄의 소유 권한을
무효화하지 않은 생성 문제를 확인했다. 공개 무효화 계약을 적용한 별도 진단
대조군은 8개 중 6개 Native 100%다. 남은 깊은 TAC 표 2개는 공통 폭 1200과
셀 폭 합 41954가 충돌하므로, 대체 표의 1행 격자도 공통 상자에 맞춘 대조군을
검증한다. 원래 입력·Print·미달 결과는 그대로 구분해 보존하고 현재 정식 helper의
통과로 보고하지 않는다. 아직 정식 테스트나 기대값을 변경하지 않았다.

이전 Native257/fresh074 전체 PASS와 양성 129조건은 위 새 코드의 검증으로
재사용하지 않는다. 변경 후 같은 기존 입력 범위에서 새로 확인한다.

## Native258/fresh075 대조와 검사 등록 — 2026-10-26

같은 큰 그림 API 입력은 Native 90.69451%, fresh WASM 89.93941%다. 기준 devel에서는
각각 89.19482%, 88.37924%였다. 양쪽 개선이지만 fresh가 90% 미만이므로 해당 그림
검사는 정식 등록하지 않았다. 좁은 대조군은 Native 100%, fresh 99.95997%다.
큰 표 뒤의 vpos=0/67200 두 입력은 모두 후보·base Native/fresh 각 100%·전체 1쪽이다.
대표 PNG를 두 backend에서 직접 판독하고 이어짐 입력의 같은 페이지 RGBA도 확인했다.
잘못된 기존 2쪽 모듈을 제거하고 새 이름의 관계·완료 페이지 ±2px 검사로 교체한다.
수정 전 FAIL/수정 후 PASS 실행은 `independent-regression-drafts-4257-result.json`,
전쪽 비교는 `current-source-contracts-4254-result.json`과 정식 asset에 연결했다.
그림 관련 초안은 ignored output에만 남긴다.

공개 무효화 계약을 적용한 주석 8조건은 Native 100%다. 공통 상자에 맞춰 셀 격자를
축소하거나 원래 표 template 크기를 유지하는 진단은 깊은 중첩 표 2조건을 해결하지
못했고 정식 helper에 적용하지 않았다. 기본 글꼴 정보만 추가한 #8031/#8039 대조군도
미달이어서 원본 해결이나 정식 검사 변경으로 보고하지 않는다. 원래 입력·Print를 보존한다.

원격 devel에 #8188이 병합돼 새 head는 `2546c8b2ab7041399fb15fe82e0f50c1ade21724`다.
79개 runtime 파일이 달라 코드 동등성으로 대체할 수 없다. 객체 전용 merge 사전 분석에서
충돌 18곳을 확인했다. 현재 WASM 패키지는 리베이스 전 코드로 명확히 구분하며,
관련 소스·테스트와 기능을 보존해 리베이스한 뒤 새 source로 다시 검증한다.
삭제한 sample16·편람의 옛 파생 참조를 재생성하지 않는다.

## #8188 리베이스와 우선 보존 검증

`2546c8b2ab7041399fb15fe82e0f50c1ade21724` 위로 리베이스를 완료했다.
이전 head는 `432416bf08`이며 backup ref와 전체 실행 로그는 ignored 검토 경로에 보존한다.
새 base의 원본 글꼴 프로그램·수식 좌표계·이미지 Print frame·Canvas glyph ink 및
`issue_8160` 검사는 유지했다. 삭제한 옛 sample16·편람 검사와 파생 참조를 재생성하지 않는다.

첫 Native 빌드는 기존 독립 자모 분기의 `CharWidthDecision` 초기화에 새 base의
`measurement_character` 필드가 없어 실패했다. 실제 측정 문자를 선택하지 않는
0 전진폭 분기이므로 다른 비측정 분기와 같은 `None`을 지정한다. 조판 폭은 바꾸지 않는다.
composer의 import 순서는 정식 formatter 결과를 반영한다.

사용자 지시에 따라 다음 우선 검증은 #8188의 동일한 변환본 3개 × 64쪽 × Native/fresh WASM이다.
기준 Print PDF·입력 SHA와 실제 공급 글꼴 프로그램을 원 PR 증적과 대사한다.
결과가 나오기 전에는 이 384쪽의 raw 100% 유지나 통합 PR 준비 완료를 선언하지 않는다.

### #8188의 실제 회귀와 좌표축·AST 연결 보정

우선 재측정 source `68eb7e204dc63b67f0687992bb53c6b30d6a42c1`의 Native는
3문서 전 192쪽 중 184쪽만 raw 100%였다. HWPX는 6·24·25·26·53·56쪽,
HWP2024와 HWP2022는 각각 6쪽이 달랐다. HWPX 최저는 25쪽 49.23069%다.
입력/PDF 및 실제 공급 글꼴 217개 SHA는 #8188의 `rebase-8231` 증적과 같다.
원본 글꼴 프로그램은 다른 호스트의 비공개 로컬 보존본에서 가져와 ignored output에서만
공급했다. 공개 OFL 배포본이나 재배포 허가를 확보한 파일로 취급하지 않는다.

실제 `RHWP_DIAG_FLOW=1`에서 HWPX 문단466의 저장 후보는 502.08px,
`same_frame=false`, 앞 내용과 저장 상대 간격이 인증한 원점은 486.9866667px다.
표의 실제 높이·선언 높이는 둘 다 43.4666667px로 맞는다. 높이 결함이 아니라,
같은 좌표축이 아닌 두 원점에 `max`를 적용한 현재 분기가 약15.09px를 더했다.
문단841·876에도 같은 불일치가 있다. #8188의 분기처럼 축 미일치 시에는
앞 내용으로 인증한 원점을 사용하고, 같은 축의 독립 개체 원점과 실제 내용 하한을
함께 지키는 현재 경로는 보존한다. 문서 ID·쪽 번호·목표 좌표 조건은 추가하지 않는다.

수식은 구형 파서가 명시 중괄호를 새 `Group`으로 보존하지만 #8188의
`legacy::supports`·`compound_frame` 등이 이를 모르는 연결 누락이다.
AST의 그룹 소유는 유지하며 해당 레거시 프레임의 표시 가능성·연산자/분수 구조
조회에서만 `ungrouped()`를 사용한다. 다른 세대의 파싱·그룹 글꼴 범위는 유지한다.

변경 전 같은 보관 라이브러리로 기존 `issue_8160_all_pages.rs`를 실제 실행해
3개 모두 FAIL을 확인했다. 세 원본 6쪽의 분수선 기대105.383992px 대비
101.760000px가 대표 실패다. 기존 독립 기대값과 ±2px는 바꾸지 않는다.
진단·원본 전쪽 TSV·정수 미대응 수·review/overlay 및 red 실행은 ignored 검토
경로의 4269·4272·4274·4275 기록에 있다. 보정 후 새 제품의 전쪽 검증이 필요하다.

보정 전 fresh WASM076 빌드는 완료되지 않았고 실행 세션이 exit143으로 종료됐다.
이를 최적화 완료나 fresh 출력으로 재사용하지 않으며 보정 소스를 새로 빌드한다.
