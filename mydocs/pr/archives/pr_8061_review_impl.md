# PR #8061 구현·검증 기록

## 원 주장과 고정 기준

원 head `a9c06c8a8a526205a59c68b1ed40eb66d84ad9c2`를 통합 대응
`140c38ddc5c5a87504fb562c18cec052f77ac291`로 적용했다.
브랜치는 `review/semanticist21-20261019`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 누적 원 PR 45개 외의
체리픽이나 실물 HWP 범위 확대는 하지 않는다.

원 검사는 셀에 속한 글 뒤 그림을 쪽의 글 뒤 도형보다 항상 위에 그려야 한다고
가정했다. 과거 전체 Rust에서 이 검사가 PASS한 사실은 독립 Print와의 일치를
입증하지 못했다. 기하 정보가 생략된 초기 HWPX와 정상 기하 정보를 명시한 별도
대조군을 구분하고, 속성 차이와 독립 Print로 원 가정을 검증했다.

## 독립 Print의 레이어 반례 — 2026-10-26

초기 입력은 사각형의 원래 크기·꼭짓점과 그림 원본 크기가 없었다. RHWP에는
표시됐으나 한컴에는 PHOTO만 인쇄돼 raw 0.95511%였다. 이 결과는 보존하고,
소유·배치·색상·텍스트·BinData는 유지한 별도 기하 보완본을 사용했다.
4px PNG의 원본 크기 300 HWPUNIT은 정상 삽입본의 16px/1,200 HWPUNIT과 대조했다.
초기 입력을 정상 한컴 저장본으로 부르거나 보완본의 점수로 덮어쓰지 않았다.

같은 보완본에서 한 번에 속성 하나를 바꾸고 MCP engine 2020의 한컴
11.0.0.9136 Print로 출력했다. `pdf_print_method=0`,
`hancom2020_pdf_driver_one_up`, 입력 전처리 없음, 전체 1쪽이다.
떠 있는 표에서는 쪽 도형과의 겹침을 유지하도록 원본 위치를 명시했다.
8조건의 입력·Print·해시·관측표는 [정식 증거](../assets/cell_picture_layer_qualified_20261026/README.md)에 있다.

`flowWithText=0, zOrder=0` 그림은 더 높은 쪽 도형에 가려졌고,
flowWithText=1 또는 그림 zOrder=2에서는 보였다. 흐르는 표와 떠 있는 표 양쪽에서
같은 구분이 확인됐다. 테두리만 추가한 대조군은 bitmap 다음에 테두리가 그려졌다.
같은 불투명 영역은 가림 순서가 틀려도 raw 100%여서 실제 색상·순서도 판독했다.

## 공통 구현과 소비 경로

[anchored_cell_paint.rs](../../../src/renderer/layout/anchored_cell_paint.rs)로 기존
`layout_picture` wrapper와 셀 순서 조립을 옮겼다. 측정·좌표를 문서별로 보정하지 않는다.

1. 기존 `layout_picture_full`이 만든 bitmap·테두리·캡션에 원본 그림의 layer를 부여한다.
2. 일반 셀의 비-TAC BehindText 그림에서 flowWithText=0이고 기존 셀 프레임 계약에
   해당하지 않으면 내부 묶음 소속을 기록한다. 셀의 편집 주소와 절대 좌표는 유지한다.
3. 셀 순서 조립 시 그 묶음을 같은 쪽의 도형들과 정렬한다. 흐름을 따르는 그림과
   기존 셀 프레임 paint는 표의 그리기 계층을 유지한다. 보이지 않는 부모의 자식은 올리지 않는다.
4. 한 그림의 부분들은 같은 원본 DocPath 정렬키와 내부 순서를 유지한다. 주소가 없는
   테두리가 bitmap보다 앞서지 않는다. 묶음 식별용 NodeId는 정렬에 쓰지 않는다.
5. 기존 paint replay가 조립된 순서를 소비한다. Native와 fresh WASM에 별도 보정식을
   추가하지 않는다. RenderLayerInfo의 내부 필드는 `serde(skip)`으로 공개 JSON에서 제외한다.

실제 공통 파일 네 개의 SHA-256과 생산물 해시는
[provenance.json](../assets/cell_picture_layer_qualified_20261026/provenance.json)에 있다.
빌드 당시 main HEAD와 검증 worktree HEAD, 실제 파일 overlay를 구분했다.
Native 257과 표준 release·wasm-opt 완료 fresh WASM 074는 같은 런타임 소스를 소비했다.

## 실제 시각 검증과 새 회귀

