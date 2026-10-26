# PR #8049 리뷰 — 캡션 자동 번호의 자리표와 표시 문자열

## 최종 판정

**머지 보류 — 원본·반례의 Native/fresh Print 대조와 검사 소비값 보정을 완료했으며, 통합 후보의 전체 필수 검증을 계속한다.**

현재 기록은 로컬 체리픽 통합 검토다. 원격 push·PR 생성·review/comment·merge는
작업지시자의 구체적인 승인 뒤에 수행한다.

## 접수 정보

| 항목 | 확인값 |
| --- | --- |
| 원 PR·작성자 | [#8049](https://github.com/edwardkim/rhwp/pull/8049) / semanticist21 |
| 원 head·기능 커밋 | `285f86c92c27eda03616491a1d1f0c8210c9b80a` |
| 로컬 체리픽 | `0e433877e62ea089adb2595a272f9d42bdbee04b` |
| 통합 브랜치·검증 제품 | `review/semanticist21-20261019` / `e787be830763d871703e025aa7adae499f954618` |
| 고정 devel | `2e3b4f880bd11d35398e6ea0100846611f13a72d` |
| reviewer·현재 상태 | jangster77 요청 확인; 1238 조회에서 OPEN·Draft=false |

원문·파일·커밋·reviewer 재조회는 ignored
`pr8049-current-review-1238.json`에 보존했다. 전체45개·Draft10개의 원 head도1238에서
이전1119와 같음을 확인했다. 변경 파일은 `paragraph_layout.rs`와
`caption_auto_number_position.rs`다. 현재 원격 CI·merge 가능 상태를 완료로 추정하지 않는다.

## 원 주장과 현재 구현의 소비 경로

원 PR은 캡션의 자동 번호 위치를 원문 컨트롤 주소에서 찾는다. 두 공백을 찾던 방식은
`<그림 [번호]>`의 자리표를 남기거나 `<표 Ⅱ-[번호]>`의 번호를 누락했다. 원 작성자는
두 문자열의 회귀 검사와 원 head의 전체 테스트 결과를 제시했다.

통합 후보는 `control_source_positions()`의 원문 자리표 한 칸을
`composer/auto_number_projection::project_caption`에서 표시 번호로 투영한다.
`layout/picture_footnote::layout_caption`이 이 구성을 실제 캡션 줄 배치에 전달한다.
모델의 `text` 길이는 한 칸을 유지하고, paint·실제 화면 문자열은 `display_text`를
소비한다. 뒤 글자·TAC의 원문 위치를 번호의 자릿수만큼 전진시키지 않는다.

기존 검사의 `page_text`는 모델 `run.text`만 읽어 표시 번호를 누락했다. 독립 Print와
실제 Native/fresh 출력에서 번호가 있는 것을 확인한 뒤 공개 표시 계약인
`TextRunNode::display_or_text()`를 읽도록 보정했다. 기대 문자열과 공차는 유지했다.

## 독립 시각 증거와 실제 검사

[입력·독립 PDF·전체 TSV·대표 review/overlay](../assets/caption_autonumber_projection_20261021/README.md)와
[해시·Print·명령 출처](../assets/caption_autonumber_projection_20261021/provenance.json)를 보존했다.
같은 원본과 기존 검사의 같은 XML 반례이며, 새 대조군으로 원본을 대체하지 않았다.
실제 한컴2020 `11.0.0.9136`·PrintMethod0·one-up 출력이다.

| 입력 | Native082 raw | fresh033 raw | 전체 쪽수 |
| --- | ---: | ---: | --- |
| 원본 `ta-pic-001-r.hwpx` | 99.98114% | 99.98117% | 양쪽1=PDF1 |
| 공백 없는 기존 반례 | 99.98115% | 99.98118% | 양쪽1=PDF1 |

독립 Print의 실제 문자는 각각 `<그림 1>`, `<표 Ⅱ-1>`이다. 같은 배율의 Native/fresh
review·standalone overlay에서 번호·앞뒤 공백·그림과 캡션의 관계를 직접 판독했다.
얇은 표 선의 raster 농도 차이는 남는다. raw99.98%를 RGB 동일로 해석하지 않는다.

고정 devel의 Native raw는85.24536%/85.55080%, fresh는85.29363%/85.57508%였다.
현재 후보의 개선과 기존 base의 미달을 구분하며, 미달을 이유로 기존 검사를 이관하지 않았다.
같은 Native082 제품에서 검사 소비값 보정 전에는2FAIL, 보정 뒤에는2PASS였고 두
strict build는 모두 exit0이다. 이 비교는 테스트의 원문/표시 소비 계약을 확인한다.
실행 원문은 ignored `caption-current-{native,wasm}-1244` 및 `caption-test-consumer-*-1250.json`에 있다.

## 조판 원칙과 남은 통합 조건

| 항목 | 판정·근거 |
| --- | --- |
| 구현 주장과 반례 | 충족: 원본과 앞뒤 공백이 없는 기존 반례에서 실제 표시 번호 확인 |
| 측정·배치의 공통 결과 | 충족: 원문 한 칸과 표시 문자열을 함께 둔 composed run을 캡션 배치가 소비 |
| 검사 기대값의 독립 근거 | 충족: 원 기대 문자열 유지, 독립 Print와 같은 화면 문자열을 검사 |
| 전쪽 Native/fresh 시각 | 충족: 두 입력의 전체1쪽이 모두99.981%대, 누락·쪽수 차이 없음 |
| 확대 판독 | 충족: 번호·주변 공백·그림/캡션 관계 확인, 선 농도 차이 별도 기록 |
| 통합 후보 전체 gate | 진행 중: 전체 Rust 실패 보정, 최신 코드 lint·fresh 전쪽·다른 PR 기능 보존 검증 |

전체 통합의 필수 실패를 해결한 뒤 최종 판정을 갱신한다. 원 작성자의 전체 통과 주장과
현재 누적 후보의 전체 테스트 결과를 합치지 않는다. 승인된 통합의 실제 merge SHA를
확인한 뒤 원 PR별 기여 기록·대표 PNG·후속 조치를 연결한다.

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
