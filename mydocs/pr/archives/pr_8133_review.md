# PR #8133 리뷰 — 장치 화소 획 하한

## 최종 판정

**머지 보류 — #8116 리베이스 후 양쪽 기능 보존 검증 진행 중.**

아래 완료 결과는 리베이스 전 고정 후보의 실제 기록이다. 새 base의 최종 수용 판정은 추가 검증 뒤 갱신한다.
원 head의 단독 실행이나 원격 승인을 뜻하지 않는다. 원 변경을 누적 적용한 아래 통합 후보에 대한 판정이며,
통합 PR의 최신 head CI·검토와 별도 머지 지시는 이후 조건이다. 이번 작업은 통합 PR 제출까지다.

## 접수와 적용

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#8133](https://github.com/edwardkim/rhwp/pull/8133) / planet6897 / devel |
| 고정 원 head | `465d85f13cab961628ec07247ea6ade82e0cbc46` |
| 원 PR의 이슈 연결 | 원 PR 자동 닫기 연결 없음; 통합 PR의 자동 닫기로 전환하지 않음 |
| 원 head CI 참고값 | SUCCESS; 통합 후보 CI와 구분 |
| 검증한 통합 code/test head | `1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559` |
| 시각 production source | `f272b81c28b2525c5f6e520ef1e5d499b16672d3` |
| 고정 기준 devel | `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c` |
| 실제 적용·중복 | 12개 적용 / 0개 patch 중복 제외 |
| reviewer·조회 시점 상태 | jangster77 지정 유지 / OPEN / mergeable=MERGEABLE; 머지 전 재조회 필요 |

브랜치는 `review/integration-20261026-document-updates`다. [적용 순서·SHA·단계 기록](<../../../mydocs/pr/archives/pr_8133_review_impl.md>)과
[원 head·reviewer·댓글·스레드 재조회](<../../../mydocs/working/assets/review_integration_20261026/validation/source-pr-preflight.json>), [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)을 연결한다.

## 변경과 실제 검증 범위

실제 출력 좌표계에서 장치 한 화소를 계산해 획 하한을 적용한다. 점선의 주기는 보정한 획이 아닌 원래 명목 굵기를 사용한다. CanvasKit 기록의 외부 변환·cache key·예외 복원도 확인했다.