8조건 모두 Native/fresh WASM raw 100%·Print와 1쪽 일치이며 base 대비
0.1%p 초과 하락 0쪽이다. 기존 정상 RowBreak 2쪽도 양쪽 raw 100%다.
기존 114조건의 실제 tree에서 셀 그림이 있는 6조건·94쪽을 추려 재출력했다.
선행 검증 합계는 15조건·104쪽/backend, 전쪽 90% 이상·쪽수 일치다.
영향 6조건과 RowBreak까지의 192쪽 base 비교도 0.1%p 초과 하락 0쪽이다.
범위 밖 HWP·sample16·편람을 추가하지 않았다.

잘못된 `cell_behind_text_picture_above_paper_shapes.rs`는 삭제했다.
[새 이름의 검사](../../../tests/cases/verified_cell_picture_layer_scope.rs)는
8조건의 소유·보존·포함·독립 PDF 좌표·SVG 순서와 Native 실제 PNG 가림을 보호한다.
독립 쪽 그림 2조건과 테두리 1조건의 순서 검사는 수정 전 FAIL / 수정 후 PASS다.
SVG는 구조 검사에만 쓰고 시각 검증은 Native/fresh PNG로 수행했다.
[정상 RowBreak 검사](../../../tests/cases/verified_rowbreak_successor_print_frame.rs)도
50/10행·셀 120개 보존·뒤 문단·독립 Print 외곽과 기준선을 함께 보호한다.

첫 실행의 8개 관계 실패는 동일 좌표의 약 1e-14px 부동소수점 차이였다.
원본 정수 HWPUNIT의 동등·포함으로 바로잡았으며 독립 PDF ±2px는 유지했다.
최종 focused nextest는 exit 0, 10 PASS·0 FAIL이다. jobs 8·test threads 12·공용
`target/pr-review`를 사용했다. 각 쪽 양쪽 raw 100%와 직접 판독을 먼저 완료했으므로
좌표 검사 자격을 충족하며 관계 검사도 함께 유지한다.

## 실행 증거와 남은 절차

정식 증거 폴더에는 원문 TSV·대표 PNG·독립 기대 좌표·간결한 provenance만 등록했다.
로그·전체 inventory·중간 집계는 ignored `output/pr-review/semanticist21-20261019/`에 둔다.
주요 영수증은 `cell-scope-focused-4175-result.json`,
`cell-scope-negative-proof-4176.json`, `cell-scope-base-comparison-4179.json`,
`cell-layer-refresh-validation-4166-result.json`이다.

새 공통 소스의 전체 Rust·lint·Studio·최종 제출 범위 검증을 마친 뒤 통합 head를
확정한다. 과거 후보의 전체 PASS를 새 코드에 적용하지 않는다. PR 직전 base가 바뀌면
#8228 변경을 우선 보존하고 양쪽 기능을 재검증한다. 실제 통합 code CI 뒤 옵션 2의
self-review·오늘할일 후속 PR을 처리하고, merge 뒤 원 PR에 실제 증적과 링크를 남긴다.
아직 PR 생성·원격 CI·merge·close를 완료했다고 기록하지 않는다.

sample16·편람의 삭제와 이전 파생 참조 부재는 검증 worktree에서도 유지한다.
주 작업 트리의 과거 ignored generated 파일은 실행·복원·stage하지 않으며 실제 수집 소스에서
메모리로 계산한 신규 harness와 준비한 검증 worktree를 대조한다.

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/verified_cell_picture_layer_scope.rs](../../../tests/cases/verified_cell_picture_layer_scope.rs): SHA-256 `943d352051dc56854faf4fec87c6768f615f4e176f5930f298159ea86ab624de`.
  - `verified_cell_picture_bitmap_precedes_its_own_border`: PASS, `logs/cell-scope-focused-4175.log:8` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_floating_higher_z_preserves_page_order`: PASS, `logs/cell-scope-focused-4175.log:9` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_floating_owner_preserves_page_order`: PASS, `logs/cell-scope-focused-4175.log:10` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_floating_text_flow_preserves_owner_plane`: PASS, `logs/cell-scope-focused-4175.log:7` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_following_text_inherits_cell_paint`: PASS, `logs/cell-scope-focused-4175.log:12` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_higher_page_z_order_remains_above_shape`: PASS, `logs/cell-scope-focused-4175.log:11` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_page_shape_occludes_independent_picture`: PASS, `logs/cell-scope-focused-4175.log:13` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_print_pixels_match_independent_occlusion`: PASS, `logs/cell-scope-focused-4175.log:15` (`rhwp::regression_suite_013`).
  - `verified_cell_picture_without_occluder_remains_visible`: PASS, `logs/cell-scope-focused-4175.log:6` (`rhwp::regression_suite_013`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
