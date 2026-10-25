# PR #8062 리뷰 — fix(typeset): 개체만 있는 줄과 겹치는 저장 줄은 저장 vpos 차이만큼 나아간다

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8062](https://github.com/edwardkim/rhwp/pull/8062) / semanticist21 |
| 고정 원 head | `e8b4e14d017d86c96be2c676b6d56bab9cfa657d` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `e8b4e14d017d86c96be2c676b6d56bab9cfa657d` | `e4766125e6e2f5230aaaa8542b2a5ed65a64fbcf` |

### 실제 소비 경로와 반례

저장된 개체 줄의 잉크 높이와 다음 줄 원점까지의 흐름 걸음은 다르다. overlapping_object_lines_advance_by_the_stored_vpos_step은 조판 부호 줄과 큰 TAC shape를 둔 뒤 lineHeightSum+lineSpacingSum이 원본 마지막 vpos+height의 범위를 이중 합산하지 않는지 검사한다. 원본 HWPUNIT에서 환산한 점유 계약을 절대 화면 좌표 핀으로 오인해 제거하지 않는다.



### 현재 소스와 검사

- [src/renderer/typeset/paragraph/format.rs](../../../src/renderer/typeset/paragraph/format.rs)
- [tests/cases/object_line_overlapping_stored_ladder.rs](../../../tests/cases/object_line_overlapping_stored_ladder.rs)

- `tests/cases/object_line_overlapping_stored_ladder.rs`: SHA-256 `28e9b9f1213b491da8cb382c91eccd63215489ad522d83519a97fb7217910cc8`. 직접 선언 검사 1개: `overlapping_object_lines_advance_by_the_stored_vpos_step`.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

원 PR의 증적이나 helper assertion만으로 독립 Print 일치를 선언하지 않는다. 적용 입력·최종 head의 결과를 개별적으로 대조한 뒤 최종 판정을 갱신한다.

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

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8062_review_impl.md)를 따른다.
