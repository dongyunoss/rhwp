# PR #8061 리뷰 — 셀 그림의 편집 소유와 쪽 그리기 순서

## 최종 판정

**메인터너 보정 후 수용 가능 — 같은 재현의 독립 Print·Native/fresh WASM 시각 검증과 새 집중 회귀를 완료했다. 새 공통 코드의 전체 제출 게이트는 추가 확인 중이다.**

원 contributor head를 직접 merge하지 않는다. 새 코드의 전체 Rust·lint·Studio 및 통합
head CI를 완료하기 전에는 통합하지 않는다. 아직 PR을 제출하거나 원 PR을 close하지 않았다.

## 접수 정보와 통합 대응

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8061](https://github.com/edwardkim/rhwp/pull/8061) / semanticist21 |
| 고정 원 head | `a9c06c8a8a526205a59c68b1ed40eb66d84ad9c2` |
| 통합 대응 | `140c38ddc5c5a87504fb562c18cec052f77ac291` |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |
| 원격 확인 | 2026-10-26 KST: 원 head 일치, reviewer jangster77 지정, draft=false |
| 새 생산 코드 | Native 257 / 최적화 fresh WASM 074.Git head와 실제 덧씌운 파일 해시는 아래 provenance에서 구분한다. |

## 원 주장과 독립 검증

원 검사는 흐르는 표의 글 뒤 그림을 쪽 단위 도형보다 항상 위에 두었다. 정상 기하 정보를
명시한 같은 재현의 독립 한컴 Print에서는 `flowWithText=0, zOrder=0` 그림이
`zOrder=1` 쪽 도형에 가려졌다. 그림의 zOrder를 2로 높이거나 flowWithText를 1로 바꾸면
그림이 위에 보였다. 떠 있는 표도 같은 구분을 따랐다. 따라서 셀의 편집 소유와 쪽/표의
그리기 소유를 분리했다. 그림 bitmap·테두리는 같은 개체의 내부 순서를 유지한다.

초기 진단 입력의 생략된 기하 정보와 레이어 의미의 결함을 구분했다. 초기 불완전한
HWPX의 raw 0.95511%를 정상 문서의 결과로 바꾸지 않았다. 생성 방식·8개 입력과 Print
PDF·producer·페이지별 TSV·대표 PNG·직접 판독은
[정식 증거](../assets/cell_picture_layer_qualified_20261026/README.md)를 따른다.

## 실제 소비 경로와 검사

- [anchored_cell_paint.rs](../../../src/renderer/layout/anchored_cell_paint.rs)의 그림 생성 후
  layer 부여 → 셀 순서 조립 → 같은 쪽 도형과 묶음 정렬 → 기존 paint replay가 소비한다.
- [render_tree.rs](../../../src/renderer/render_tree.rs)의 내부 묶음은 `serde(skip)`이며
  임시 NodeId로 z-order를 정하지 않는다. 편집 주소와 절대 좌표는 유지한다.
- [새 회귀](../../../tests/cases/verified_cell_picture_layer_scope.rs)의 8개 조건은 셀 주소,
  하나의 bitmap·PHOTO 보존, 원본 단위의 포함, 독립 Print 좌표 ±2px, 실제 그리기 순서를
  검사한다. Native 전용 1개는 같은 8조건의 실제 PNG 가림 색상도 검사한다.
- 잘못된 이전 `cell_behind_text_picture_above_paper_shapes.rs` 검사는 삭제했다.
  과거 후보 `61528013511c87c732524f91c6f224286a055337`에서 이 검사가 PASS했던 것은
  잘못된 가정을 검출하지 못한 결과다. 해당 PASS를 현재 결함의 검증 근거로 쓰지 않는다.

## 실제 결과 — 2026-10-26

| 검증 | 실제 결과 |
| --- | --- |
| 같은 재현의 속성 대조 8조건 | Native/fresh WASM 각각 전쪽 raw 100%, 각각 독립 Print와 1쪽 일치 |
| 고정 base와 페이지별 비교 | 8조건 × 2 backend, 0.1%p 초과 하락 0쪽 |
| 실제 레이어 결함의 검출 | 독립 쪽 그림 2조건·테두리 1조건, 수정 전 FAIL / 수정 후 PASS |
| 새 집중 회귀 | 셀 그림 9개 + 정상 RowBreak 1개 = 10 PASS·0 FAIL |
| 새 공통 코드의 선행 영향 검증 | 15조건·104쪽/backend, 전쪽 90% 이상·쪽수 일치 |
| 기존 영향 문서·RowBreak의 base 비교 | 7조건·96쪽/backend, 0.1%p 초과 하락 0쪽 |
| 직접 판독 | 8조건의 동일 영역 및 대표 양쪽 정본 review·standalone overlay 확인 |

raw 100%는 RGB 동일을 뜻하지 않는다. 불투명 영역의 색상 가림은 별도 PNG 검사로
보호했다. PHOTO의 작은 래스터 경계와 떠 있는 그림의 약 1.12px 원점 차이는 남으며
독립 좌표 ±2px 안이다. 점수만으로 차이를 숨기거나 좌표를 clamp하지 않았다.

같은 원본 HWPUNIT 좌표가 f64 연산 뒤 약 1e-14px 달라지는 관계 assertion 오류는
원본 정수 단위에서 동등·포함을 비교하도록 바로잡았다. 독립 Print의 ±2px 조건은
유지했다. 수정 후 focused nextest의 최종 exit 0과 10 PASS·0 FAIL을 확인했다.

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
`cell-scope-focused-4175-result.json`, `logs/cell-scope-focused-4175.log`,
`cell-scope-negative-proof-4176.json`, `cell-scope-base-comparison-4179.json`에 종료와
수정 전후 근거가 있다. [파일별 생산 코드·입력·TSV·PNG 해시](../assets/cell_picture_layer_qualified_20261026/provenance.json)를
고정했다. 과거 코드의 전체 nextest 11,708 PASS와 Studio 결과를 새 코드의 전체 완료로
재사용하지 않는다.

## 제외와 후속 처리

sample16·편람의 기존 삭제를 유지하며 이전 파생 참조를 재생성하지 않는다.
새 입력은 같은 #8061 재현의 8조건뿐이며 다른 실물 HWP를 수집하지 않았다.
원 PR별 기록은 통합 PR에 유지하고, 실제 code head CI 뒤 승인된 옵션 2의
self-review·오늘할일 후속 PR을 처리한다. merge 뒤 실제 merge SHA의 증적·통합 링크와
보정 결과를 원 PR에 남긴다. contributor fork·branch는 보존한다.
[개별 구현 기록](pr_8061_review_impl.md)을 따른다.
