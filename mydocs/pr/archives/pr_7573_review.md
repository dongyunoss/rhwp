---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7573_review.md
last_verified: 2026-10-21
---

# PR #7573 검토 — 오려 붙이기 선택 경계의 표 소유 보존

## 최종 판정

**머지 보류 — 메인터너 보정 후 개체 보존 검사는 통과했으며 최신 통합 검증을 진행 중이다.**
원 Draft는 엔진 수정이 없는 실패 재현이다. 보정 후보의 필수 전체 게이트와 최신 독립
출력 확인을 완료한 뒤 판정하며, 원격 게시·PR 생성·merge는 수행하지 않았다.

## 접수 정보와 적용 계보

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7573](https://github.com/edwardkim/rhwp/pull/7573), semanticist21, devel 대상 Draft |
| 2026-10-21 재조회 head | `84e80b5d82455254bdbdaca0a2b19dff92f1be12` — 동결한 검토 범위와 같음 |
| 기능 commit → 로컬 적용 | `006884716b10166dbc1f31b4541b0277f520cbd5` → `58816a24089aadeb543394672e53a6ea76233964` |
| 메인터너 보정 | `0eb36de6d575e27e3381a85ec86b8363568d7fbf` |
| reviewer | jangster77; 지정 영수증과 현재 요청 목록 확인 |
| 관련 범위 | #7444의 선택 경계 개체; #7509의 개체 뒤 입력·복사와 구분 |

원문·commit·변경 파일 영수증은 ignored output의 `pr7573-current-review-1325.json`,
reviewer 지정 기록은 `pr-7573-reviewers.json`이다. 원 head의 devel merge는 기능 적용과 구분했다.

## 원 주장·공통 보정·반례

`copySelection`과 `deleteRange`가 선택의 경계 표를 다르게 소유하면 같은 범위를 오려
빈 문단에 붙일 때 표가 둘로 늘어난다. 원 PR은 공개 API로 `abc`·표·`xyz`·빈 문단을
생성하고 정상 중간 선택1개와 양 끝 경계 실패2개를 제시한다.

보정은 `commands/text_editing.rs`의 공통 `delete_paragraph_boundary`에서 복사와 같은
반개방 글자 범위를 사용한다. 문단 끝까지 선택한 경우 후행 개체를 포함하며, 경계 안의
표·그림을 역순으로 제거한 뒤 텍스트를 삭제한다. 개체 슬롯 제거는 모델의 공통
`remove_control`을 사용해 글자 모양과 필드의 control 번호도 함께 갱신한다. 같은 문단,
여러 문단과 셀 경계 삭제가 이 연산을 소비한다. 특정 문서·표 번호의 예외를 추가하지 않는다.

## 실제 검사와 입력 증거

`tests/cases/issue_7444_cut_boundary_object.rs`의 세 검사는 선택 중간, 선택 끝의 표 문단
시작, 선택 시작의 표 문단 시작을 각각 실행한다. 복사→삭제→빈 문단 붙이기를 실제로
호출하고 문서 전체 표 수를 검사한다. 전체 원문·서식 보존과 모든 개체 종류를 이 세
수량 검사만으로 입증하지 않으며, 해당 경계의 내용·소유·원문 보존 반례를 추가 대조한다.

`a524b12732be30d556920ccbf1102e89de538bf7`의 전체 Rust1180에서 세 검사는 모두 PASS했다.
최신 제품의 집중 실행과 최종 전체 검증은 별도로 완료한다. 원 구현에서 두 경계가
의도한 원인으로 실패한 실행 증거를 연결하며, 원 PR의 주장만을 독립 실행으로 세지 않는다.

API 생성 HWPX의 `pr7573-before`, `pr7573-after-interior`, `pr7573-after-start-boundary`,
`pr7573-after-end-boundary`와 각각의 독립 한컴 Print PDF를 대조한019 원장은 Native/fresh
양쪽 전쪽 raw100%다. 생산 source는 `73b33fde72642faf71c7060bceba47358520262b`이며
최신 후보 결과로 쓰지 않는다. 입력·PDF·TSV 해시와 명령은 ignored
`integration-fixtures-019-native/results.json`·`integration-fixtures-019-wasm/results.json`에 있다.
최종 제출 전에 최신 양쪽 전쪽·직접 판독·devel 무회귀와 안정 경로의 입력 보존을 완료한다.

## 조판 원칙과 후속 판정

소유·내용 순서·누락·중복이 주 검사다. 텍스트 좌표가 비슷하거나 쪽수가 같다는 사실로
표 보존을 대신하지 않는다. 이 PR 자체는 표 분할·이어받기 구현을 변경하지 않는다.
삭제한 개체로 조판이 달라지므로 시각 검증은 적용하며, 최종 통합 게이트를 미완료로
남긴 채 PR을 제출하지 않는다. #7444 전체의 해결이나 close를 이 재현 범위로 단정하지 않는다.

## Merge 후 contributor comment 계획

실제 통합 승인·완료 뒤 경계 소유 차이를 검출한 원 기여와 공통 삭제 연산의 보정을
구분해 한국어 존댓말로 설명한다. 실제 merge SHA·CI와 같은 SHA의 대표 review·overlay,
전체 TSV를 연결한 뒤 원 PR을 처리한다. 현재는 게시·close를 수행하지 않았다.

## 수정 전후의 실제 검출 증거 재확인

초기 집중 실행002의 source `119f1d9fdf4daec7336a888019829365e7472656`에서
두 결함 검사는 각각 FAIL, 정상 대조군은 PASS했다. 보정 뒤003의 source
`7614917f08880ca937f2d933540d87299eaef6b9`에서는 세 검사 모두 PASS했다.
이 옛 SHA와 현재 계보의 `0eb36de6d5`는 stable patch-id
`a28524be417518740347130b49a9a0edd5668000`가 같으며 검사 파일 바이트도 전후·현재가 같다.
`batch-cut-before-after-proof-1334.json`에 이 연결을 기록했다. 기존에 충분한 실행
증거를 재사용했으며 재컴파일만을 위해 같은 옛 소스를 다시 빌드하지 않았다.

현재 Native087의 strict 집중 실행1333도 해당3검사 모두 PASS했다. 편집 저장본의
최신 Native1328은 모든 관련 페이지100%다. fresh036 대조와 최종 전체 게이트는 계속한다.

### 최신 제품의 실제 재검증

제품 소스 `43abfbab6748e5345d848de8661098dc5b59b10e`의 전체 Rust1352에서도
`issue_7444_cut_boundary_object`의 세 검사는 모두 PASS했다. 통합 후보의 전체 실행은
다른59개 실패가 있어 통과로 보고하지 않는다. 같은 소스의 Native088·최적화 fresh037에서
위 네 편집 저장본을 전쪽 재출력했고 모두 raw100%·독립 Print PDF 쪽수 일치를 확인했다.
`batch-cut-native-1347/results.json`과 `batch-cut-wasm-1347/results.json`이 최신 실행이며
옛019·1328의 결과와 구분한다. 각 원장에 실제 명령과 입력·PDF·TSV·producer 해시가 있다.
대표 PNG 직접 판독·최신 devel 대조·제출 경로 보존과 통합 후보의 나머지 필수 게이트를
완료한 뒤 최종 판정을 갱신한다. 현재 판정은 머지 보류다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/issue_7444_cut_boundary_object.rs](../../../tests/cases/issue_7444_cut_boundary_object.rs): SHA-256 `41337da29e671877401935322a39efff28c0a1e00b7440a2ecf66c1d28973040`.
  - `cut_with_table_in_selection_interior_moves_one_table`: PASS, `logs/final-full-nextest-4052.log:10791` (`rhwp::regression_suite_025`).
  - `cut_ending_at_table_paragraph_start_does_not_duplicate_table`: PASS, `logs/final-full-nextest-4052.log:10788` (`rhwp::regression_suite_025`).
  - `cut_starting_at_table_paragraph_start_does_not_duplicate_table`: PASS, `logs/final-full-nextest-4052.log:10790` (`rhwp::regression_suite_025`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `006884716b10166dbc1f31b4541b0277f520cbd5` | `415772049558c46e1600e7daa1dfb93217726383` |
| `84e80b5d82455254bdbdaca0a2b19dff92f1be12` | 원 devel merge; 기능 체리픽에서 제외 |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
