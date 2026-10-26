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

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/issue_7444_cut_boundary_object.rs](../../../tests/cases/issue_7444_cut_boundary_object.rs): SHA-256 `41337da29e671877401935322a39efff28c0a1e00b7440a2ecf66c1d28973040`.
  - `cut_ending_at_table_paragraph_start_does_not_duplicate_table`: PASS, `logs/current-full-nextest-4184.log:6669` (`rhwp::regression_suite_010`).
  - `cut_starting_at_table_paragraph_start_does_not_duplicate_table`: PASS, `logs/current-full-nextest-4184.log:6670` (`rhwp::regression_suite_010`).
  - `cut_with_table_in_selection_interior_moves_one_table`: PASS, `logs/current-full-nextest-4184.log:6671` (`rhwp::regression_suite_010`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
