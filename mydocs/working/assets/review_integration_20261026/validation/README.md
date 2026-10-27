# 통합 후보의 검증 증거

최종 검증은 진행 중이다. 아래 3ae/44개·813쪽 수치는 이전 단계의 기록이며,
최신 보정·재실행은 [통합 계획](../../../review_integration_20261026_document_updates_plan.md)과
단계별 v13/v14 증거를 따른다. 전체 Rust·최종 최적화 WASM이 완료되기 전에는 제출 완료로 판정하지 않는다.

production source는 `3ae26ef0a1a20a8810813a085179220f56d2561a`이며,
기준 devel은 `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`다.
아직 전체 Rust 검사와 최종 통합 검토가 남았다. 이 문서는 해당 검사를 완료했다고 판정하지 않는다.

- [전쪽 결과·입력·명령·해시](visual-validation.json): Native와 fresh WASM 각각
  44개 조합·813쪽 비교, 90% 미만 0쪽이다. HWP/HWPX 38개 조합과 PDF 3종의
  fidelity·editable 6개 조합이다. 합산한 비교 횟수를 개별 문서 쪽수로 부르지 않는다.
- `tsv/native/`, `tsv/wasm/`에는 원본 TSV 88개를 보존한다.
  `representative/`는 각 조합의 최저 쪽, `detail/`은 보정 영역의 직접 판독 자료다.
- [Native source 일치](final-native-source-reconciliation.json),
  [fresh WASM 빌드·동기화·모듈 검사](final-wasm-build.json)를 연결한다.
  별도 코드 변경 없이 커밋한 source가 고정 Native 바이너리의 실제 컴파일 입력과 일치한다.
- [동일 회귀 실행 파일의 수정 전후](source-frame-before-after-proof.json):
  보정 전 6개 모두 배치 assertion 실패, 현재 Native 6개 모두 통과다.
  최소 표 상자만 따로 분리한 직전 후보도 선언 높이 345.2667px에 대한 337.7px 실패를 확인했다.
  초기 검사 코드의 의존성 컴파일 실패·수집 전 실행은 결함 재현으로 계산하지 않는다.
- [기존 정렬 검사](logs/issue4068-corrected-contract.log) 5개가 통과했다.
  잘못된 Top 오프셋 핀을 제거하고 원본 Center·내용 포함을 검사하며 실제 부모 clip 대조군을 유지한다.
- [PDF 설정·실제 WASM import](pdf-layout-ui-review.json)는 3종 × 2모드와 비 PDF 대조군이다.
  WASM 시각 검증의 PDF 입력 중간본은 실제 브라우저 import·내보내기에서 생성했다.
  이 중간본이나 임시 실행 스크립트는 회귀 fixture로 커밋하지 않는다.
- [CanvasKit 문서 사전 검사](sungeo-canvaskit-preflight.json)는 94쪽 중 54쪽에서
  작업량 50,000 상한과 첫 쪽의 shaping 제약을 기록한다. 전체 문서 UI의 Canvas2D 전환을
  CanvasKit 통과로 세지 않는다. calendar의 실제 CanvasKit UI와 별도로
  [34쪽 원본 선 연산 검사](sungeo-p34-lines-v2-review.json)는 원래 계층·clip을 보존한
  선 연산에 대해 두 CanvasKit 모드 × 1배·0.5배의 실제 Skia 굵기와 화소를 검사했다.
  점선 끝의 면적 coverage를 실선의 50% 대비로 판정하지 않고, 독립 Print로 검증한
  기존 Native 점선 검사의 최소 채널 232 기준을 함께 사용한다.
- [알려진 공식 사양 차이](official-spec-known-residuals.json)는 원 PR의 20·31쪽 차이가
  이번 보정 전후 동일한 PNG 해시임을 기록한다. 해당 차이를 해결했다고 주장하지 않는다.
- `logs/`는 실제 실행 결과다. 전체 `output/` 백업이 아니며, 임시 producer 실행 파일과
  font cache·개인 글꼴 프로그램은 포함하지 않는다.

- [기준 대비 십진수 비교](base-head-comparison.json): 각 backend의 41개 조합·767개 기준 쪽을
  모두 대조했다. [전체 페이지 차이](base-head-page-deltas.tsv)에서 양쪽 모두 하락 0쪽이다.
  기준의 공식 사양 83쪽 대 71쪽, cellnotes 12쪽 대 11쪽 불일치는 실패로 보존하고,
  대응 PDF가 있는 모든 쪽의 진단 TSV와 추가 페이지 해시를 별도로 기록했다.
  새로운 편집 우선 3모드는 구 버전에 없는 기능이므로 무리한 base 동등 비교를 하지 않았다.
- [입력 포함·해시 감사](tracked-input-audit.json)와 [글꼴·래스터 출처](font-and-raster-provenance.json):
  정식 입력·PDF 79개가 커밋에 있고 실제 파일 해시가 일치한다. 모든 비교의 실제 글꼴 공급 목록도 같다.
- [공식 사양 PDF 재사용 근거](official-spec-reference-reuse.json)는 새 MCP Print 실패 2건과
  같은 저장 빌드의 공식 PDF 재사용을 구분한다. 공식 원문 PDF와 사양·정오표는 수정하지 않았다.
- [정식 비교·overlay](review-panels.json)는 이미 산출한 실제 PNG와 독립 PDF를 원래 좌표로
  병치했다. `review/`와 `overlay/`에서 중첩 표·획·수식·여러 줄 그림 호스트·PDF 편집 모드를 확인한다.
- PDF 브라우저 영수증의 전체 본문 span 배열은 커밋에 보존하지 않는다. 각 페이지의 개수·
  해시·section 소유 및 실제 편집 본문 문단 수를 보존한 실행 증거이며, 테스트 fixture/golden이 아니다.
- [E2E 원장 검사](logs/final-e2e-manifest-after.log)는 172파일·172행이 일치한다.
  빠진 AI 제공자 행을 추가했으며 기존 암호 검사의 직접 실행 표기와 상위 검사의 간접 import 설명을
  구분했다. 실제 테스트 호출은 변경하지 않았다.

- 전체 Rust의 첫 최종 후보 실행은 보정이 필요해 중단했다.
  [실행 결과](rust-before-safety-corrections.json): 8,854 PASS·실제 assertion 실패 7개,
  SIGINT 3개·미실행 2,757개·기존 skip 45개다. 출력의 failed 10개에는 SIGINT 3개가 포함되며
  엔진 결함 10개로 세지 않는다. 전체 회귀 완료·제출 가능으로 판정하지 않는다.
- [독립 여백 편집 Print](civil-form-independent-margin-edit.json)는 기존 0 이동 기대값을
  원본 여백만큼 한 번 이동하는 관계로 정정할 근거다. 원본 민원 서식 전 2쪽 Native는
  보정 전·현재 모두 100%이며 실제 WASM 선행 검증은 별도 완료해야 한다.
