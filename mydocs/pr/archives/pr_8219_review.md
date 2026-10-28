# PR #8219 리뷰 — HWP3 연결 그림의 결정적 순서

> **2026-10-28 통합 후보 검증:** 이전 전체 회귀는 완료됐고 변경 검사와 Native·fresh WASM 비교 결과를 추가했다. 아래 과거 결과는 각 측정 당시 head의 기록이다. 한국어 가정통신문 2쪽의 0.1%p 초과 하락 및 일부 측정 근거는 미해결이므로 통합 후보의 최종 통과로 해석하지 않는다. 현재 상태는 [통합 계획](../../working/review_integration_20261026_document_updates_plan.md)을 따른다.

## 최종 판정

**통합 후보 최종 판정 보류 — HWP3 연결 그림의 결정적 순서의 관련 결과는 아래에 기록했다.**
원 head의 단독 실행이나 원격 승인을 뜻하지 않는다. 원 변경을 누적 적용한 아래 통합 후보에 대한 판정이며,
통합 PR의 최신 head CI·검토와 별도 머지 지시는 이후 조건이다. 작업지시자가 승인한 통합 PR은 남은 검사와 미해결 기준을 처리한 뒤 제출한다.

## 접수와 적용

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#8219](https://github.com/edwardkim/rhwp/pull/8219) / planet6897 / devel |
| 고정 원 head | `bfe5d44cf1d08806d9ac12aff1096096ed0853c0` |
| 원 PR의 이슈 연결 | 원 PR 자동 닫기 연결 없음; 통합 PR의 자동 닫기로 전환하지 않음 |
| 원 head CI 참고값 | SUCCESS; 통합 후보 CI와 구분 |
| 검증한 통합 code/test head | `2e902abacf328bccdd7780606666ac3b1c5c1ce9` |
| 시각 production source | `ae8c413f84a01a4cc4591357056183709f21b150` |
| 고정 기준 devel | `faa885e8b54ffc7cdf4196cd43a3103f362499a2` |
| 실제 적용·중복 | 1개 적용 / 0개 patch 중복 제외 |
| reviewer·조회 시점 상태 | jangster77 지정 유지 / OPEN / mergeable=MERGEABLE; 머지 전 재조회 필요 |

브랜치는 `review/integration-20261026-document-updates`다. [적용 순서·SHA·단계 기록](<../../../mydocs/pr/archives/pr_8219_review_impl.md>)과
[원 head·reviewer·댓글·스레드 재조회](<../../../mydocs/working/assets/review_integration_20261026/validation/source-pr-preflight.json>), [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)을 연결한다.

## 변경과 실제 검증 범위

연결 그림 BinData를 id 순으로 배치한다. 여러 번 파싱한 순서뿐 아니라 저장 HWP5 재열기 뒤 각 그림의 외부 경로가 원래 그림과 일치하는지 확인한다.

