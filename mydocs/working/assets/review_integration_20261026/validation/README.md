# 리베이스 전 완료한 통합 후보의 검증 증거

검증 종료 후 devel에 #8116이 반영됐다. 아래는 0bb 기준에서 완료한 결과이며, 새 base 리베이스·양쪽 기능 보존은 진행 중이다.

생산 source는 `f272b81c28b2525c5f6e520ef1e5d499b16672d3`, 전체 Rust 검증 head는
`1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559`, 기준 devel은 `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`다.
소스가 같은 증거 커밋·빌드 head를 분리했으며 아래 결과는 최종 보정 뒤 실제 실행한 것이다.

| 경로 | 입력 조합 | 전쪽 비교 횟수 | 최저 raw | 90% 미만 |
| --- | ---: | ---: | ---: | ---: |
| native | 63 | 941 | 90.81277% | 0 |
| wasm | 63 | 941 | 90.94873% | 0 |

쪽수는 문서별 전체 페이지의 합산 비교 횟수이며 하나의 문서 쪽수가 아니다.
57개 HWP/HWPX 조합과 PDF 세 종류의 위치 고정·편집 우선 6개 조합을 포함한다.

- [전쪽 입력·명령·해시·TSV·대표 PNG](visual-validation.json), [실제 글꼴·래스터 출처](font-and-raster-provenance.json)를 연결한다.
- `tsv/native/`와 `tsv/wasm/`는 각각 원본 TSV 63개다. `representative/`는 각 입력의 최저 쪽이며, `detail/`은 보정 영역이다.
- [기준 대비 비교](base-head-comparison.json)와 [쪽별 십진수 차이](base-head-page-deltas.tsv)는 Native·WASM 각각 60개 조합·895개 기준 쪽이다.
  native: 양의 하락 0쪽, 2%p 이상 0쪽, 최대 하락 0.00000%p.
  wasm: 양의 하락 0쪽, 2%p 이상 0쪽, 최대 하락 0.00000%p.
  같은 PDF·실제 글꼴 목록의 120쌍을 검증했다. 기준의 공식 사양 83쪽/71쪽과 cellnotes 12쪽/11쪽 오류는 실패로 보존하고 대응 쪽의 진단·추가 쪽 해시를 기록했다.
  새 editable 3모드는 구 버전에 없어 독립 PDF·실제 본문 소유 검사로 별도 검증한다.
- [입력 커밋 감사](tracked-input-audit.json): 정식 경로 117개의 파일 바이트와 commit blob이 일치한다. PDF import 중간본 12개는 실제 CLI/브라우저 생성 결과로, 원본 PDF·생성 명령·SHA를 보존하고 fixture로 커밋하지 않는다.
- [전체 Rust 실행 원장](final-rust-validation.json)은 fmt·세 Clippy·workspace build·고정 base 정책·전체 nextest·Native Skia lib 및 네 Native 모듈의 실제 종료 결과다.
- [최종 전체 검사에 포함된 관련 17개](final-contract-subset.json)는 그림 fit 6개·no-LineSeg 4개·source-frame 소유 7개다. 별도 실행 횟수로 더하지 않는다.
- [최적화 WASM 빌드](final-wasm-build.json), [Native source 일치](v14-native-source-reconciliation.json), [생산 소스 동일성](production-source-continuity.json)으로 Native·WASM·회귀 head를 연결한다. 실제 pkg/public JS·WASM 해시와 WebAssembly.compile을 확인했다.
- [Studio·Python·정책 검사](non-rust-validation.json)는 같은 frontend source의 Studio 2,607 PASS·기존 skip 2개와 완료된 focused·계약 검사를 구분한다. 새 최적화 WASM의 [브라우저 실행 원장](browser-validation.json)에 AI 제공자 E2E·PDF 설정과 가져오기·제한된 CanvasKit 검사를 기록했다. AI는 모의 API이며 실제 키를 사용하지 않았다.
- [실제 PDF 화면 영수증](pdf-layout-ui-review.json): 2010 16쪽·2020 15쪽·mac 15쪽의 두 모드와 비 PDF 대조군이다. 편집 우선 본문 문단 소유를 확인했다. 대형 span 배열은 개수·해시만 보존하며 golden으로 사용하지 않는다.
- [CanvasKit 제한된 선 연산](sungeo-p34-lines-v2-review.json)은 원본 sungeo 34쪽의 계층·clip을 유지한 선/배경 연산이다. 두 모드 × 두 배율에서 실제 Skia 획 최소 1 장치 화소와 원본 네 점선의 세 지점 화소를 검사했다. [문서 사전 검사](sungeo-canvaskit-preflight.json)의 전체 UI Canvas2D 전환과 구분하며 전체 문서 CanvasKit 통과로 세지 않는다.
- [정식 비교·overlay](review-panels.json)의 `review/`·`overlay/`는 실제 최신 PNG와 독립 PDF의 병치다. 수식·획·중첩 표·여러 줄 그림·중국어 글줄·온새미로 42쪽·재정동향·PDF 편집 모드를 포함한다.
- [Native 확대 판독](v14-native-reading-notes.json), [현재 WASM 확대 판독](v14-wasm-reading-notes.json), [Native 판독 연속성](v14-direct-reading-continuity.json), [WASM 판독 연속성](v14-wasm-direct-reading-continuity.json)을 연결한다. 기존 글꼴·작은 표식·도식 빗금 차이를 이번 프레임 보정의 해결로 주장하지 않는다.
- [공식 사양 20·31쪽 잔여 차이](official-spec-known-residuals.json), [실패한 새 Print와 공식 PDF 재사용](official-spec-reference-reuse.json)를 분리한다. 공식 PDF·사양·정오표는 수정하지 않았다.
- [독립 여백 편집 Print](civil-form-independent-margin-edit.json)는 같은 원본의 top margin만 바꾼 출력으로 기존 0 이동 기대값을 정정한 근거다.
- [최초 source-frame 수정 전후](source-frame-before-after-proof.json), [7번째 회귀·정렬 보정](v12-focused-proofs.json)은 실제 assertion 실패와 성공을 구분한다. 컴파일 실패나 0개 수집을 결함 재현으로 세지 않는다.

## 실패·중단 이력

- [초기 전체 실행](rust-before-safety-corrections.json): 8,854 PASS·실제 assertion 실패 7개·SIGINT 3개·미실행 2,757개·기존 skip 45개. 전체 통과로 세지 않는다.
- [그림 fit 보정 전 완료된 전체 실행](rust-before-inline-picture-fit-correction.json): 11,619 PASS·재정동향 실제 실패 3개·기존 skip 45개. 이후 공통 물리 행 상자 기반 수용으로 보정했다.
- [v13 WASM 중단](v13-wasm-corrective-stop.json)은 정적 검토에서 불필요한 부동 도형 점유 합산을 제거하기 위해 최적화를 중단한 기록이다. 최종 v14 빌드·Native·WASM·Rust 결과와 혼동하지 않는다.
- v12·v13 및 이전 3ae/44개·813쪽 결과는 역사적 단계 증거이며 최종 63개 결과를 대신하지 않는다.

원 PR 20건별 archive 검토와 적용 SHA는 [통합 계획](../../../review_integration_20261026_document_updates_plan.md)에 연결한다. 로컬 완료와 통합 PR의 원격 CI·머지를 구분한다. 임시 실행 스크립트·생성 fixture/golden·개인 글꼴 파일은 포함하지 않는다. [원시 로그 정리 감사](temporary-log-audit.json)는 최종 트리에서 제거한 임시 로그의 요약·해시와 실제 종료 결과를 보존한다.
