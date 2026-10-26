# PR #8094 리뷰 — fix(layout): 캡션 자동 번호 뒤 캐럿과 클릭 위치를 그린 글자에 맞춘다

## 최종 판정

**머지 보류 — 고정한 통합 코드 후보의 전체 로컬 검증과 이 PR의 필수 증거를 대조 중이다.**

최신 원격 CI·통합·후속 처리는 아직 완료되지 않았다. 원 contributor head의 직접 merge가 아니라 메인터너 보정한 통합 후보를 검토한다. 실제 완료 결과를 수집한 뒤 판정을 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8094](https://github.com/edwardkim/rhwp/pull/8094) / semanticist21 |
| 고정 원 head | `b9f84e4105b146902297a4e3c46fd82e0f2c6225` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 현재 통합 후보 | `d6ea10837218e7e490b166ed76af221490b61ba2` |
| 초기 렌더 소스·증거 등록(과거) | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

현재 대응표는 통합 head `fcc9c69c006c3cd134a0ed0dcd1f2822bc32aa2d`·기준 devel `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`의 실제 history를 대조했다. 원 head의 merge 전용 커밋과 이미 포함된 동등 구현은 기능 체리픽과 구분한다. 이 표는 커밋 출처이며 전체 검증 완료 판정은 아니다.

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `cf62911cb24009aca4b7c6e2857b1c0a2f4e8a21` | `73bdaaa52c3ad4771f73a6dd5277b90fbc9d3d23` |
| `b9f84e4105b146902297a4e3c46fd82e0f2c6225` | `9e2c52d5805ab731ca3d2c8bd4e2df6fb88dec4d` |

### 실제 소비 경로와 반례

자동 번호의 원본 자리표와 표시 문자열을 projection으로 구분하고 후속 문자의 source offset을 보존한다. caption_auto_number_caret는 표시한 캡션의 실제 글자 끝과 caret/hit 결과를 같은 tree에서 대조한다. parser의 raw 8유닛 슬롯, OLE caption·picture footnote의 owner 주소와 continuation을 함께 확인한다. cap 번호를 본문 문자열에 덧붙여 logical 길이를 바꾸는 방식으로 위치를 맞추지 않는다.



### 현재 소스와 검사

- [src/parser/body_text.rs](../../../src/parser/body_text.rs)
- [src/parser/body_text/tests.rs](../../../src/parser/body_text/tests.rs)
- [src/renderer/composer.rs](../../../src/renderer/composer.rs)
- [src/renderer/composer/auto_number_projection.rs](../../../src/renderer/composer/auto_number_projection.rs)
- [src/renderer/layout/paragraph_layout.rs](../../../src/renderer/layout/paragraph_layout.rs)
- [src/renderer/layout/picture_footnote.rs](../../../src/renderer/layout/picture_footnote.rs)
- [src/renderer/layout/table_partial.rs](../../../src/renderer/layout/table_partial.rs)
- [tests/cases/caption_auto_number_caret.rs](../../../tests/cases/caption_auto_number_caret.rs)
- [tests/issue_2069_ole_object_selection.rs](../../../tests/issue_2069_ole_object_selection.rs)

- `tests/cases/caption_auto_number_caret.rs`: SHA-256 `22bd86ccf753ae78aaaead24692c2e8de1bf15251bb83aa1baa7206c0cc9af1f`. 직접 선언 검사 2개: `caret_after_caption_number_sits_on_drawn_glyph`, `click_after_caption_number_lands_on_clicked_glyph`.
- `tests/issue_2069_ole_object_selection.rs`: SHA-256 `3fc7f6bdcb2ce8ccc2573a29aacb20a025dff37191e53e7cc107cc5f57e40c9d`. 직접 선언 검사 9개: `hwp_ole_preview_is_selectable_and_drives_empty_para_caret`, `hwpx_ole_preview_is_selectable_and_drives_empty_para_caret`, `hwp_enter_after_square_ole_respects_height_boundary`, `hwpx_enter_after_square_ole_respects_height_boundary`, `hwp_enter_backspace_reenter_keeps_ole_anchor_flow`, `hwp_square_ole_merge_undo_restores_removed_paragraph_meta`, `hwpx_enter_backspace_reenter_keeps_ole_anchor_flow`, `hwp_ole_caption_properties_roundtrip`, `hwpx_ole_caption_properties_roundtrip`.

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

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/caption_auto_number_caret.rs](../../../tests/cases/caption_auto_number_caret.rs): SHA-256 `22bd86ccf753ae78aaaead24692c2e8de1bf15251bb83aa1baa7206c0cc9af1f`.
  - `caret_after_caption_number_sits_on_drawn_glyph`: PASS, `logs/current-full-nextest-4184.log:5766` (`rhwp::regression_suite_007`).
  - `click_after_caption_number_lands_on_clicked_glyph`: PASS, `logs/current-full-nextest-4184.log:5767` (`rhwp::regression_suite_007`).
- [tests/issue_2069_ole_object_selection.rs](../../../tests/issue_2069_ole_object_selection.rs): SHA-256 `3fc7f6bdcb2ce8ccc2573a29aacb20a025dff37191e53e7cc107cc5f57e40c9d`.
  - `hwp_enter_after_square_ole_respects_height_boundary`: PASS, `logs/current-full-nextest-4184.log:10472` (`rhwp::regression_suite_024`).
  - `hwp_enter_backspace_reenter_keeps_ole_anchor_flow`: PASS, `logs/current-full-nextest-4184.log:10474` (`rhwp::regression_suite_024`).
  - `hwp_ole_caption_properties_roundtrip`: PASS, `logs/current-full-nextest-4184.log:10476` (`rhwp::regression_suite_024`).
  - `hwp_ole_preview_is_selectable_and_drives_empty_para_caret`: PASS, `logs/current-full-nextest-4184.log:10475` (`rhwp::regression_suite_024`).
  - `hwp_square_ole_merge_undo_restores_removed_paragraph_meta`: PASS, `logs/current-full-nextest-4184.log:10477` (`rhwp::regression_suite_024`).
  - `hwpx_enter_after_square_ole_respects_height_boundary`: PASS, `logs/current-full-nextest-4184.log:10478` (`rhwp::regression_suite_024`).
  - `hwpx_enter_backspace_reenter_keeps_ole_anchor_flow`: PASS, `logs/current-full-nextest-4184.log:10479` (`rhwp::regression_suite_024`).
  - `hwpx_ole_caption_properties_roundtrip`: PASS, `logs/current-full-nextest-4184.log:10480` (`rhwp::regression_suite_024`).
  - `hwpx_ole_preview_is_selectable_and_drives_empty_para_caret`: PASS, `logs/current-full-nextest-4184.log:10481` (`rhwp::regression_suite_024`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8094_review_impl.md)를 따른다.
