# PR #8049 구현·검증 기록 — 캡션 자동 번호의 표시 계약

## 범위와 현재 상태

[개별 리뷰](pr_8049_review.md)의 원 head·기능 커밋·로컬 체리픽을 따른다.
원본·기존 반례의 독립 Print와 Native082/fresh033 전쪽·직접 판독을 완료했고,
원 기대 문자열을 유지하며 테스트의 표시 문자열 소비를 보정했다. 통합 전체는 머지 보류다.

## 완료한 검증과 보정

1. 원 PR 본문·파일·커밋·reviewer와 실제 현재 head를 확인했다.
2. `control_source_positions → project_caption → layout_caption → display_or_text`의
   원문 주소·표시 문자열·실제 paint 소비를 대조했다. 원문 한 칸과 번호 문자열의 길이를 구분했다.
3. 원본과 기존 XML 반례를 같은 한컴2020 Print와 대조했다. 두 입력의 전체1쪽은
   Native/fresh 모두99.981%대이며, PDF 쪽수와 내용·번호·앞뒤 공백을 직접 확인했다.
4. 기존 `page_text` helper가 모델 자리표만 읽는 것을 확인했다. 실제 paint와 같은
   `display_or_text()`를 읽도록 바꿨다. 기대 문자열·좌표·공차·제품 구현은 바꾸지 않았다.
5. 같은 Native082에 같은 두 정식 함수를 연결해 검사 보정 전2FAIL·후2PASS를 확인했다.
   컴파일은 모두 성공했다. 원본·반례·PDF·Native/fresh TSV·대표 PNG와 출처를
   [최종 증거](../assets/caption_autonumber_projection_20261021/README.md)에 보존했다.

## 이어서 완료할 통합 절차

Rust 변경 범위의 최종 lint·전체 회귀와 다른 원 PR의 기능 보존 검증을 완료한다.
새 code head의 필수 항목이 끝나기 전에는 PR 생성·승인·통합을 진행하지 않는다.
원격 단계는 작업지시자의 승인 뒤 수행하며, 원 contributor history를 rewrite하지 않는다.
실제 통합 merge SHA 확인 뒤 원 PR별 comment·close와 문서·이슈 후속 처리를 실행한다.
로그·중간 JSON·다음 실행 인계는 ignored output에 보존한다.

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/caption_auto_number_position.rs](../../../tests/cases/caption_auto_number_position.rs): SHA-256 `e60e6d59a95326c2af875fe47ca4bb07528ea0896936c4b5ce67feab5350c736`.
  - `caption_number_replaces_its_placeholder`: PASS, `logs/current-full-nextest-4184.log:7443` (`rhwp::regression_suite_013`).
  - `caption_number_without_surrounding_spaces_is_drawn`: PASS, `logs/current-full-nextest-4184.log:7444` (`rhwp::regression_suite_013`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

## 최종 로컬 판정과 원격 잔여 — 2026-10-26

[개별 최종 근거](pr_8049_review.md#최종-로컬-판정-근거--2026-10-26)에 따라
캡션 자동 번호의 표시 문자열·앞뒤 간격 범위를 `메인터너 보정 후 수용 가능`으로 판정했다.
검사·출력 source는 위와 같고 마지막 fetch에서 devel이 전진하지 않았음을 확인했다.
이전 단계의 미완료 문장은 당시 상태이며 현재 로컬 게이트의 미실행을 뜻하지 않는다.
통합 PR의 실제 code CI 성공 뒤 옵션 2의 self-review·오늘할일 후속 PR을 처리한다.
정확한 통합 head의 CI·merge·devel 포함 확인 전에는 원 PR을 close하지 않는다.
이 판정을 다른 원 PR의 미완료 항목이나 sample16·편람의 검증으로 확대하지 않는다.
