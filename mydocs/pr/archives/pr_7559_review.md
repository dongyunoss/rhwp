---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7559_review.md
last_verified: 2026-10-21
---

# PR #7559 리뷰 — 새 표의 저장 전후 원점

## 최종 판정

**머지 보류 — 메인터너 보정의 집중 검사는 통과했으며 통합 후보의 필수 검증을 계속한다.**
원 PR은 실패 재현만 담은 Draft다. 보정 없이 원 head를 병합 가능한 수정으로 판정하지 않는다.
현재 기록은 로컬 검토이며 원격 review·push·PR 생성·merge는 실행하지 않았다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR | [#7559](https://github.com/edwardkim/rhwp/pull/7559), semanticist21, devel 대상 Draft |
| 2026-10-21 조회 head | `5f1f1057bd1b7981b3801b815aa28b98860932a2` |
| 기능 commit | `ebba2950fe97e00d38715e05236f6182f1a74da8`, `2ee1ad24b403d384262d499db071bca10f6be5d5` |
| 로컬 cherry-pick | `675cf1802e`, `8caef79030`; 나머지 head commit은 devel merge |
| reviewer | jangster77; 기존 요청 API 영수증 확인 |
| 통합 branch | `review/semanticist21-20261019` |
| 최신 집중 검사 제품 | `9810f26ccfbf9073679a7b79c3cfa5ece18905b6`, Native078 |

원문·commit·변경 파일 조회는 ignored output의 `pr7559-current-review-1142.json`,
reviewer 영수증은 `pr-7559-reviewers.json`에 있다. 변경 파일은
`tests/cases/table_hwpx_save_origin_contract.rs` 하나다. 현재 파일은 원 기능 commit
`2ee1ad24b4`의 바이트와 동일하며 SHA-256은
`e5ef237b0abe215694b4c3ec0e6492d576ef18474d58aa11d8c42ae8e8f2bb2d`다.

## 원 주장과 독립 기대값

원 PR은 공개 API로 표를 만든 뒤 셀 안 여백 또는 표 offset을 변경하고, 편집 중·HWP
재열기·HWPX 재열기의 원점과 쪽수 보존을 검사한다. 작성자는 두 실패를 명시했고,
기존 #7265의 한컴 정본 여백을 되돌려 통과시키면 안 된다고 설명했다.

#7265 → 실제 통합 #7430, 관련 #7538의 미병합 상태와 #7518 → #7570의 계산 줄 계약을
먼저 대조했다. 분석·코드 소비 위치·실행 기록은
[계산 빈 호스트 보정](../../working/task_semanticist21_20261020_computed_empty_host_frame.md)에 있다.
예전 저장 fixture의 통과를 현재 공개 API 입력의 검증으로 대신하지 않았다.

현재 API의 offset·padding 및 HWP/HWPX 네 입력을 각각 독립 한컴2020 MCP Print로
출력했다. 호스트의 계산 LineSeg를 정상 저장 줄로 오인하거나 높이 0으로 없애지 않고,
빈 글줄·표 앞 여백·양수 offset·표 높이·뒤 여백의 물리 점유를 공통 결과로 계산했다.
fit과 실제 표 배치가 같은 표 원점·점유 끝을 소비한다. 뒤 문단도 표를 피하는 첫 줄의
확정 원점을 사용하며 문서 ID·글자 크기·좌표별 예외는 없다.

## 시각 증거와 회귀 검사

관련 검증 자료는 [계산 빈 호스트 증거](../assets/computed_empty_host_line_20261020/README.md)에
연결했다. 제품 `bc649064faaa2193380c1ab5b2567b4470f2472e`의 14개 경계 입력은
각 전 1쪽 Native/fresh WASM raw100%이며, 공개 API 원본 네 조건과 합친 18조건도
양쪽100%다. 실제 한컴2020 `11.0.0.9136`·PrintMethod0·1-up PDF다.
같은 입력·Print·글꼴의 고정 devel 대조와 확대 판독을 완료한 과거 제품의 증거이며,
최신 통합 head의 최종 전쪽 결과로 재표기하지 않는다.

원 PR의 저장 전후 원점 불변 검사 두 개는 기대값을 바꾸지 않고 Native078에서 모두
통과했다. 공통 빈 호스트 관계 검사 5개와 형제 표 관계 검사 7개도 같은 제품에서
통과했다. 실행 명령·정식 소스 SHA·로그는
`emphasis-metric-current-failure-focus-1135.json`에 있다.
원점 불변의 0.2px 허용 오차를 독립 PDF의 절대 좌표 기준값으로 설명하지 않는다.

## 조판 원칙과 통합 판정

| 항목 | 판정·근거 |
| --- | --- |
| 원 주장과 검사 | 공개 API의 두 저장 경로·쪽수·원점 보존을 직접 검사; 원 검사 바이트 유지 |
| 공통 결과의 소비 | `computed_host_frame`의 표 원점·점유 끝을 fit과 실제 배치가 함께 소비 |
| 핵심 반례 | 작은/큰 빈 줄·양수 offset·다음 문단 앞 간격·표 위에 들어가는 글줄과 회피 글줄 |
| 회귀 검출 | 빈 호스트 정식 관계 검사 5개는 이전 제품에서 4FAIL·정상 대조 1PASS, 보정 뒤 5PASS |
| 독립 시각 증거 | 위 과거 제품의 전쪽 Native/fresh100%와 직접 판독·원문 TSV·독립 Print 보존 |
| 최신 통합 gate | fresh030·전체 Rust·lint·관련 전쪽 시각 및 최신 devel 기능 보존 검증 진행 중 |

필수 통합 검증을 완료한 뒤 최종 판정을 갱신한다. 전체 Rust1085의 별도 실패를 집중
검사 통과로 해소했다고 보고하지 않는다. 실제 통합 승인·merge 뒤에는 원 재현 기여와
보정 범위, 실제 merge SHA 및 대표 review·overlay를 원 PR에 연결하는 후속 처리를 한다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/table_hwpx_save_origin_contract.rs](../../../tests/cases/table_hwpx_save_origin_contract.rs): SHA-256 `e5ef237b0abe215694b4c3ec0e6492d576ef18474d58aa11d8c42ae8e8f2bb2d`.
  - `generated_table_padding_origin_survives_hwpx_save`: PASS, `logs/final-full-nextest-4052.log:8774` (`rhwp::regression_suite_017`).
  - `generated_table_offset_origin_survives_hwpx_save`: PASS, `logs/final-full-nextest-4052.log:8773` (`rhwp::regression_suite_017`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `ebba2950fe97e00d38715e05236f6182f1a74da8` | `d206926acd8f493623bf199a9fe8d2e542ab97c6` |
| `2ee1ad24b403d384262d499db071bca10f6be5d5` | `b76531a100b4a5f796e10659cd5617515e428d4d` |
| `5f1f1057bd1b7981b3801b815aa28b98860932a2` | 원 devel merge; 기능 체리픽에서 제외 |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.
