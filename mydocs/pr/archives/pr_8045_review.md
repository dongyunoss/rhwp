# PR #8045 리뷰 — fix(layout): 칸 안 어울림 중첩 표 옆의 글도 저장 줄 시작 위치를 따른다

## 최종 판정

**메인터너 보정 후 수용 가능 — 정상 한컴 저장본의 중첩 표·옆 본문 소유 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 아래 현재 증거와
실제 검사 실행을 대조했다. 통합 PR의 정확한 code head CI·merge·후속 처리는
앞으로 수행할 조건이며 로컬 통과로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

원 head `a3509563f11449954dfc64c946c93420fd30953e`가 원격과 같음을 재확인했다. 검증 코드는
`61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 제출 전 fetch에서도 같은 base를
확인했으므로 추가 리베이스하지 않았다. 이후 문서·증거 커밋은 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 원 PR의 현재 검사 | 아래 모듈·함수별 실제 PASS와 로그 행을 확인했다. 소스 존재를 실행 결과로 대신하지 않았다. |
| 전쪽 Native/fresh WASM | 기존 원본 1조건·전체 1쪽/backend에서 최저 100.00000% / 100.00000%, 독립 Print 쪽수 일치·90% 미만/누락 0쪽 |
| 고정 devel 보존 | 동일 입력·Print·글꼴·backend의 페이지별 비교에서 0.1%p 초과 하락 0쪽 |
| 직접 판독 | 정상 한컴 저장본의 column_start=20100·vertical_pos=1600과 메모·부모 표·뒤 본문의 관계를 확인했다. 잘못된 수동 줄 캐시 원본의 위치 차이와 0.1%p 초과 하락은 비활성 음성 자료에 그대로 공개하며 통과로 바꾸지 않는다. 이 판정은 그 수동 원본의 출력 개선 완료를 뜻하지 않는다. |
| 전체 로컬 게이트 | fmt·수집/단위 정책·Release build/lib·전체 nextest·Skia 3종·Native/WASM/workspace Clippy·workspace build·doctest 완료. 실제 종료 근거는 아래 공통 기록 참조 |
| 원격 절차 | 통합 PR의 정확한 code head CI·실제 merge·후속 comment는 앞으로 수행할 조건이다. 원 contributor head를 직접 승인·merge한 기록이 아니다. |

기존 원본의 전체 페이지를 대조했다. [전쪽 TSV·원본/Print·producer·해시와 판독](../assets/inline_wrap_qualified_20261026/README.md)을
이 PR의 직접 근거로 연결한다. raw는 색상 경계 보정 전 2px 실루엣이며 RGB 완전 일치나
과거 별도 지표의 통과를 뜻하지 않는다. Native255·표준 release/wasm-opt를 마친 fresh073을
사용했다. 측정 뒤 코드 변경은 없으며, 과거 제품의 표는 당시 결과로 구분한다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/inline_wrap_qualified_20261026/native/pr8045-hancom-saved-square/review_001.png) | [overlay](../assets/inline_wrap_qualified_20261026/native/pr8045-hancom-saved-square/overlay_001.png) |
| fresh WASM | [review](../assets/inline_wrap_qualified_20261026/fresh-wasm/pr8045-hancom-saved-square/review_001.png) | [overlay](../assets/inline_wrap_qualified_20261026/fresh-wasm/pr8045-hancom-saved-square/overlay_001.png) |

Merge 후에는 이 대표 파일을
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/inline_wrap_qualified_20261026/`
위 표의 실제 상대 경로로 고정해 contributor comment에 두 이미지를 함께 표시한다.
같은 merge의 devel 포함·asset 존재·CI 결과를 먼저 확인하고 한국어 존댓말의
`--body-file` 본문을 게시한 뒤 API에서 한글·링크·실제 Markdown을 다시 확인한다.


## 접수 정보

이하 최초 접수와 중간 검토의 제품·기준 SHA 및 당시 진행 상태를 보존한다.
현재 판정은 위 「최종 로컬 판정 근거」와 아래 「통합 후보의 실제 검사 실행」이 정본이다.
과거의 진행 중·미완료 표현을 현재 로컬 게이트 상태로 사용하지 않는다.

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8045](https://github.com/edwardkim/rhwp/pull/8045) / semanticist21 |
| 고정 원 head | `a3509563f11449954dfc64c946c93420fd30953e` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 현재 통합 후보 | `d6ea10837218e7e490b166ed76af221490b61ba2` |
| 초기 렌더 소스·증거 등록(과거) | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `a3509563f11449954dfc64c946c93420fd30953e` | `c44903e1eef4ff1226997122998a1916ce3b77d5` |

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

- [tests/cases/cell_square_nested_table_wrap.rs](../../../tests/cases/cell_square_nested_table_wrap.rs): SHA-256 `6bdc987b7049512edaf81565daa5346a639d876f9a2e0e4c21e7337426277de8`.
  - `verified_square_saved_lane_and_parent_frame_share_source_ownership`: PASS, `logs/current-full-nextest-4184.log:11498` (`rhwp::regression_suite_028`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8045_review_impl.md)를 따른다.
