# 통합 후보의 검증 증거

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
