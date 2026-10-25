# PR #7819 리뷰 — Fix CharOverlap logical slots and inline note rendering

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#7819](https://github.com/edwardkim/rhwp/pull/7819) / semanticist21 |
| 고정 원 head | `36f9d3ac4091cc5c85cd74dee030c93b62c80964` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=true |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `7ded7ca1d468326844ede2e934b10c4add9c7d81` | `1269479a2a0f5dfac5a6ce9ca43346d987ea2f35` |
| `a3f80b14adcbec5974debb537a02eb84aa061603` | `c9334ddea87c9472c7b3a964e8cde6916d337fa0` |
| `36f9d3ac4091cc5c85cd74dee030c93b62c80964` | `aa90bdb2c5b1e34e7e341803cb66b9314c0e5c52` |

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

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_7819_review_impl.md)를 따른다.
