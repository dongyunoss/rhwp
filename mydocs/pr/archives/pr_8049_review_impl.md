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

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/caption_auto_number_position.rs](../../../tests/cases/caption_auto_number_position.rs): SHA-256 `e60e6d59a95326c2af875fe47ca4bb07528ea0896936c4b5ce67feab5350c736`.
  - `caption_number_replaces_its_placeholder`: PASS, `logs/final-full-nextest-4052.log:10389` (`rhwp::regression_suite_024`).
  - `caption_number_without_surrounding_spaces_is_drawn`: PASS, `logs/final-full-nextest-4052.log:10390` (`rhwp::regression_suite_024`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `285f86c92c27eda03616491a1d1f0c8210c9b80a` | `0aced7f0e0adb11e6163a1036a98b60bc6080b5e` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
