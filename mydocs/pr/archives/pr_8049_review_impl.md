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
