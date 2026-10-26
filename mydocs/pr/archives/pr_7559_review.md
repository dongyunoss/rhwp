---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7559_review.md
last_verified: 2026-10-26
---

# PR #7559 리뷰 — 새 표의 저장 전후 원점

## 최종 판정

**메인터너 보정 후 수용 가능 — 공개 API 표의 저장 전후 원점·빈 호스트 점유 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 아래 현재 증거와
실제 검사 실행을 대조했다. 통합 PR의 정확한 code head CI·merge·후속 처리는
앞으로 수행할 조건이며 로컬 통과로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

원 head `5f1f1057bd1b7981b3801b815aa28b98860932a2`가 원격과 같음을 재확인했다. 검증 코드는
`61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 제출 전 fetch에서도 같은 base를
확인했으므로 추가 리베이스하지 않았다. 이후 문서·증거 커밋은 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 원 PR의 현재 검사 | 아래 모듈·함수별 실제 PASS와 로그 행을 확인했다. 소스 존재를 실행 결과로 대신하지 않았다. |
| 전쪽 Native/fresh WASM | 18조건·각 1쪽(전체 18쪽/backend)에서 최저 100.00000% / 100.00000%, 독립 Print 쪽수 일치·90% 미만/누락 0쪽 |
| 고정 devel 보존 | 동일 입력·Print·글꼴·backend의 페이지별 비교에서 0.1%p 초과 하락 0쪽 |
| 직접 판독 | 표 원점·행 높이와 뒤 본문 순서를 확인했다. 원점 불변의 0.2px는 저장 전후 관계 검사이며 독립 PDF의 절대 좌표 핀이 아니다. |
| 전체 로컬 게이트 | fmt·수집/단위 정책·Release build/lib·전체 nextest·Skia 3종·Native/WASM/workspace Clippy·workspace build·doctest 완료. 실제 종료 근거는 아래 공통 기록 참조 |
| 원격 절차 | 통합 PR의 정확한 code head CI·실제 merge·후속 comment는 앞으로 수행할 조건이다. 원 contributor head를 직접 승인·merge한 기록이 아니다. |

조건 번호 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 059, 060, 061, 062. [전쪽 TSV·원본/Print·producer·해시와 판독](../assets/source_pr_qualified_20261026/README.md)을
이 PR의 직접 근거로 연결한다. raw는 색상 경계 보정 전 2px 실루엣이며 RGB 완전 일치나
과거 별도 지표의 통과를 뜻하지 않는다. Native255·표준 release/wasm-opt를 마친 fresh073을
사용했다. 측정 뒤 코드 변경은 없으며, 과거 제품의 표는 당시 결과로 구분한다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/source_pr_qualified_20261026/png/native/046/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/native/046/overlay_001.png) |
| fresh WASM | [review](../assets/source_pr_qualified_20261026/png/fresh-wasm/046/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/fresh-wasm/046/overlay_001.png) |

Merge 후에는 이 대표 파일을
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/source_pr_qualified_20261026/`
위 표의 실제 상대 경로로 고정해 contributor comment에 두 이미지를 함께 표시한다.
같은 merge의 devel 포함·asset 존재·CI 결과를 먼저 확인하고 한국어 존댓말의
`--body-file` 본문을 게시한 뒤 API에서 한글·링크·실제 Markdown을 다시 확인한다.


## 원 기여의 현재 커밋 대응

현재 대응표는 통합 head `fcc9c69c006c3cd134a0ed0dcd1f2822bc32aa2d`·기준 devel `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`의 실제 history를 대조했다. 원 head의 merge 전용 커밋과 이미 포함된 동등 구현은 기능 체리픽과 구분한다. 이 표는 커밋 출처이며 전체 검증 완료 판정은 아니다.

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `ebba2950fe97e00d38715e05236f6182f1a74da8` | `31c9b59bf291209963b87ec04d3af80a7c93daff` |
| `2ee1ad24b403d384262d499db071bca10f6be5d5` | `e8f140492570fdf65dd48872df006e6d3ce8e5d3` |
| `5f1f1057bd1b7981b3801b815aa28b98860932a2` | 원 PR의 merge 전용 커밋: 별도 기능 적용 없음 |

## 접수 정보

이하 최초 접수와 중간 검토의 제품·기준 SHA 및 당시 진행 상태를 보존한다.
현재 판정은 위 「최종 로컬 판정 근거」와 아래 「통합 후보의 실제 검사 실행」이 정본이다.
과거의 진행 중·미완료 표현을 현재 로컬 게이트 상태로 사용하지 않는다.

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

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/table_hwpx_save_origin_contract.rs](../../../tests/cases/table_hwpx_save_origin_contract.rs): SHA-256 `e5ef237b0abe215694b4c3ec0e6492d576ef18474d58aa11d8c42ae8e8f2bb2d`.
  - `generated_table_offset_origin_survives_hwpx_save`: PASS, `logs/current-full-nextest-4184.log:10121` (`rhwp::regression_suite_022`).
  - `generated_table_padding_origin_survives_hwpx_save`: PASS, `logs/current-full-nextest-4184.log:10126` (`rhwp::regression_suite_022`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
