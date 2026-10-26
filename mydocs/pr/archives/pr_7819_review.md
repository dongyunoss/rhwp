# PR #7819 리뷰 — Fix CharOverlap logical slots and inline note rendering

## 최종 판정

**메인터너 보정 후 수용 가능 — 글자겹침·주석의 논리 슬롯과 캐럿 소유 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 통합 PR의 정확한
code head CI·merge·후속 처리는 앞으로 수행할 조건이며 로컬 검증으로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

코드·회귀 후보는 `61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 이후 문서·증거는 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 현재 회귀와 입력 | 해당 원 PR의 기존 입력·독립 Print 27조건, 각 전체 1쪽. 새 입력·PDF·회귀 생성 없음 |
| Native/fresh WASM | 전쪽 최저 95.73216% / 95.27616%, 독립 PDF 쪽수 일치·90% 미만/누락/측정 불가 0쪽 |
| base 보존 | 같은 입력·Print·글꼴·backend의 전쪽 0.1%p 초과 하락 0쪽 |
| 실제 검사 | 아래 현재 모듈·함수의 실제 PASS 행과 source SHA를 확인했다. 파일 존재를 실행 결과로 대신하지 않았다. |
| 확대 판독·px 경계 | 22개 캐럿 원본과 5개 크기 대조군을 확인했다. non-BMP 문자·같은 앵커·문단 끝·각주/미주와 앞뒤 원문·주석 선을 같은 좌표로 판독했다. 현재 ±2px 검사에서 읽는 nonbmp 네 파일은 양쪽 raw 100%다. HWPX의 큰 내부 글자는 그 HWPX Print에도 있으므로 HWP와 화면이 같다는 주장으로 바꾸지 않는다. |
| 전체 로컬 게이트 | 전체 nextest 11,708 PASS·0 FAIL, Skia 3종·fmt·정책·Native/WASM/workspace Clippy·workspace build·doctest의 실제 완료 근거를 재사용했다. 소스 불변을 확인했고 다시 빌드하지 않았다. |

[전쪽 Native/fresh 원문 TSV·base 비교·입력/Print·producer·글꼴과 PNG 해시](../assets/active_regression_qualified_20261026/README.md)를
연결한다. 정본 패널 12쌍과 같은 좌표의 최근접 확대 6장을 실제로 열었다. crop에서 점수를
다시 계산하거나 원본을 이동·정렬하지 않았다. 아래 현재 패널은 같은 source의 완료 캡처를
정본 함수로 합성한 재사용이며 새 재캡처로 표시하지 않는다.

크기 50 대조군의 겹친 glyph 윤곽·상대 배치 잔차는 base와 같이 남는다. 이 검토는
원 PR의 논리 슬롯·주석·캐럿과 기존 크기 계약의 유지이며 그 자형 잔차를 고쳤다고
주장하지 않는다. 이 쪽의 95%대 점수로 px 좌표 검사를 정당화하지 않고, 실제 새 px
보호는 양쪽 100%인 네 non-BMP 파일에 한정했다. 기호·각주 선과 작은 글자 가장자리의
래스터 농도 차이도 RGB 완전 일치로 설명하지 않는다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/active_regression_qualified_20261026/png/native/001/review_001.png) | [overlay](../assets/active_regression_qualified_20261026/png/native/001/overlay_001.png) |
| fresh WASM | [review](../assets/active_regression_qualified_20261026/png/fresh-wasm/001/review_001.png) | [overlay](../assets/active_regression_qualified_20261026/png/fresh-wasm/001/overlay_001.png) |

[같은 좌표 확대](../assets/active_regression_qualified_20261026/details/001-detail.png)도 함께
확인했다. Merge 뒤에는 위 실제 상대 경로를
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/active_regression_qualified_20261026/`
아래로 고정해 contributor comment에 review와 overlay를 같이 표시한다. 정확한 통합 head의
CI·merge·devel 포함과 asset 존재를 확인한 뒤 한국어 존댓말의 `--body-file`을 게시하고 API에서
실제 Markdown·한글·이미지 링크를 재조회한다. 원 contributor head를 직접 merge한 기록이 아니다.


## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#7819](https://github.com/edwardkim/rhwp/pull/7819) / semanticist21 |
| 고정 원 head | `36f9d3ac4091cc5c85cd74dee030c93b62c80964` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=true |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 현재 통합 후보 | `d6ea10837218e7e490b166ed76af221490b61ba2` |
| 초기 렌더 소스·증거 등록(과거) | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `7ded7ca1d468326844ede2e934b10c4add9c7d81` | `2e942b530b412f404c76595ef57b004781250214` |
| `a3f80b14adcbec5974debb537a02eb84aa061603` | `029e133e428041af1ec16002d4b586b326e61cc3` |
| `36f9d3ac4091cc5c85cd74dee030c93b62c80964` | `4a1d15b0e78b5c6963d8c03b3ef252360f06383a` |

### 실제 소비 경로와 반례

CharOverlap과 note의 logical 한 칸을 원문 scalar 길이와 구분한다. model::Control의 논리 슬롯 분류부터 공개 getter·caret·selection·copy/split/merge 소비까지 같은 축을 확인한다. empty overlap·동일 앵커·non-BMP·말미 hit·실제 note 순서가 반례다. 정상 source 문자와 합성 overlap payload를 혼동하지 않으며, px 검사는 issue_7819_caret_axis에 독립 Print와 양쪽 98% 이상을 기록한 해당 페이지로 한정한다.