원 PR 제목: fix(renderer): Native Skia·WASM Canvas2D 획은 장치 한 칸보다 가늘게 그리지 않는다 (#8125)

주요 구현 경로: [canvaskit-renderer.ts](<../../../rhwp-studio/src/view/canvaskit-renderer.ts>), [line_replay.rs](<../../../src/renderer/skia/line_replay.rs>), [renderer.rs](<../../../src/renderer/skia/renderer.rs>), [synthetic_typeface.rs](<../../../src/renderer/skia/synthetic_typeface.rs>), [web_canvas.rs](<../../../src/renderer/web_canvas.rs>).

보호 검사: [issue-8125-hairline-canvas2d.test.mjs](<../../../rhwp-studio/e2e/issue-8125-hairline-canvas2d.test.mjs>), [issue_7445_biotoxin_source_ownership.rs](<../../../tests/cases/issue_7445_biotoxin_source_ownership.rs>), [native_fractional_glyph_origin.rs](<../../../tests/cases/native_fractional_glyph_origin.rs>). 전체 nextest 또는 해당 Native/Studio 실행의 실제 결과에 포함된다.

추가 보정: `343142255a9ba509223bfff99016aad68eb1d16e`, `7ccf8dffc8afe509fb08b20ddb4bab292178d87a`, `d0ba736ddaa89917450bec032e807e55ee27aa7f`. 기여자의 원 변경과 구분하며 일반성·독립 근거·수정 전후는 통합 계획에 기록했다.

## 검증 입력과 결과

[전체 로컬 실행·명령·exit·검증 SHA](<../../../mydocs/working/assets/review_integration_20261026/validation/final-rust-validation.json>)에 fmt·세 Clippy·workspace build·고정 base 정책·전체 nextest·Native 실행 결과를 보존한다.
Studio 단위 검사는 2,607개 통과·기존 skip 2개이며 실제 브라우저와 fresh 최적화 WASM도 실행했다. [production source 동일성](<../../../mydocs/working/assets/review_integration_20261026/validation/production-source-continuity.json>)과 [WASM 빌드 출처](<../../../mydocs/working/assets/review_integration_20261026/validation/final-wasm-build.json>)로 검증 head를 구분한다.

| 입력 조합 | 전체 쪽수 | Native 최저 raw | fresh WASM 최저 raw |
| --- | ---: | ---: | ---: |
| onsemiro-hwp | 46 | [94.85728%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/native/onsemiro-hwp.tsv>) | [90.94873%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/wasm/onsemiro-hwp.tsv>) |
| onsemiro-hwpx | 46 | [94.85728%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/native/onsemiro-hwpx.tsv>) | [91.00639%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/wasm/onsemiro-hwpx.tsv>) |
| hairline-calendar | 1 | [98.83153%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/native/hairline-calendar.tsv>) | [98.51200%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/wasm/hairline-calendar.tsv>) |
| hairline-sungeo | 94 | [99.85165%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/native/hairline-sungeo.tsv>) | [98.74889%](<../../../mydocs/working/assets/review_integration_20261026/validation/tsv/wasm/hairline-sungeo.tsv>) |

해당 조합은 모두 독립 PDF의 전쪽과 대응하며 90% 미만·후보 쪽수 불일치·누락이 없다. 실제 경로·입력/PDF SHA·명령은 [전쪽 실행 원장](<../../../mydocs/working/assets/review_integration_20261026/validation/visual-validation.json>),
[커밋 blob과 실제 입력의 해시 감사](<../../../mydocs/working/assets/review_integration_20261026/validation/tracked-input-audit.json>), [실제 글꼴·PDF 래스터 출처](<../../../mydocs/working/assets/review_integration_20261026/validation/font-and-raster-provenance.json>)에 있다.

[동일 조건 base/head 판정](<../../../mydocs/working/assets/review_integration_20261026/validation/base-head-comparison.json>)에서 Native·WASM 각각 60개 조합·895개 기준 쪽을 비교했고 2%p 이상 하락은 0쪽이다. 미세 하락 여부·최대 하락은 원장의 실제 값을 따른다.
기준의 공식 사양 83쪽/71쪽과 cellnotes 12쪽/11쪽 오류는 기준 실패로 보존하고 모든 기준 쪽을 별도 진단 비교했다.
새 editable 3모드는 구 버전에 없으므로 독립 PDF·실제 본문 소유 검사로 별도 검증했다.

[현재 Native 확대 판독과 기존 외양 차이](<../../../mydocs/working/assets/review_integration_20261026/validation/v14-native-reading-notes.json>)에서 점수 통과·프레임 보정과 남은 글꼴·작은 표식·도식 빗금 외양 차이를 구분했다. 이를 새 회귀나 해결 완료로 바꾸어 보고하지 않는다.

CanvasKit 전체 sungeo UI는 shaping·작업량 상한에 따른 Canvas2D 전환이다. 전체 문서 CanvasKit 통과로 세지 않는다. calendar 실제 CanvasKit UI와 sungeo 34쪽의 원래 선 계층·clip을 보존한 두 모드 × 두 배율 검사를 구분했다. 기존 gray-grid의 색상 기대값은 기여자가 제시한 직접 한/글 2024 출력의 191±8 근거이며 위치 핀은 유지했다.

## 조판 원칙 준수 판정

| 항목 | 판정과 근거 |
| --- | --- |
| 제출 전 로컬 검증 | 충족 — [전체 로컬 실행·명령·exit·검증 SHA](<../../../mydocs/working/assets/review_integration_20261026/validation/final-rust-validation.json>). 원격 CI·머지는 완료로 세지 않음 |
| 기존 해결의 선행 분석 | 충족 — [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>); 원 이슈·원 해결·보정 SHA를 실패별로 추적 |
| 구현 근거와 일반성 | 충족 — 위 실제 소비 경로와 독립 기대값. 문서 ID 보정·출력 은폐 없이 공통 결과 사용 |
| 측정·배치 일관성 | 충족 — 공통 계산·저장 줄·프레임 결과의 실제 소비와 전쪽 Native/WASM 출력 대조 |
| 리베이스 기능 보존 | 비해당 — 검토 중 base 변경·추가 리베이스 없음. 고정 devel 대비 실제 출력은 별도 전쪽 비교 |
| 최종 출력 소비 | 충족 — 실제 Native PNG·fresh WASM Canvas, 적용되는 CanvasKit 경로를 구분 |
| 분할·이어받기 | 비해당 — 표 조각 컷·이어받기 규칙은 변경하지 않음 |
| 줄 소속과 점유 높이 | 충족 — 원본 LineSeg/개체 줄·공통 flow 결과·실제 문단/개체 소유 대조 |
| 독립성·반례 | 충족 — 실제 원본·독립 Print 또는 원본 payload/외부 경로·편집 대조군. 임시 스크립트를 정식 회귀로 대신하지 않음 |
| 기준값 변경 | 충족 — 직접 한/글 출력에 근거한 기존 색상 기대값, 위치 핀 유지 |
| 회귀 추가 선행 조건 | 충족 — 관련 입력 전쪽 Native와 해당 보강 단계의 fresh WASM 최저 90% 이상 선행 확인. 보강 회귀는 소유·순서·내용 포함 관계이며 최종 최적화 WASM도 별도 완료 |
| 수식·간격 확대 판독 | 충족 — 관련 변경·최저 쪽과 독립 Print의 직접 비교. 원 PR 잔여 차이·정책 fallback은 별도 공개 |
| 구현 주장과 검증 범위 | 충족 — 고정 source와 실제 결과를 연결. 원 head 단독 CI·정량 성능·미실행 경로로 범위 확장하지 않음 |

## 시각 증적과 남은 조건

[대표 비교·overlay](<../../../mydocs/working/assets/review_integration_20261026/validation/review-panels.json>)와 [전쪽·확대 증거 목록](<../../../mydocs/working/assets/review_integration_20261026/validation/README.md>)을 연결한다. 시각 지표는 색상 경계 보정 전 raw 실루엣이며 글꼴 예외로 gate를 우회하지 않았다.

원 fork head는 변경하지 않았다. 실제 편람 수정 #8268·충돌 #8161은 제외하며 추가 PR을 체리픽하지 않는다.

## Merge 후 contributor comment 계획

현재는 게시·close·merge를 실행하지 않는다. 추후 승인된 통합 머지 뒤 최종 merge SHA·CI URL과
Visual Sweep 정본·SHA 고정 raw 이미지·원 기여와 보정 범위·남은 차이를 정중한 한국어로 안내한다.
본문 파일로 게시하고 API로 실제 내용과 링크를 재확인하는 단계는 별도 후속 지시 대상이다.
