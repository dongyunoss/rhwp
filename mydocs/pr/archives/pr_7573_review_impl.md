# PR #7573 구현·검증 순서 — 경계 개체의 복사·삭제 소유 일치

[개별 검토](pr_7573_review.md)의 원 head·체리픽과 공통 보정을 기준으로 한다.

1. 원 Draft의 정상 대조군1개와 양 끝 실패2개를 읽고 복사·삭제의 글자 축과 문단 끝
   후행 개체 계약을 추적했다. #7509의 입력/복사 범위와 같은 문제로 묶어 완료하지 않는다.
2. `0eb36de6d5`에서 본문·셀 경계가 같은 `delete_paragraph_boundary`를 소비하게 했다.
   원본 control 번호 변경은 모델 `remove_control`에 맡겼다.
3. Rust1180의 관련3검사 PASS는 확인했으나 수량 유지가 내용과 서식 전체를 입증하지는
   않는다. 수정 전후 경계 실행, 원문·소유 보존 반례와 최신 제품 집중 검사를 완료한다.
4. API 생성 전/후4조건을 최신 Native/fresh와 독립 Print PDF로 재검증하고 직접 판독한다.
   옛019의 양쪽 raw100%와 최신 후보를 구분하고 고정 devel 대비 하락도 확인한다.
5. 전체 필수 로컬 검증과 원본·PDF·전쪽 TSV·대표 PNG 보존을 마친 뒤 최종 판정을 갱신한다.
   검증을 우회해 Draft PR을 새로 만들거나 관련 이슈 전체를 닫지 않는다.
6. 사용자 승인 후 통합 PR·CI·merge를 진행하며, 원 PR별 comment/close·관련 이슈·문서
   후속 처리에는 실제 merge SHA를 사용한다. 현재 remote 조치는 수행하지 않았다.

실행 로그·중간 JSON·다음 실행 메모는 ignored output에만 보존한다. 기존 기대값 완화와
회귀 제거로 개체 보존 결함을 숨기지 않는다.

현재 제품43ab의 전체 Rust1352에서 관련3검사 PASS를 재확인했다. Native088·fresh037의
최신1347 실행에서도 편집 저장본4조건의 전쪽 raw100%·Print 쪽수 일치를 확인했다.
통합 전체는59개 Rust 실패와 다른 시각 미달이 남아 있으며, 이 PR의 직접 판독·최신 devel
대조·제출 asset 보존도 이어서 완료한다. 실제 실행 로그는 ignored output의
`logs/batch-cut-native088-1347.log`와 `logs/wasm037-focus-queue-1351.log`에 연결된다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/issue_7444_cut_boundary_object.rs](../../../tests/cases/issue_7444_cut_boundary_object.rs): SHA-256 `41337da29e671877401935322a39efff28c0a1e00b7440a2ecf66c1d28973040`.
  - `cut_with_table_in_selection_interior_moves_one_table`: PASS, `logs/final-full-nextest-4052.log:10791` (`rhwp::regression_suite_025`).
  - `cut_ending_at_table_paragraph_start_does_not_duplicate_table`: PASS, `logs/final-full-nextest-4052.log:10788` (`rhwp::regression_suite_025`).
  - `cut_starting_at_table_paragraph_start_does_not_duplicate_table`: PASS, `logs/final-full-nextest-4052.log:10790` (`rhwp::regression_suite_025`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `006884716b10166dbc1f31b4541b0277f520cbd5` | `415772049558c46e1600e7daa1dfb93217726383` |
| `84e80b5d82455254bdbdaca0a2b19dff92f1be12` | 원 devel merge; 기능 체리픽에서 제외 |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
