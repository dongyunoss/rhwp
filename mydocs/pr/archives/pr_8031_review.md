# PR #8031 리뷰 — fix(pagination): 표 셀 안의 쪽 번호 위치도 읽는다

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8031](https://github.com/edwardkim/rhwp/pull/8031) / semanticist21 |
| 고정 원 head | `3b79ee07292b6d386af1ee9e909e0a5549ec52f8` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 이전 검증 후보(과거) | `d6ea10837218e7e490b166ed76af221490b61ba2` |
| 초기 렌더 소스·증거 등록(과거) | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 이전 고정 devel(과거) | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `3b79ee07292b6d386af1ee9e909e0a5549ec52f8` | `e70dbe177571863d533c72237feceb67344605d6` |

### 실제 소비 경로와 반례

collect_page_number_pos는 문단·control·cell을 역순으로 찾아 기존 마지막 선언 우선순위를 중첩 표 셀까지 확장한다. 머리말·꼬리말의 내용 자체를 구역 선언으로 승격하지 않는다. 새 typeset finalize와 legacy pagination engine이 같은 collector를 소비하며, 공개 render tree의 실제 -1- 표시와 legacy 결과의 page_number_pos를 각각 검사한다.

tests/cases/page_number_pos_in_table_cell.rs의 공개 표시 검사와 legacy paginator 검사. 함수 존재만으로 두 소비 경로가 동일하다고 판정하지 않고 finalize 호출을 대조했다.

### 현재 소스와 검사

- [src/document_core/queries/rendering.rs](../../../src/document_core/queries/rendering.rs)
- [src/renderer/pagination.rs](../../../src/renderer/pagination.rs)
- [src/renderer/pagination/engine.rs](../../../src/renderer/pagination/engine.rs)
- [src/renderer/typeset/section/finalize.rs](../../../src/renderer/typeset/section/finalize.rs)
- [tests/cases/page_number_pos_in_table_cell.rs](../../../tests/cases/page_number_pos_in_table_cell.rs)

- `tests/cases/page_number_pos_in_table_cell.rs`: SHA-256 `54fbd165a9ab9ac43b914beb5675cd93f930eeba58ca8eee98b1fd759dbb2051`. 직접 선언 검사 2개: `page_number_pos_inside_table_cell_paints_page_number`, `legacy_paginator_reads_page_number_pos_inside_table_cell`.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

원 PR의 증적이나 helper assertion만으로 독립 Print 일치를 선언하지 않는다. 적용 입력·최종 head의 결과를 개별적으로 대조한 뒤 최종 판정을 갱신한다.

전체 로컬 Rust 검증은 완료했다. 현재 함수별 실제 PASS와 종료 근거는 아래 「통합 후보의 실제 검사 실행」에 기록했다. 독립 시각 근거와 원격 CI는 별도로 판정한다.

통합 후보의 현재 공통 시각 비교에서는 17조건·213쪽/backend의 후보 최저가 Native 92.04708%, fresh WASM 91.53306%였다. [전쪽 TSV](../assets/semanticist21_rebase_qualified_20261026/README.md)의 수치 대응 가능한 16조건·211쪽/backend에서 0.1%p 초과 하락 0쪽을 확인했고, #6133은 base 1쪽에서 독립 Print와 같은 2쪽으로 복원된 구조 개선으로 따로 기록했다. 이는 이 PR의 별도 입력이나 아래 필수 증거가 자동으로 검증됐다는 뜻이 아니다. #8228의 변경 기능 보존과 현재 보정 대상만 확인하며 다른 HWP로 범위를 넓히지 않는다.

| 조판 검토 항목 | 현재 근거와 판정 경계 |
| --- | --- |
| 규칙·공통 소비 | 위 실제 소비 경로와 반례를 따른다. helper 뒤 덮어쓰기·다른 frame을 실제 결과와 대조한다. |
| 입력·독립 기대값 | 입력 생성 방식과 한컴 Print 출처를 증거별로 구분한다. source와 무관한 값이나 원 작성자의 주장만으로 통과 처리하지 않는다. |
| 줄·문단·개체 소유 | 현재 검사 소스의 순서·누락·중복·포함·겹침 관계를 확인한다. |
| 시각·px 자격 | 전쪽 양쪽 raw 90% 이상·쪽수 일치·직접 판독이 필요하다. ±2px는 양쪽 98% 이상을 확인한 해당 페이지의 독립 좌표에만 허용한다. |
| 리베이스 기능 보존 | 새 base와 현재 후보의 동일 입력·producer·Print 조건을 대조한다. 평균값으로 페이지별 0.1%p 초과 하락을 상쇄하지 않는다. |
| 완료된 로컬 게이트 | 아래 실제 실행 결과와 공통 종료 기록을 대조했다. 이 PR의 독립 시각 증거와 원격 CI는 별도로 판정한다. |

sample16·편람의 기존 삭제를 유지하며 삭제 파일을 향하던 파생 참조도 재생성하지 않는다. 미해결 이관 문서를 완료 범위나 이슈 닫기 목록에 넣지 않는다.

## 이전 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/page_number_pos_in_table_cell.rs](../../../tests/cases/page_number_pos_in_table_cell.rs): SHA-256 `54fbd165a9ab9ac43b914beb5675cd93f930eeba58ca8eee98b1fd759dbb2051`.
  - `legacy_paginator_reads_page_number_pos_inside_table_cell`: PASS, `logs/current-full-nextest-4184.log:5405` (`rhwp::regression_suite_005`).
  - `page_number_pos_inside_table_cell_paints_page_number`: PASS, `logs/current-full-nextest-4184.log:5406` (`rhwp::regression_suite_005`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8031_review_impl.md)를 따른다.

## API 생성 Print 대조군 추가 — 2026-10-26

작업 시작 head는 `a72ce747362f537cc6dba4906dd38a8ea40daece`, 고정 base는
`49bc935a149c20b942d3045c2f5c733aeccf57d0`다. 엔진은 c2513250과 동일하며
이번 단계에서는 제품 코드를 수정하지 않았다. 기존 최소 IR 검사·입력의 미달을
완료된 다른 입력으로 대체하지 않는다. 아래 표는 새 대조군에 한정한다.

| 항목 | 직접 확인한 결과 |
| --- | --- |
| 독립 입력·Print | [입력·기준 PDF·생성 절차·SHA](../assets/pr_8031_page_number_cell/README.md) |
| Native/fresh 전쪽 | 각각 1/1쪽, raw 100.00000%, 미달·누락 0쪽 |
| 직접 판독 | 양쪽 review·standalone overlay에서 표 외곽·후속 BODY·꼬리말을 대조함 |
| 새 이름의 정식 검사 | `page_number_cell_print_contract.rs`: 2 PASS/0 FAIL, `regression_suite_007` |
| 검사 SHA-256 | `eac944341598401ea1640e235ac4f9053435ead0a845341f99e8565858f1affe` |
| 독립 기대값 | 셀 선언 소유·각 1회 내용 보존·순서 및 Print 꼬리말 기준선 ±2px |
| 기존 검사 유지 | `page_number_pos_in_table_cell.rs`는 HEAD의 바이트 그대로 유지함 |
| 판정 경계 | 기존 최소 IR 원본의 시각 미달과 통합 전체 잔여 게이트는 아직 해결로 세지 않음 |

자동 suite 최초 실행은 오래된 생성 목록으로 0건·exit4였고, manifest `--prepare` 후
새 함수 2건을 실제 실행해 exit0을 확인했다. base070에서도 쪽 번호가 보이므로
수정 전 FAIL 근거라고 기록하지 않는다. 이는 상속 기능 보존의 양성 대조다.
로그는 `output/pr-review/semanticist21-20261019/logs/page-number-cell-auto-nextest-4322.log`다.
기존 과거 전체 검증 수치는 아래/앞선 기록의 head에만 적용한다.
