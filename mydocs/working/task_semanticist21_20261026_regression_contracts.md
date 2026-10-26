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
