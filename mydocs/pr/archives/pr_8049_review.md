# PR #8049 리뷰 — 캡션 자동 번호의 자리표와 표시 문자열

## 최종 판정

**메인터너 보정 후 수용 가능 — 캡션 자동 번호의 표시 문자열·앞뒤 간격 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 아래 현재 증거와
실제 검사 실행을 대조했다. 통합 PR의 정확한 code head CI·merge·후속 처리는
앞으로 수행할 조건이며 로컬 통과로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

원 head `285f86c92c27eda03616491a1d1f0c8210c9b80a`가 원격과 같음을 재확인했다. 검증 코드는
`61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 제출 전 fetch에서도 같은 base를
확인했으므로 추가 리베이스하지 않았다. 이후 문서·증거 커밋은 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 원 PR의 현재 검사 | 아래 모듈·함수별 실제 PASS와 로그 행을 확인했다. 소스 존재를 실행 결과로 대신하지 않았다. |
| 전쪽 Native/fresh WASM | 2조건·각 1쪽(전체 2쪽/backend)에서 최저 99.98107% / 99.98109%, 독립 Print 쪽수 일치·90% 미만/누락 0쪽 |
| 고정 devel 보존 | 동일 입력·Print·글꼴·backend의 페이지별 비교에서 0.1%p 초과 하락 0쪽 |
| 직접 판독 | 실제 그림/표 접두사·번호와 뒤 설명을 대조했다. 원 기대 문자열을 유지하며 검사 소비자를 display_or_text로 보정한 수정 전 FAIL·수정 후 PASS 근거를 유지한다. |
| 전체 로컬 게이트 | fmt·수집/단위 정책·Release build/lib·전체 nextest·Skia 3종·Native/WASM/workspace Clippy·workspace build·doctest 완료. 실제 종료 근거는 아래 공통 기록 참조 |
| 원격 절차 | 통합 PR의 정확한 code head CI·실제 merge·후속 comment는 앞으로 수행할 조건이다. 원 contributor head를 직접 승인·merge한 기록이 아니다. |

조건 번호 029, 030. [전쪽 TSV·원본/Print·producer·해시와 판독](../assets/source_pr_qualified_20261026/README.md)을
이 PR의 직접 근거로 연결한다. raw는 색상 경계 보정 전 2px 실루엣이며 RGB 완전 일치나
과거 별도 지표의 통과를 뜻하지 않는다. Native255·표준 release/wasm-opt를 마친 fresh073을
사용했다. 측정 뒤 코드 변경은 없으며, 과거 제품의 표는 당시 결과로 구분한다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/source_pr_qualified_20261026/png/native/029/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/native/029/overlay_001.png) |
| fresh WASM | [review](../assets/source_pr_qualified_20261026/png/fresh-wasm/029/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/fresh-wasm/029/overlay_001.png) |

Merge 후에는 이 대표 파일을
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/source_pr_qualified_20261026/`
위 표의 실제 상대 경로로 고정해 contributor comment에 두 이미지를 함께 표시한다.
같은 merge의 devel 포함·asset 존재·CI 결과를 먼저 확인하고 한국어 존댓말의
`--body-file` 본문을 게시한 뒤 API에서 한글·링크·실제 Markdown을 다시 확인한다.


## 원 기여의 현재 커밋 대응

현재 대응표는 통합 head `fcc9c69c006c3cd134a0ed0dcd1f2822bc32aa2d`·기준 devel `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`의 실제 history를 대조했다. 원 head의 merge 전용 커밋과 이미 포함된 동등 구현은 기능 체리픽과 구분한다. 이 표는 커밋 출처이며 전체 검증 완료 판정은 아니다.

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `285f86c92c27eda03616491a1d1f0c8210c9b80a` | `7ba00793b7198c8401f7196558930e5cd3cdb72f` |

## 접수 정보

이하 최초 접수와 중간 검토의 제품·기준 SHA 및 당시 진행 상태를 보존한다.
현재 판정은 위 「최종 로컬 판정 근거」와 아래 「통합 후보의 실제 검사 실행」이 정본이다.
과거의 진행 중·미완료 표현을 현재 로컬 게이트 상태로 사용하지 않는다.

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
