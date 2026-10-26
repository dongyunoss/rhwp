---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7572_review.md
last_verified: 2026-10-21
---

# PR #7572 검토 — 본문 조합 중 명시적 배치의 쪽 나눔 경계

## 최종 판정

**머지 보류 — 메인터너 보정은 적용했으며 최신 통합 후보의 필수 검증을 진행 중이다.**
원 Draft의 실패 재현을 그대로 수용하는 판정이 아니다. 전체 회귀·lint·Native/fresh WASM과
고정 devel 대조를 마친 뒤 판정을 갱신한다. 원격 게시·PR 생성·merge는 수행하지 않았다.

## 접수 정보와 적용 계보

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7572](https://github.com/edwardkim/rhwp/pull/7572), semanticist21, devel 대상 Draft |
| 2026-10-21 재조회 head | `59276bb29419c3a439866d132f5be69a6c759c7c` — 동결한 검토 범위와 같음 |
| 기능 commit → 로컬 적용 | `828a2a1e14c341dcd72ea41fe582486b66495939` → `0543a267117dce58224a675fcd9d3c05246720ef` |
| 메인터너 보정 | `0eb36de6d575e27e3381a85ec86b8363568d7fbf` |
| reviewer | jangster77; 지정 영수증과 현재 요청 목록 모두 확인 |
| 통합 branch | `review/semanticist21-20261019`; 추가 PR 체리픽은 중지 |

원 본문·commit·변경 파일의 재조회 영수증은 ignored output의
`pr7572-current-review-1325.json`, 최초 지정 영수증은 `pr-7572-reviewers.json`이다.
원 head의 devel merge commit은 기능 commit과 구분했다.

## 구현 주장과 실제 소비 경로

원 PR은 편집 서식의 상속 규칙 변경을 요구하지 않는다. `beginBatch`와 `endBatch` 사이의
본문 `applyCharFormat`·`replaceBodyTextLocal`이 중간 쪽 나눔을 확정하지 않아야 한다는
계약을 실패 검사 두 개로 제시한다. 원 PR의 측정 시간은 성능 보장으로 사용하지 않는다.

보정한 `commands/formatting.rs`의 본문 서식 변경은
`rebuild_paragraph_deferred_in_batch`를 통해 스타일과 해당 문단 조합을 즉시 갱신하고,
배치 중에는 구역을 dirty로 남긴다. `commands/text_editing.rs`의 local replace도 문단
조합 결과를 갱신한 뒤 배치 여부를 먼저 확인한다. `flowChanged`가 true여도 배치 중에는
직접 `paginate()`하지 않으며 `documentPaginationPending`을 true로 반환한다.
마지막 쪽 소속은 공통 `endBatch`가 확정한다. 특정 문서·음절·좌표로 분기하지 않는다.

## 검사 항목과 증거

`tests/cases/body_ime_format_batch.rs`는 다음 세 경로를 검사한다.

| 검사 | 확인하는 계약 |
| --- | --- |
| `body_char_format_defers_pagination_until_end_batch` | 실제 쪽수가 늘어나는 비배치 대조군, 배치 중 기존 쪽수, 종료 후 페이지·캐럿 상태의 동등성 |
| `local_body_replace_defers_pagination_until_end_batch` | 실제 쪽수가 줄어드는 대조군, 기존 서식 상속, 종료 후 동일한 배치 결과 |
| `explicit_composition_format_preserves_text_caret_and_roundtrip_after_batch` | ㅎ→하→한 각 확정 경계의 텍스트·서식·캐럿, 앞뒤 문단, HWP/HWPX 저장과 snapshot 복원 |

세 검사는 `a524b12732be30d556920ccbf1102e89de538bf7`의 전체 Rust1180에서 모두 PASS했다.
이 전체 실행 자체는 다른 실패가 있어 통과로 보고하지 않는다. 최신 제품의 집중 검사와
최종 전체 게이트는 별도로 완료한다. 원 구현에서 의도한 원인으로 실패하는 실행 증거도
최종 검출 근거와 연결하며, 원 PR의 실패 주장만으로 독립 재현을 대신하지 않는다.

공개 API에서 만든 `pr7572-before`, `pr7572-all-formatted`, `pr7572-composed` HWPX와
각각의 독립 한컴 Print PDF는 `integration-fixtures-019-native`·`integration-fixtures-019-wasm`
기록에서 양쪽 전쪽 raw100%다. 생산 source는 `73b33fde72642faf71c7060bceba47358520262b`이며,
현재 head의 시각 결과로 재인용하지 않는다. 입력·PDF·TSV 해시와 실제 명령은 해당 원장에 있다.
최종 제출 전 같은 입력의 최신 양쪽 출력·직접 판독·고정 devel 비교 및 안정 경로 보존을 완료한다.

## 조판 원칙과 남은 필수 검증

일반 문단의 줄 구성 갱신과 페이지 확정 시점을 구분한다. 배치 종료 후 내용·소유·캐럿을
비배치 결과와 대조하며 절대 px 좌표를 정답으로 고정하지 않는다. 표 분할·rowspan 변경은
이 PR 자체에는 비해당이다. 편집 결과가 조판에 영향을 주므로 Print 시각 검증은 적용된다.
통합 후보의 미완료 게이트를 Draft나 원격 CI 대기로 넘기지 않는다.

## Merge 후 contributor comment 계획

실제 통합이 승인·완료되면 원 재현 검사의 기여, 배치 중 직접 페이지 확정을 막은 보정과
서식 상속 계약의 보존을 한국어 존댓말로 설명한다. 실제 merge SHA와 검증 head CI,
merge SHA로 고정한 대표 review·overlay 이미지와 전체 TSV를 연결하고 원 PR을 처리한다.
현재는 게시·close를 수행하지 않았다.

## 수정 전후의 실제 검출 증거 재확인

초기 집중 실행002의 source `119f1d9fdf4daec7336a888019829365e7472656`에서
두 결함 검사는 각각 FAIL, 정상 대조군은 PASS했다. 보정 뒤003의 source
`7614917f08880ca937f2d933540d87299eaef6b9`에서는 세 검사 모두 PASS했다.
이 옛 SHA와 현재 계보의 `0eb36de6d5`는 stable patch-id
`a28524be417518740347130b49a9a0edd5668000`가 같으며 검사 파일 바이트도 전후·현재가 같다.
`batch-cut-before-after-proof-1334.json`에 이 연결을 기록했다. 기존에 충분한 실행
증거를 재사용했으며 재컴파일만을 위해 같은 옛 소스를 다시 빌드하지 않았다.

현재 Native087의 strict 집중 실행1333도 해당3검사 모두 PASS했다. 편집 저장본의
최신 Native1328은 모든 관련 페이지100%다. fresh036 대조와 최종 전체 게이트는 계속한다.

### 최신 제품의 실제 재검증

제품 소스 `43abfbab6748e5345d848de8661098dc5b59b10e`의 전체 Rust1352에서도
`body_ime_format_batch`의 세 검사는 모두 PASS했다. 전체 실행은 다른59개 실패가 있어
통과로 보고하지 않는다. 같은 소스의 Native088·최적화 fresh037에서 위 세 편집 저장본을
전쪽 재출력했고 모두 raw100%·독립 Print PDF 쪽수 일치를 확인했다.
`batch-cut-native-1347/results.json`과 `batch-cut-wasm-1347/results.json`에 실제 명령,
입력·PDF·TSV·producer 해시를 남겼다. 옛019·1328의 결과를 최신 실행으로 바꾸어 쓰지 않았다.
대표 PNG 직접 판독·최신 devel 대조·제출 경로 보존과 통합 후보의 나머지 필수 게이트를
완료한 뒤 최종 판정을 갱신한다. 현재 판정은 머지 보류다.

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
