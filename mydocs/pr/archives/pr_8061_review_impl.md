# PR #8061 구현·검증 기록

## 범위와 고정 기준

[개별 리뷰](pr_8061_review.md)의 원 head `a9c06c8a8a526205a59c68b1ed40eb66d84ad9c2`와 실제 소비 경로를 따른다. `review/semanticist21-20261019`에서 고정 devel `238de6de41a208648f4ade6b494fe433a81c1fce` 위로 누적한 45개 원 PR 중 하나다. 추가 PR을 체리픽하거나 다른 HWP로 범위를 확대하지 않는다.

## 원 커밋과 통합 대응

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `a9c06c8a8a526205a59c68b1ed40eb66d84ad9c2` | `140c38ddc5c5a87504fb562c18cec052f77ac291` |

충돌 보정과 원 기능의 유지 여부는 원 커밋·현재 코드 후보·독립 기대값을 대조한다. 메인터너의 공통 renderer·편집 소유 보정과 원 contributor 변경을 혼동하지 않는다. 원 head는 직접 merge 대상이 아니다.

## 실행 순서와 현재 상태

1. 원 PR의 reviewer·head를 확인하고 기능 commit을 최신 devel 위에 누적 적용했다. 원격 원 head를 재작성하지 않았다.
2. 코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 로컬 Rust 검증을 완료했다. 렌더 source는 `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`이며 이후 증거 등록은 런타임을 바꾸지 않았다. 미완료 항목을 완료로 표시하지 않는다.
3. 개별 리뷰의 독립 Print·Native/fresh WASM·소비 경로와 전체 로컬 게이트를 완료한 뒤 최종 판정을 갱신한다. PR 직전 devel head를 재조회하고 변경됐으면 리베이스한 기능과 현재 보정 범위를 함께 검증한다.
4. 승인된 통합 PR을 실제 템플릿에 맞춰 제출하고 정확한 code head CI를 관찰한다. 별도로 승인된 옵션 2의 self-review·오늘할일 후속 PR은 code CI 뒤에 처리한다. 원 PR별 검토 기록은 통합 PR에 유지한다.
5. CI·merge 조건을 확인해 통합하고 merge SHA의 devel 포함을 확인한다. 이 PR에는 보정·검증 결과와 통합 링크를 남긴 뒤 원 PR을 close한다. 관련 이슈는 전체 해결 범위가 확인된 경우만 닫는다.
6. contributor fork·branch를 보존하고 소유·실행 상태를 확인한 검토 worktree·임시 산출물만 정리한다. 공유 `target/pr-review`는 유지한다.

## 제외와 복원 경계

sample16·편람의 삭제된 회귀·파생 참조는 복구하지 않는다. 별도 이관 검사를 이 통합의 통과 증거로 다시 등록하지 않는다. 통합 전 코드 변경이 필요하면 일반 추가 commit으로 보정하고 영향 검증을 재실행한다. contributor history를 reset·amend·force-push하지 않는다.

## 완료 전 남은 기록

정확한 전체 검증 terminal 결과, 적용 입력의 시각 증거·직접 판독과 최종 판정, 실제 통합 PR·CI·merge SHA·후속 comment URL은 수행 후 기록한다. 이 계획 자체를 해당 단계의 완료로 간주하지 않는다.

## 독립 Print의 레이어 반례 — 2026-10-26

초기 진단용 HWPX는 원 검사와 같이 사각형의 원래 크기·꼭짓점, 그림의 원본 크기를
생략했다. RHWP는 선언된 배치 크기로 표시했지만 한컴 Print에는 두 개체가 모두
나오지 않았다. 이 초기 입력의 raw 0.95511%를 정상 한컴 문서의 회귀로 해석하지 않는다.
원본 바이트는 보존하고, 선언된 크기의 사각형 꼭짓점과 4×4 PNG의 원본 크기를
명시한 별도 대조군을 만들었다. 소유·배치·색상·텍스트·BinData는 그대로 두었다.
그림의 75 HWPUNIT/CSS px 변환은 기존 정상 삽입본의 16px/1,200 HWPUNIT 대응과
대조했다. 보완본 통과를 초기 입력 자체의 일치로 보고하지 않는다.

같은 보완본에서 한 번에 속성 하나만 바꿔 MCP engine 2020의 한컴 11.0.0.9136으로
Print했다. 모든 결과는 `pdf_print_method=0`,
`pdf_output_mode=hancom2020_pdf_driver_one_up`, 입력 전처리 없음, 1쪽이다.

| 조건 | 독립 Print 관측 | 현재 Native·fresh WASM 관측 |
| --- | --- | --- |
| 그림 `flowWithText=0, zOrder=0`, 쪽 도형 `zOrder=1` | 녹색 도형에 빨간 그림이 가려짐 | 빨간 그림이 도형 위에 보임: 불일치 |
| 쪽 도형만 제거 | 빨간 그림이 보임 | 빨간 그림이 보임 |
| 그림 `zOrder`만 2로 변경 | 빨간 그림이 도형 위에 보임 | 빨간 그림이 도형 위에 보임 |
| 그림 `flowWithText`만 1로 변경 | 빨간 그림이 도형 위에 보임 | 빨간 그림이 도형 위에 보임 |

4조건의 후보/base × Native/fresh WASM은 16회 모두 raw 100%, 1쪽 일치다.
하지만 불투명한 녹색·빨간 영역은 실루엣이 같아 첫 행의 색상·레이어 결함을 점수로
검출하지 못한다. 고정 base에서도 같은 차이가 관측되므로 이 결과를 현재 브랜치에서
새로 생긴 회귀라고 단정하지 않는다. 독립 Print와 실제 출력의 차이는 남아 있으므로
원 PR의 검사가 올바른 조건을 고정하는지와 공통 레이어 소비 조건을 보정해야 한다.
실루엣 통과나 전체 Rust PASS를 수용 판정으로 대신하지 않았다.

실제 소비 경로는 `layout/picture_footnote.rs`의 셀 그림 layer 부여 →
`layout.rs::order_cell_anchored_objects`의 BehindText 분류·상속 layer 및
`ImageNode.text_wrap` 덮어쓰기 → `paint/replay_order.rs`의 최종 replay plane이다.
현재 분류 조건에는 `flow_with_text` 구분이 없다. 셀의 편집 소유와 쪽/표의 그리기
계층을 같은 상태로 간주해도 되는지 위 반례에 따라 보정·재검증한다.

증적 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
`pr8061-explicit-geometry-4144/provenance.json`에 보완 전후 입력 해시·변경 ZIP 항목,
`pr8061-layer-controls-print-4147-result.json`에 세 Print 작업의 동일 job ID·출력 해시·
실제 engine 계약, `pr8061-explicit-validation-4146-result.json` 및
`pr8061-controls-validation-4149-result.json`에 16회 TSV·producer·페이지 수를 기록했다.
새로운 실물 문서로 범위를 넓히거나 sample16·편람의 파생 자료를 만들지 않았다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/cell_behind_text_picture_above_paper_shapes.rs](../../../tests/cases/cell_behind_text_picture_above_paper_shapes.rs): SHA-256 `91529c3a53126706559223060bcfd63a02725e57c06bd3f50e17b77a8b46f6a0`.
  - `flow_cell_behind_text_picture_paints_above_paper_behind_text_shape`: PASS, `logs/final-full-nextest-4052.log:11206` (`rhwp::regression_suite_027`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `a9c06c8a8a526205a59c68b1ed40eb66d84ad9c2` | `140c38ddc5c5a87504fb562c18cec052f77ac291` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