### 현재 소스와 검사

- [src/document_core/helpers.rs](../../../src/document_core/helpers.rs)
- [src/document_core/queries/cursor_nav.rs](../../../src/document_core/queries/cursor_nav.rs)
- [src/model/control.rs](../../../src/model/control.rs)
- [src/model/paragraph.rs](../../../src/model/paragraph.rs)
- [src/renderer/layout/paragraph_layout.rs](../../../src/renderer/layout/paragraph_layout.rs)
- [tests/cases/char_overlap_caret_axis.rs](../../../tests/cases/char_overlap_caret_axis.rs)
- [tests/cases/issue_7408_stored_row_key_folded_origin.rs](../../../tests/cases/issue_7408_stored_row_key_folded_origin.rs)

- `tests/cases/char_overlap_caret_axis.rs`: SHA-256 `9a680fef8d58f03c6492f58e25f9f57c0195006f1f49b2265b3181c7fdb53c89`. 직접 선언 검사 10개: `mixed_overlap_note_uses_one_axis_in_public_getters_and_both_formats`, `public_caret_and_selection_use_the_rendered_overlap_and_note_boundaries`, `input_before_and_after_overlap_and_note_preserves_source_slots`, `logical_copy_excludes_or_includes_the_overlap_slot_without_neighbor_text`, `split_merge_and_snapshot_restore_keep_the_logical_seam_after_overlap`, `empty_overlap_and_same_anchor_slots_are_counted_without_claiming_middle_input`, `floating_and_auto_number_keep_their_existing_caret_contract`, `non_bmp_text_and_note_keep_source_order_complete_caret_and_hit_boundaries`, `same_anchor_notes_keep_control_order_and_terminal_overlap_geometry`, `issue_7819_print_qualified_overlap_note_caret_boundaries_stay_within_two_pixels`.
- `tests/cases/issue_7408_stored_row_key_folded_origin.rs`: SHA-256 `5916b148a6b41d49de4b4d14ba626a6c079cb274b88afec4cba224c442963f65`. 직접 선언 검사 4개: `footnote_sample_keeps_every_stored_cut`, `endnote_sample_keeps_every_stored_cut`, `tac_picture_sample_keeps_its_stored_cuts`, `edited_paragraph_is_rewrapped`.

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

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/char_overlap_caret_axis.rs](../../../tests/cases/char_overlap_caret_axis.rs): SHA-256 `9a680fef8d58f03c6492f58e25f9f57c0195006f1f49b2265b3181c7fdb53c89`.
  - `empty_overlap_and_same_anchor_slots_are_counted_without_claiming_middle_input`: PASS, `logs/current-full-nextest-4184.log:10135` (`rhwp::regression_suite_023`).
  - `floating_and_auto_number_keep_their_existing_caret_contract`: PASS, `logs/current-full-nextest-4184.log:10134` (`rhwp::regression_suite_023`).
  - `input_before_and_after_overlap_and_note_preserves_source_slots`: PASS, `logs/current-full-nextest-4184.log:10143` (`rhwp::regression_suite_023`).
  - `issue_7819_print_qualified_overlap_note_caret_boundaries_stay_within_two_pixels`: PASS, `logs/current-full-nextest-4184.log:10137` (`rhwp::regression_suite_023`).
  - `logical_copy_excludes_or_includes_the_overlap_slot_without_neighbor_text`: PASS, `logs/current-full-nextest-4184.log:10139` (`rhwp::regression_suite_023`).
  - `mixed_overlap_note_uses_one_axis_in_public_getters_and_both_formats`: PASS, `logs/current-full-nextest-4184.log:10138` (`rhwp::regression_suite_023`).
  - `non_bmp_text_and_note_keep_source_order_complete_caret_and_hit_boundaries`: PASS, `logs/current-full-nextest-4184.log:10140` (`rhwp::regression_suite_023`).
  - `public_caret_and_selection_use_the_rendered_overlap_and_note_boundaries`: PASS, `logs/current-full-nextest-4184.log:10141` (`rhwp::regression_suite_023`).
  - `same_anchor_notes_keep_control_order_and_terminal_overlap_geometry`: PASS, `logs/current-full-nextest-4184.log:10144` (`rhwp::regression_suite_023`).
  - `split_merge_and_snapshot_restore_keep_the_logical_seam_after_overlap`: PASS, `logs/current-full-nextest-4184.log:10142` (`rhwp::regression_suite_023`).
- [tests/cases/issue_7408_stored_row_key_folded_origin.rs](../../../tests/cases/issue_7408_stored_row_key_folded_origin.rs): SHA-256 `5916b148a6b41d49de4b4d14ba626a6c079cb274b88afec4cba224c442963f65`.
  - `edited_paragraph_is_rewrapped`: PASS, `logs/current-full-nextest-4184.log:4437` (`rhwp::regression_suite_002`).
  - `endnote_sample_keeps_every_stored_cut`: PASS, `logs/current-full-nextest-4184.log:4438` (`rhwp::regression_suite_002`).
  - `footnote_sample_keeps_every_stored_cut`: PASS, `logs/current-full-nextest-4184.log:4439` (`rhwp::regression_suite_002`).
  - `tac_picture_sample_keeps_its_stored_cuts`: PASS, `logs/current-full-nextest-4184.log:4449` (`rhwp::regression_suite_002`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_7819_review_impl.md)를 따른다.
