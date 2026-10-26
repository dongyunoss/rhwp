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

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/body_ime_format_batch.rs](../../../tests/cases/body_ime_format_batch.rs): SHA-256 `b4c005fc87e4e3518f82fe093e6b33153edff44921f5cc88b02c07cd29c5f6ea`.
  - `body_char_format_defers_pagination_until_end_batch`: PASS, `logs/final-full-nextest-4052.log:11204` (`rhwp::regression_suite_027`).
  - `local_body_replace_defers_pagination_until_end_batch`: PASS, `logs/final-full-nextest-4052.log:11210` (`rhwp::regression_suite_027`).
  - `explicit_composition_format_preserves_text_caret_and_roundtrip_after_batch`: PASS, `logs/final-full-nextest-4052.log:11205` (`rhwp::regression_suite_027`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `828a2a1e14c341dcd72ea41fe582486b66495939` | `dcf9b8b52d7ca01b412deb8b0515c4f39651a83e` |
| `59276bb29419c3a439866d132f5be69a6c759c7c` | 원 devel merge; 기능 체리픽에서 제외 |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
