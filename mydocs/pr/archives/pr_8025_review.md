# PR #8025 리뷰 — fix(canvas): 굵은 표시용 글꼴을 SVG처럼 굵게 그린다

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8025](https://github.com/edwardkim/rhwp/pull/8025) / semanticist21 |
| 고정 원 head | `6ddffd1aeb4f538f5576f33a1f882bd13c0e40db` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `6ddffd1aeb4f538f5576f33a1f882bd13c0e40db` | `119e5750ea7362507008e5e7a5636436e9b5186d` |

### 실제 소비 경로와 반례

CanvasTextFont::for_positioned_text는 style.bold만 보지 않고 is_visually_bold를 사용한다. 실제 faux-bold 획을 이미 적용하는 face는 굵기 descriptor를 중복 적용하지 않는 조건도 유지한다. HY헤드라인M·HY견고딕·한양견고딕·HYGothic-Extra와 일반 함초롬바탕의 적용·비적용을 같은 descriptor 경로에서 검사한다.

tests/cases/canvas_heavy_display_face_bold.rs. Canvas 실제 출력 증거는 해당 글꼴·producer의 Print 대조와 별도로 연결해야 하며 descriptor 검사만으로 수용하지 않는다.

### 현재 소스와 검사

- [src/renderer/canvas_text_font.rs](../../../src/renderer/canvas_text_font.rs)
- [src/renderer/style_resolver.rs](../../../src/renderer/style_resolver.rs)
- [src/renderer/web_canvas.rs](../../../src/renderer/web_canvas.rs)
- [tests/cases/canvas_heavy_display_face_bold.rs](../../../tests/cases/canvas_heavy_display_face_bold.rs)

- `tests/cases/canvas_heavy_display_face_bold.rs`: SHA-256 `ebd47aa003636d0bfc5924565cc683bfb1a6f402d4c3705a2b69fd188e30f31c`. 직접 선언 검사 1개: `heavy_display_faces_paint_bold_on_canvas`.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

원 PR의 증적이나 helper assertion만으로 독립 Print 일치를 선언하지 않는다. 적용 입력·최종 head의 결과를 개별적으로 대조한 뒤 최종 판정을 갱신한다.

기존 월간 수출입 원본의 [실제 Canvas·독립 Print 증거](../assets/intrinsic_heavy_face_20261026/README.md)를 연결했다. 원본 셀의 HY헤드라인M·bold=false 제목 10개와 실제 적재 프로그램을 확인했고, 전 19쪽의 Native255/fresh WASM073 최저는 96.62490%/96.89977%였다. 고정 base 대비 0.1%p 초과 하락은 양쪽 0쪽이다. 4·8·10쪽의 제목 굵기·기준선·간격과 표·차트 경계를 양쪽 review·overlay로 직접 판독했다. descriptor 검사만으로 승인하지 않았으며, 남은 ○ glyph 차이는 같은 증거에서 base 보존·개선과 구분했다. 새로운 HWP 문서는 추가하지 않았다.

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

- [tests/cases/canvas_heavy_display_face_bold.rs](../../../tests/cases/canvas_heavy_display_face_bold.rs): SHA-256 `ebd47aa003636d0bfc5924565cc683bfb1a6f402d4c3705a2b69fd188e30f31c`.
  - `heavy_display_faces_paint_bold_on_canvas`: PASS, `logs/final-full-nextest-4052.log:9090` (`rhwp::regression_suite_019`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `6ddffd1aeb4f538f5576f33a1f882bd13c0e40db` | `119e5750ea7362507008e5e7a5636436e9b5186d` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8025_review_impl.md)를 따른다.
