---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7589_review.md
last_verified: 2026-10-26
---

# PR #7589 검토 — 입력 상자의 저장 개행과 Print 표시

## 최종 판정

**메인터너 보정 후 수용 가능 — 양식 입력 상자의 실제 Print 첫 줄 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 아래 현재 증거와
실제 검사 실행을 대조했다. 통합 PR의 정확한 code head CI·merge·후속 처리는
앞으로 수행할 조건이며 로컬 통과로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

원 head `300d1af385064eb32bb85c94f3a40ab7213d2d2a`가 원격과 같음을 재확인했다. 검증 코드는
`61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 제출 전 fetch에서도 같은 base를
확인했으므로 추가 리베이스하지 않았다. 이후 문서·증거 커밋은 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 원 PR의 현재 검사 | 아래 모듈·함수별 실제 PASS와 로그 행을 확인했다. 소스 존재를 실행 결과로 대신하지 않았다. |
| 전쪽 Native/fresh WASM | 2조건·각 1쪽(전체 2쪽/backend)에서 최저 100.00000% / 100.00000%, 독립 Print 쪽수 일치·90% 미만/누락 0쪽 |
| 고정 devel 보존 | 동일 입력·Print·글꼴·backend의 페이지별 비교에서 0.1%p 초과 하락 0쪽 |
| 직접 판독 | 사용자가 수동 Print한 두 PDF 모두 FIRST만 출력했다. 원 작성자의 multiline=true이면 모든 줄이 인쇄된다는 가정은 수용하지 않고 실제 한컴 인쇄 계약과 비교했다. |
| 전체 로컬 게이트 | fmt·수집/단위 정책·Release build/lib·전체 nextest·Skia 3종·Native/WASM/workspace Clippy·workspace build·doctest 완료. 실제 종료 근거는 아래 공통 기록 참조 |
| 원격 절차 | 통합 PR의 정확한 code head CI·실제 merge·후속 comment는 앞으로 수행할 조건이다. 원 contributor head를 직접 승인·merge한 기록이 아니다. |

조건 번호 057, 058. [전쪽 TSV·원본/Print·producer·해시와 판독](../assets/source_pr_qualified_20261026/README.md)을
이 PR의 직접 근거로 연결한다. raw는 색상 경계 보정 전 2px 실루엣이며 RGB 완전 일치나
과거 별도 지표의 통과를 뜻하지 않는다. Native255·표준 release/wasm-opt를 마친 fresh073을
사용했다. 측정 뒤 코드 변경은 없으며, 과거 제품의 표는 당시 결과로 구분한다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/source_pr_qualified_20261026/png/native/057/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/native/057/overlay_001.png) |
| fresh WASM | [review](../assets/source_pr_qualified_20261026/png/fresh-wasm/057/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/fresh-wasm/057/overlay_001.png) |

Merge 후에는 이 대표 파일을
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/source_pr_qualified_20261026/`
위 표의 실제 상대 경로로 고정해 contributor comment에 두 이미지를 함께 표시한다.
같은 merge의 devel 포함·asset 존재·CI 결과를 먼저 확인하고 한국어 존댓말의
`--body-file` 본문을 게시한 뒤 API에서 한글·링크·실제 Markdown을 다시 확인한다.


## 접수 정보와 적용 계보

