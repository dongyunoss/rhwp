# PR #7572 구현·검증 순서 — 명시적 배치의 페이지 확정 소유

[개별 검토](pr_7572_review.md)의 원 head·적용 SHA와 메인터너 보정을 기준으로 한다.
이미 적용한45개 PR 범위만 검토하며 새 PR은 체리픽하지 않는다.

1. 원 Draft의 본문·검사 두 개와 실제 직접 `paginate()` 호출을 대조했다. 기존 서식 상속과
   배치 중 페이지 확정을 서로 다른 계약으로 유지했다.
2. 공통 본문 서식 helper와 local replace가 배치 중 문단 조합·dirty 상태를 갱신하도록
   `0eb36de6d5`에서 보정했다. 비배치 경로의 즉시 쪽 나눔은 유지한다.
3. 종료 직후 캐럿·페이지 정보, 앞뒤 문단·서식, HWP/HWPX 왕복과 snapshot을 추가 검사했다.
   Rust1180에서 관련3검사 PASS를 확인했으며 전체 실행의 다른75실패를 숨기지 않는다.
4. 수정 전 결함 검출과 최신 제품 집중 실행, 동일 API 저장본3조건의 최신 Native/fresh
   Print 대조·직접 판독·devel 무회귀를 완료한다. 이전 source의 raw100%를 최신 결과로 쓰지 않는다.
5. 통합 후보의 필수 전체 로컬 검증을 마친 뒤 개별 최종 판정·입력/TSV/대표 PNG를 고정한다.
   검토 기록만을 위한 통합 PR 번호의 별도 PR은 만들지 않는다.
6. 사용자 승인 후 remote push·PR을 수행한다. 이후 CI·merge·원 PR comment/close·이슈와
   문서 정리는 실제 검증 head와 merge SHA에 따라 처리한다.

로그·중간 JSON·다음 실행 메모는 ignored output에만 남긴다. 테스트 기대값을 완화해 완료하지 않는다.

현재 제품43ab의 전체 Rust1352에서 관련3검사 PASS를 재확인했다. Native088·fresh037의
최신1347 실행에서도 편집 저장본3조건의 전쪽 raw100%·Print 쪽수 일치를 확인했다.
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

- [tests/cases/body_ime_format_batch.rs](../../../tests/cases/body_ime_format_batch.rs): SHA-256 `b4c005fc87e4e3518f82fe093e6b33153edff44921f5cc88b02c07cd29c5f6ea`.
  - `body_char_format_defers_pagination_until_end_batch`: PASS, `logs/current-full-nextest-4184.log:7440` (`rhwp::regression_suite_013`).
  - `explicit_composition_format_preserves_text_caret_and_roundtrip_after_batch`: PASS, `logs/current-full-nextest-4184.log:7441` (`rhwp::regression_suite_013`).
  - `local_body_replace_defers_pagination_until_end_batch`: PASS, `logs/current-full-nextest-4184.log:7442` (`rhwp::regression_suite_013`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
