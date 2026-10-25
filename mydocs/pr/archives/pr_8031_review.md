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
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `3b79ee07292b6d386af1ee9e909e0a5549ec52f8` | `7f9763ba3001bd90bf1c7a5d4e9e807a93a61ae7` |

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

전체 로컬 검증은 고정 target `target/pr-review`, 빌드 잡 8개·검사 스레드 12개로 순차 실행 중이다. fmt·suite 정책·unit tier 정책·Release build·Release 라이브러리 검사는 통과했다. 전체 nextest·Native Skia·lint 등의 후속 결과는 아직 완료되지 않았으므로 미리 PASS로 쓰지 않는다.

통합 후보의 현재 공통 시각 비교에서는 고정 base 대비 17조건·213쪽/backend에 대해 Native 최저 92.04708%, fresh WASM 최저 91.53306%, 0.1%p 초과 하락 0쪽을 확인했다. 이는 이 PR의 별도 입력이나 아래 필수 증거가 자동으로 검증됐다는 뜻이 아니다. #8228의 변경 기능 보존과 현재 보정 대상만 확인하며 다른 HWP로 범위를 넓히지 않는다.

| 조판 검토 항목 | 현재 근거와 판정 경계 |
| --- | --- |
| 규칙·공통 소비 | 위 실제 소비 경로와 반례를 따른다. helper 뒤 덮어쓰기·다른 frame을 실제 결과와 대조한다. |
| 입력·독립 기대값 | 입력 생성 방식과 한컴 Print 출처를 증거별로 구분한다. source와 무관한 값이나 원 작성자의 주장만으로 통과 처리하지 않는다. |
| 줄·문단·개체 소유 | 현재 검사 소스의 순서·누락·중복·포함·겹침 관계를 확인한다. |
| 시각·px 자격 | 전쪽 양쪽 raw 90% 이상·쪽수 일치·직접 판독이 필요하다. ±2px는 양쪽 98% 이상을 확인한 해당 페이지의 독립 좌표에만 허용한다. |
| 리베이스 기능 보존 | 새 base와 현재 후보의 동일 입력·producer·Print 조건을 대조한다. 평균값으로 페이지별 0.1%p 초과 하락을 상쇄하지 않는다. |
| 미완료 로컬 게이트 | 전체 Rust 검증 종료 결과 및 이 PR에 적용되는 독립 증거를 연결한 뒤 최종 판정을 변경한다. |

sample16·편람의 기존 삭제를 유지하며 삭제 파일을 향하던 파생 참조도 재생성하지 않는다. 미해결 이관 문서를 완료 범위나 이슈 닫기 목록에 넣지 않는다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8031_review_impl.md)를 따른다.