이하 최초 접수와 중간 검토의 제품·기준 SHA 및 당시 진행 상태를 보존한다.
현재 판정은 위 「최종 로컬 판정 근거」와 아래 「통합 후보의 실제 검사 실행」이 정본이다.
과거의 진행 중·미완료 표현을 현재 로컬 게이트 상태로 사용하지 않는다.

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7589](https://github.com/edwardkim/rhwp/pull/7589), semanticist21, devel 대상 Draft |
| 2026-10-21 조회 head | `300d1af385064eb32bb85c94f3a40ab7213d2d2a` |
| 기능 commit | `b03f3072bf81ffa83ee43e6cc77cf8c7f2e10aef` |
| 로컬 cherry-pick | `cb0118ccf1`; head의 나머지 commit은 devel merge이므로 체리픽하지 않음 |
| reviewer | jangster77; 요청 API 영수증으로 지정 확인 |
| 메인터너 보정 | `7ee2327f7c87a5d1263b480b3ad27025874bc96a`, `612c09bae4453bc3747816c51f4c4a9cfff3581d` |
| 현재 통합 branch | `review/semanticist21-20261019` |

최신 PR 본문·commit·파일 목록은 ignored output의
`pr7589-current-review-1106.json`, reviewer 영수증은 `pr-7589-reviewers.json`이다.
원 PR의 본문에 적힌 예전 test head와 실제 조회한 merge head를 구분했다.

## 구현 주장과 독립 기대값

원 PR은 같은 `FIRST\nSECOND`에서 MultiLine=1/0의 SVG가 달라야 한다고 주장한다.
파서·직렬화가 개행과 속성을 보존하는지 검사한 점은 유지한다. 그러나 그 뒤의
`assert_ne!`는 이 입력의 한컴 인쇄 동작을 입증하지 않는다.

사용자가 두 HWPX를 수정 없이 한컴의 파일 → 인쇄 → PDF로 출력했고,
win10-ted의 `C:\temp`에서 원본 바이트를 가져왔다. 두 PDF는 각각 A4 한 쪽이며
실제 Creator는 `Hwp 2022 12.0.0.4605`다. 파일명의2020과 실제 제품을 구분한다.
두 Print의 래스터는 동일하고 입력 상자에는 `FIRST`만 표시된다.

공통 `FormObjectNode`의 표시 문자열에서 첫 저장 줄을 선택하되 모델 원문과
MultiLine 속성은 보존하도록 보정했다. SVG·Canvas·Native가 같은 표시 결과를
소비하며 특정 문서나 좌표에 따른 예외는 없다. 모든 입력 상자의 편집 중 화면까지
MultiLine이 무의미하다고 일반화하지 않는다.

## 입력·시각 증거와 회귀 검사

네 입력/PDF와 각 SHA·바이트 수·생산자 해시·원문 TSV·review/overlay는
[입력 상자 Print 증거](../assets/pr_7589_form_multiline/README.md)에 있다.
기존의 동일한 커밋 파일을 재사용하며 별도 사본을 추가하지 않았다.

증적 source `fca7ff627b7d05d6edb8dade112ead8b43c2db6f`에서 두 입력 모두
Native/fresh WASM raw100%, 전체 쪽수1=1이고 같은 영역의 직접 판독도 완료했다.
이 과거 생산 증거를 최신 통합 head의 재검증 결과로 바꾸어 쓰지 않는다.

`tests/cases/form_multiline_rendering.rs`는 원문·속성 보존과 FIRST 표시,
SECOND 미표시, 두 결과의 동일성을 함께 검사한다. 동일성만 검사하면 이전의 잘못된
연결 출력도 통과하므로 단독 검출 근거로 쓰지 않는다. 절대 px 위치는 고정하지 않는다.

`775de4c16fa2cc5e36fc588e7f500625e7a4939c`의 전체 Rust1085에서는 이 파일의
두 검사 모두 PASS했다. 전체 실행은 별도62실패가 있어 통과로 보고하지 않는다.
이후 공통 줄 투영 보정 `96038d8647`의 실제 소스 검증과 최종 시각 대조를 계속한다.

## 조판 원칙과 남은 필수 검증

| 항목 | 판정·근거 |
| --- | --- |
| 원 주장·기대값 대조 | 원 SVG 차이 기대값은 독립 Print와 불일치; 보정 검사에서 표시와 저장을 구분 |
| 측정·표시 소비 | 공통 FormObjectNode의 표시 문자열을 각 backend가 소비 |
| 일반성·반례 | MultiLine=0/1, 저장 개행 있음/없음; 원문은 보존 |
| 분할·이어받기 | 이 PR 자체는 비해당; 통합 후보의 공통 줄 변경은 별도 필수 검증 |
| 시각 선행 조건 | 기존 증적 source에서 양쪽100%·직접 판독 확인; 최신 head의 최종 검증은 진행 중 |
| 최종 통합 gate | 미완료; 전체 필수 검증과 최신 devel 대비 하락 검사를 마친 뒤 판정을 갱신 |

## Merge 후 contributor comment 계획

실제 통합 merge가 승인·완료된 뒤 원 재현 검사의 기여, 독립 Print로 기대값을 보정한
이유와 범위를 한국어 존댓말로 설명한다. 실제 merge SHA·검증 head CI와 원 PR의 연결을
적고 위 대표 Native/WASM review·overlay를 merge SHA 고정 raw URL의 이미지로 표시한다.
그 뒤 원 PR을 통합 PR과 연결해 처리한다. 현재는 문안 게시나 PR close를 실행하지 않았다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/form_multiline_rendering.rs](../../../tests/cases/form_multiline_rendering.rs): SHA-256 `ae0297fb8e81b93876e23a756dea1abb80bc6ae046d5ecd21f81f9cf2dd0c01c`.
  - `edit_print_displays_first_stored_line_for_both_multiline_values`: PASS, `logs/final-full-nextest-4052.log:7495` (`rhwp::regression_suite_013`).
  - `multiline_edit_without_line_break_keeps_single_line_output`: PASS, `logs/final-full-nextest-4052.log:7497` (`rhwp::regression_suite_013`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `b03f3072bf81ffa83ee43e6cc77cf8c7f2e10aef` | `d68c870ad15946f381d44be54eee8a322e4147c6` |
| `300d1af385064eb32bb85c94f3a40ab7213d2d2a` | 원 devel merge; 기능 체리픽에서 제외 |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
