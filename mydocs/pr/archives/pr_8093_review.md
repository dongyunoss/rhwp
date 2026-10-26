# PR #8093 리뷰 — fix(edit): 글자처럼 취급한 양식 개체도 캐럿 한 칸을 차지한다

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8093](https://github.com/edwardkim/rhwp/pull/8093) / semanticist21 |
| 고정 원 head | `75c93fe0cc91c6ff74ebd85ac3c8f3d3b708a6d9` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `75c93fe0cc91c6ff74ebd85ac3c8f3d3b708a6d9` | `9ae5b6a118fded6f030f2dc6a09ce611d3c8f8b0` |

### 실제 소비 경로와 반례

Form의 treatAsChar를 논리 한 칸으로 투영하고, inline_control_projection·equation_line_metrics·inline_box_rows·nested_table_frames가 Form 메트릭을 소비한다. 같은 줄 본문과 개체의 높이·기준선 합집합을 사용하며 정상 저장 LineSeg를 임의로 덮지 않는다. 기존 화면 폭 90px 하한은 원본 Form 폭과 실제 caret 구간의 관계로 바꿨다. 입력·Enter 뒤 문단 소유도 유지하고 양쪽 100% 페이지의 독립 가/나 원점만 ±2px로 보호한다.



### 현재 소스와 검사

- [src/document_core/queries/doc_tree_nav.rs](../../../src/document_core/queries/doc_tree_nav.rs)
- [src/model/control.rs](../../../src/model/control.rs)
- [src/model/paragraph.rs](../../../src/model/paragraph.rs)
- [tests/cases/form_object_caret_slot.rs](../../../tests/cases/form_object_caret_slot.rs)

- `tests/cases/form_object_caret_slot.rs`: SHA-256 `ea341313d115cc45b2dad91cd5f49e91eacbc2d70b7658ce61866362985316cd`. 직접 선언 검사 4개: `verified_form_caret_order_and_source_width_ownership`, `verified_typing_before_form_preserves_text_and_control_order`, `verified_split_after_form_preserves_original_paragraph_ownership`, `verified_form_mixed_text_baseline_matches_independent_print`.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

- [기존 시각 증거: inline_wrap_qualified_20261026](../assets/inline_wrap_qualified_20261026/README.md)

기존 재현의 한컴 Print와 Native255/fresh WASM073 전체 1쪽을 각각 확인했다. 위 증거의 각 입력은 양쪽 raw 100%이며 직접 review·overlay 판독을 마쳤다. 원본·PDF·쪽수·실제 producer·TSV·PNG의 해시는 증거 폴더에 있다. 100%를 RGB 완전 일치로 해석하지 않는다.

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

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/form_object_caret_slot.rs](../../../tests/cases/form_object_caret_slot.rs): SHA-256 `ea341313d115cc45b2dad91cd5f49e91eacbc2d70b7658ce61866362985316cd`.
  - `verified_form_caret_order_and_source_width_ownership`: PASS, `logs/final-full-nextest-4052.log:4054` (`rhwp::regression_suite_001`).
  - `verified_typing_before_form_preserves_text_and_control_order`: PASS, `logs/final-full-nextest-4052.log:4058` (`rhwp::regression_suite_001`).
  - `verified_split_after_form_preserves_original_paragraph_ownership`: PASS, `logs/final-full-nextest-4052.log:4056` (`rhwp::regression_suite_001`).
  - `verified_form_mixed_text_baseline_matches_independent_print`: PASS, `logs/final-full-nextest-4052.log:4055` (`rhwp::regression_suite_001`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `75c93fe0cc91c6ff74ebd85ac3c8f3d3b708a6d9` | `9ae5b6a118fded6f030f2dc6a09ce611d3c8f8b0` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8093_review_impl.md)를 따른다.
