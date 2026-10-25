# PR #8045 리뷰 — fix(layout): 칸 안 어울림 중첩 표 옆의 글도 저장 줄 시작 위치를 따른다

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8045](https://github.com/edwardkim/rhwp/pull/8045) / semanticist21 |
| 고정 원 head | `a3509563f11449954dfc64c946c93420fd30953e` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `a3509563f11449954dfc64c946c93420fd30953e` | `9a78717e02a04dbda6a83abfd79c9395501c032a` |

### 실제 소비 경로와 반례

어울림 표의 저장 줄 anchor를 nested_wrap_flow에서 공통으로 판정한다. 실제 배치의 nested_table_flow_advance와 측정의 unabsorbed_nested_table_paragraph_height가 같은 결과를 소비하며, 줄 높이 없는 합성 입력의 기존 fallback은 유지한다. 수동 fixture의 잘못된 column/vpos 핀을 제거하고 실제 한컴 저장본의 원본 HU 열 원점·부모 높이·내용 단일 소유와 독립 PDF 원점을 검사한다.



### 현재 소스와 검사

- [src/renderer/layout/table_layout.rs](../../../src/renderer/layout/table_layout.rs)
- [tests/cases/cell_square_nested_table_wrap.rs](../../../tests/cases/cell_square_nested_table_wrap.rs)

- `tests/cases/cell_square_nested_table_wrap.rs`: SHA-256 `6bdc987b7049512edaf81565daa5346a639d876f9a2e0e4c21e7337426277de8`. 직접 선언 검사 1개: `verified_square_saved_lane_and_parent_frame_share_source_ownership`.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

- [기존 시각 증거: inline_wrap_qualified_20261026](../assets/inline_wrap_qualified_20261026/README.md)
- [기존 시각 증거: source_pr8045/manual_cache_reference](../assets/source_pr8045/manual_cache_reference/README.md)

기존 재현의 한컴 Print와 Native255/fresh WASM073 전체 1쪽을 각각 확인했다. 위 증거의 각 입력은 양쪽 raw 100%이며 직접 review·overlay 판독을 마쳤다. 원본·PDF·쪽수·실제 producer·TSV·PNG의 해시는 증거 폴더에 있다. 100%를 RGB 완전 일치로 해석하지 않는다.

수동 작성 Square 원본의 잘못된 저장 LineSeg는 정상 저장본과 구분한다. 수동 원본의 현재 raw 93.69297%/93.39219% 및 base 대비 0.43577%p/0.45851%p 하락을 통과로 보고하지 않는다. 잘못된 수동 값을 정상 기대값으로 고정하던 검사를 실제 한컴 저장 계약으로 대체했으며, 저장본의 통과를 수동 원본의 일치로 바꾸어 주장하지 않는다.

전체 로컬 검증은 고정 target `target/pr-review`, 빌드 잡 8개·검사 스레드 12개로 순차 실행 중이다. fmt·suite 정책·unit tier 정책·Release build·Release 라이브러리 검사는 통과했다. 전체 nextest·Native Skia·lint 등의 후속 결과는 아직 완료되지 않았으므로 미리 PASS로 쓰지 않는다.

통합 후보의 현재 공통 시각 비교에서는 17조건·213쪽/backend의 후보 최저가 Native 92.04708%, fresh WASM 91.53306%였다. [전쪽 TSV](../assets/semanticist21_rebase_qualified_20261026/README.md)의 수치 대응 가능한 16조건·211쪽/backend에서 0.1%p 초과 하락 0쪽을 확인했고, #6133은 base 1쪽에서 독립 Print와 같은 2쪽으로 복원된 구조 개선으로 따로 기록했다. 이는 이 PR의 별도 입력이나 아래 필수 증거가 자동으로 검증됐다는 뜻이 아니다. #8228의 변경 기능 보존과 현재 보정 대상만 확인하며 다른 HWP로 범위를 넓히지 않는다.

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

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8045_review_impl.md)를 따른다.
