# PR #8057 리뷰 — fix(render): 가운뎃점으로 쓴 U+2024를 가운데 점으로 그린다

## 최종 판정

**메인터너 보정 후 수용 가능 — 점 기호의 실제 공급자·자연 전진·뒤 글자 보존 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 통합 PR의 정확한
code head CI·merge·후속 처리는 앞으로 수행할 조건이며 로컬 검증으로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

코드·회귀 후보는 `61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 이후 문서·증거는 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 현재 회귀와 입력 | 해당 원 PR의 기존 입력·독립 Print 3조건, 각 전체 1쪽. 새 입력·PDF·회귀 생성 없음 |
| Native/fresh WASM | 전쪽 최저 100.00000% / 100.00000%, 독립 PDF 쪽수 일치·90% 미만/누락/측정 불가 0쪽 |
| base 보존 | 같은 입력·Print·글꼴·backend의 전쪽 0.1%p 초과 하락 0쪽 |
| 실제 검사 | 아래 현재 모듈·함수의 실제 PASS 행과 source SHA를 확인했다. 파일 존재를 실행 결과로 대신하지 않았다. |
| 확대 판독·px 경계 | Gulim과 Arial에 없는 점의 실제 대체 공급자와 HCR Batang이 가진 원 프로그램을 구분했다. 두 경로의 점 높이·기준선·기호 모양과 뒤 글자의 자연 간격을 같은 좌표로 확대 판독했다. 원 PR의 글꼴 없는 IR에서 모든 U+2024를 벡터 원으로 바꾸던 가정을 수용하지 않았고, 실제 공급자·원문·소유 관계 및 독립 Print 좌표를 보호했다. |
| 전체 로컬 게이트 | 전체 nextest 11,708 PASS·0 FAIL, Skia 3종·fmt·정책·Native/WASM/workspace Clippy·workspace build·doctest의 실제 완료 근거를 재사용했다. 소스 불변을 확인했고 다시 빌드하지 않았다. |

[전쪽 Native/fresh 원문 TSV·base 비교·입력/Print·producer·글꼴과 PNG 해시](../assets/active_regression_qualified_20261026/README.md)를
연결한다. 정본 패널 12쌍과 같은 좌표의 최근접 확대 6장을 실제로 열었다. crop에서 점수를
다시 계산하거나 원본을 이동·정렬하지 않았다. 아래 현재 패널은 같은 source의 완료 캡처를
정본 함수로 합성한 재사용이며 새 재캡처로 표시하지 않는다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/active_regression_qualified_20261026/png/native/028/review_001.png) | [overlay](../assets/active_regression_qualified_20261026/png/native/028/overlay_001.png) |
| fresh WASM | [review](../assets/active_regression_qualified_20261026/png/fresh-wasm/028/review_001.png) | [overlay](../assets/active_regression_qualified_20261026/png/fresh-wasm/028/overlay_001.png) |

[같은 좌표 확대](../assets/active_regression_qualified_20261026/details/028-detail.png)도 함께
확인했다. Merge 뒤에는 위 실제 상대 경로를
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/active_regression_qualified_20261026/`
아래로 고정해 contributor comment에 review와 overlay를 같이 표시한다. 정확한 통합 head의
CI·merge·devel 포함과 asset 존재를 확인한 뒤 한국어 존댓말의 `--body-file`을 게시하고 API에서
실제 Markdown·한글·이미지 링크를 재조회한다. 원 contributor head를 직접 merge한 기록이 아니다.


## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8057](https://github.com/edwardkim/rhwp/pull/8057) / semanticist21 |
| 고정 원 head | `76271c14e4823a10a69747930f1c128f7bc7380a` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `76271c14e4823a10a69747930f1c128f7bc7380a` | `03e7749638e0af72a97710bb516b1e0707936b4b` |

### 실제 소비 경로와 반례

Unicode U+2024를 글꼴과 무관하게 벡터 원으로 바꾸는 원 검사는 독립 Print와 맞지 않아 수용하지 않았다. 현재 verified_point_glyph_supplier는 Gulim/Arial의 실제 공급자와 HCR Batang의 원 프로그램 우선을 분리하고 동일 glyph로 측정/paint한다. 세 입력의 전쪽 Native224/fresh063 raw 100%·직접 확대 판독과 독립 PDF 좌표가 선행 근거이며, 원문 기호·뒤 글자 순서 관계도 유지한다. 라이선스 추가 감사를 이 PR의 미해결 항목으로 넣지 않는다.



### 현재 소스와 검사

- [src/renderer/render_tree.rs](../../../src/renderer/render_tree.rs)
- [src/renderer/skia/text_replay.rs](../../../src/renderer/skia/text_replay.rs)
- [src/renderer/svg.rs](../../../src/renderer/svg.rs)
- [src/renderer/web_canvas.rs](../../../src/renderer/web_canvas.rs)
- [tests/cases/verified_point_glyph_supplier.rs](../../../tests/cases/verified_point_glyph_supplier.rs)

- `tests/cases/verified_point_glyph_supplier.rs`: SHA-256 `748415e9d21bd6b15518fef73c7cfb727d389bcfb66406b1e338413ffcb5da9b`. 직접 선언 검사 3개: `verified_arial_missing_points_keep_print_supplier_and_following_glyphs`, `verified_gulim_missing_points_keep_print_supplier_and_following_glyphs`, `verified_hcr_owned_points_keep_their_original_font_program`.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

위 현재 원본·독립 Print·Native/fresh 결과와 직접 판독으로 판정했다. 아래 과거 제품의 수치는 당시 결과로 구분한다.

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

- [tests/cases/verified_point_glyph_supplier.rs](../../../tests/cases/verified_point_glyph_supplier.rs): SHA-256 `748415e9d21bd6b15518fef73c7cfb727d389bcfb66406b1e338413ffcb5da9b`.
  - `verified_arial_missing_points_keep_print_supplier_and_following_glyphs`: PASS, `logs/final-full-nextest-4052.log:9567` (`rhwp::regression_suite_020`).
  - `verified_gulim_missing_points_keep_print_supplier_and_following_glyphs`: PASS, `logs/final-full-nextest-4052.log:9569` (`rhwp::regression_suite_020`).
  - `verified_hcr_owned_points_keep_their_original_font_program`: PASS, `logs/final-full-nextest-4052.log:9570` (`rhwp::regression_suite_020`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `76271c14e4823a10a69747930f1c128f7bc7380a` | `03e7749638e0af72a97710bb516b1e0707936b4b` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8057_review_impl.md)를 따른다.