원 PR 제목: fix(hwp3): 연결 그림 BinData 를 id 순으로 놓아 저장·렌더를 결정적으로 (#8218)

주요 구현 경로: [mod.rs](<../../../src/parser/hwp3/mod.rs>).

보호 검사: [issue_8218_hwp3_link_bindata_order.rs](<../../../tests/cases/issue_8218_hwp3_link_bindata_order.rs>). 전체 nextest 또는 해당 Native/Studio 실행의 실제 결과에 포함된다.

이 PR에 별도 단독 보정은 추가하지 않았다. 누적 통합에 필요한 컴파일 호환 보정과 다른 PR의 공통 경로 보정은 통합 후보에 포함된다.

## 검증 입력과 결과

[전체 로컬 실행·명령·exit·검증 SHA](<../../../mydocs/working/assets/review_integration_20261026/validation/final-rust-validation.json>)에 fmt·세 Clippy·workspace build·고정 base 정책·전체 nextest·Native 실행 결과를 보존한다.
Studio 단위 검사는 2,607개 통과·기존 skip 2개이며 실제 브라우저와 fresh 최적화 WASM도 실행했다. [production source 동일성](<../../../mydocs/working/assets/review_integration_20261026/validation/production-source-continuity.json>)과 [WASM 빌드 출처](<../../../mydocs/working/assets/review_integration_20261026/validation/final-wasm-build.json>)로 검증 head를 구분한다.

이 PR의 직접 검사는 사용자가 지정한 `samples/hwp3-sample4.hwp`를 사용한다. 원본의 그림 13개는 모두 내장이며 HWP5 저장 후 ID·순서·바이트가 보존되는지 확인한다. 원 PR의 외부 연결 순서 계약은 같은 원본의 그림 레코드 두 개만 메모리에서 연결형으로 바꾼 합성 입력으로 별도 확인하며, 원본에 외부 연결이 있다고 주장하지 않는다. 원본의 독립 한컴 2020 Print는 36쪽이며 21쪽은 Native·fresh WASM 각 100.00000%, 전체 최저는 1쪽 Native 64.32418% / WASM 64.81033%다. 21쪽 내장 그림은 독립 Print 좌표를 사용한다. [실제 원본·Print·전쪽 TSV](<../../../mydocs/pr/assets/issue8219/sample4/evidence.json>). 전체 90% 기준은 통과하지 않았다. 같은 원본·새 PDF·현재 글꼴 환경에서 고정 base와 후보의 Native 및 WASM PNG가 36쪽 모두 바이트 단위로 같고 0.1%p 이상 하락은 없다. 이 동등성은 기존의 1쪽 미달을 해결했다는 뜻은 아니다.

작업지시자는 1쪽을 글꼴 제약으로 더 보정하지 않고 진행하도록 했다. 원본 1쪽은 `중간공한`을 요청한다. OFL 후보 동글 Regular를 별도 환경에서 명시적으로 치환한 Native 1쪽은 raw 41.54053%로 기존 64.32418%보다 낮아 적용하지 않았다. 글꼴 치환은 조판과 출력 양쪽에 적용한 실험이며, 이 결과만으로 1쪽의 모든 차이 원인을 확정하지 않는다. 미달은 유지하며 사용자의 진행 지시와 구분해 기록한다.

## 조판 원칙 준수 판정

| 항목 | 판정과 근거 |
| --- | --- |
| 제출 전 로컬 검증 | 진행 중 — [이전 로컬 실행](<../../../mydocs/working/assets/review_integration_20261026/validation/final-rust-validation.json>)과 후속 검증은 실행 당시 후보를 구분한다. 남은 비교와 미해결 기준은 [통합 계획](../../working/review_integration_20261026_document_updates_plan.md)에서 추적한다. 전체 회귀는 작업지시자 지시에 따라 재실행하지 않는다. |
| 기존 해결의 선행 분석 | 충족 — [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>); 원 이슈·원 해결·보정 SHA를 실패별로 추적 |
| 구현 근거와 일반성 | 충족 — 위 실제 소비 경로와 독립 기대값. 문서 ID 보정·출력 은폐 없이 공통 결과 사용 |
| 측정·배치 일관성 | 비해당 — 글꼴 순수 판정·digest 또는 저장 payload·순서·AI 전송 계약이며 조판 계산을 바꾸지 않음 |
| 리베이스 기능 보존 | 충족 — #8116 새 base의 15개 계약·4개 문서와 기존 17개 계약·전쪽을 함께 검증. 한 충돌에서 두 수용 조건을 보존했으며 커밋 대조와 실행 원장에 기록 |
| 최종 출력 소비 | 충족 — 글꼴·digest의 독립 입력 반례, payload 재열기·교체 반례 또는 실제 Studio UI |
| 분할·이어받기 | 비해당 — 표 조각 컷·이어받기 규칙은 변경하지 않음 |
| 줄 소속과 점유 높이 | 비해당 — 새 줄 배치 규칙 없음 |
| 독립성·반례 | 충족 — 실제 원본·독립 Print 또는 원본 payload/외부 경로·편집 대조군. 임시 스크립트를 정식 회귀로 대신하지 않음 |
| 기준값 변경 | 비해당 — 이 PR의 baseline·golden·허용치를 완화하지 않음 |
| 회귀 추가 선행 조건 | 비해당 — 새 물리 배치/pixel 기대값 없음 |
| 수식·간격 확대 판독 | 비해당 — 물리 배치 변경 없음 |
| 구현 주장과 검증 범위 | 충족 — 고정 source와 실제 결과를 연결. 원 head 단독 CI·정량 성능·미실행 경로로 범위 확장하지 않음 |

## 시각 증적과 남은 조건

원 fork head는 변경하지 않았다. 실제 편람 수정 #8268·충돌 #8161은 제외하며 추가 PR을 체리픽하지 않는다.

## Merge 후 contributor comment 계획

현재는 게시·close·merge를 실행하지 않는다. 추후 승인된 통합 머지 뒤 최종 merge SHA·CI URL과
Visual Sweep 정본·SHA 고정 raw 이미지·원 기여와 보정 범위·남은 차이를 정중한 한국어로 안내한다.
본문 파일로 게시하고 API로 실제 내용과 링크를 재확인하는 단계는 별도 후속 지시 대상이다.

## 2026-10-29 중간공한 설치 후 1쪽 확인

최종 중간공한 설치 뒤 sample4 1쪽만 같은 독립 PDF 래스터로 측정했다. Native 64.32418% → 66.60007%, WASM 64.81033% → 66.44902%다. 새 글꼴 환경에서 후속 표 보정 전후 PNG는 양 경로 모두 같다. WASM은 새 글꼴을 명시 공급해 해당 글꼴 SHA의 실제 로드를 확인했다. 양쪽 모두 90% 미만이며 글꼴 공급 조건이 달라 결과 차이의 유일한 원인을 단정하지 않는다. 사용자 결정에 따라 이 쪽을 추가 보정하지 않고, 다른 35쪽은 이 실험으로 재측정하지 않았다. [1쪽 측정·해시](<../../pr/assets/issue8219/sample4/final-konghan-page1.json>).

제목의 인접 한글 9쌍은 한컴 PDF의 글자 원점 간격이 새 글꼴의 자연 전진폭보다 약 0.05em 크다. 원본 파싱 결과는 굵게·자간 0%·장평 100%다. PDF 글자 경계와 rhwp run 경계의 차이는 확인했으나 서로 다른 종류의 경계이며 단독으로 원인을 확정하지 않는다. [글자별 진단](../../pr/assets/issue8219/sample4/final-konghan-page1.json).
