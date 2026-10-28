# 통합 PR 본문 준비안

> 작성 중: 아래 검증 항목이 끝나기 전에는 PR 제출용 본문으로 확정하지 않는다.
> 원격 push와 PR 생성은 작업지시자의 승인을 받은 뒤 진행한다.

## 제목안

문서 조판·그림·PDF 처리와 성능 개선 통합

## 본문안

planet6897의 문서 조판, 그림 저장, PDF 가져오기 및 성능 개선 PR 20건을
최신 `devel` 위에 통합한다. 표·각주와 저장 프레임의 경계, 가는 획, 그림
호스트 재조판, PDF 본문 흐름 및 Studio 설정을 함께 검토하고 통합 과정에서
드러난 호환성 문제를 보정한다.

대상: #8115, #8128, #8133, #8142, #8184, #8202, #8205, #8206, #8209,
#8210, #8213, #8215, #8217, #8219, #8221, #8227, #8229, #8237, #8265,
#8283. 원 head와 최초 적용은 [기존 체리픽 원장](assets/review_integration_20261026/cherry_pick_ledger.json), 리베이스 후 현재 적용 SHA는 [대응 원장](assets/review_integration_20261026/cherry_pick_current_ledger_20261028.json)에 기록한다.
#8268과 #8161은 이 통합 대상이 아니다.

## 제출 전 검증 기록 — 미완료

- 최신 기준 `db5cd5bcc60a69e42f419f18a7f57a33b6fb7fdc`(#8328) 반영 후
  변경의 Native·fresh WASM 전쪽을 검사 중이다. 최종 결과와 근거 링크는
  검증 종료 후 기입한다. 페이지별 하락 기준은 0.1%p다.
- #8319 반영 범위 Native는 197개 입력·256쪽에서 실행 오류, 0.1%p 이상 하락과
  90% 미만이 없었다. [전쪽 비교 원장](assets/review_integration_20261026/validation/post-rebase-8328/incoming-native-comparison.json).
- #8328 기준 대비 fresh WASM은 55개 입력·329쪽에서 0.1%p 이상 하락이 없었다.
  기준에서도 90% 미만인 여섯 쪽은 기존 예외로 별도 기록한다.
  [전쪽 비교 원장](assets/review_integration_20261026/validation/post-rebase-8328/fixed-wasm-matrix.json).
- 표 뒤 문단 보정 당시 전체 Rust 회귀검사는 11,898개 통과·45개 skip으로 종료했다. 이후 원 PR 추가 체리픽과 회귀 assertion 변경이 있으므로 이 실행을 최종 head의 전체 회귀 결과로 표기하지 않는다. 작업지시자 지시에 따라 전체 회귀는 다시 실행하지 않으며 필요한 남은 개별 검사를 진행한다.
- #8184 후속 보정 전 측정에서 #8188 세 문서의 Native 및 fresh WASM 총 384쪽은 실제 이미지의 양방향
  반경 2px 미대응 픽셀이 0이다. [전쪽 픽셀 계약](assets/review_integration_20261026/validation/post-rebase-8328/sample16-fixed-wasm/combined-pixel-contract.json).
- #7723의 2010 저장본은 과거 PDF 기준과 새 MCP PDF 기준의 결과가 다르다.
  새 기준의 64쪽 중 2쪽만 100%여서 `Closes #7723`을 넣거나 #7445를 완료
  처리할 수 없다. 원인 조사와 확인 결과는 [통합 계획](review_integration_20261026_document_updates_plan.md)에 기록한다.
- 기존 WASM TSV 일부는 원래 실행 환경을 아직 재현하지 못한다. 동일 현재 환경의
  과거 바이너리와 후보 비교도 병행하며 전체 기준을 완료로 판정하지 않는다.
- 리베이스 시 두 #8149 후속 체리픽을 기존 묶음 뒤로 이동했다. 충돌은 없었고 리베이스 전후 최종 트리는 동일했다. [순서와 원 PR 대조](assets/review_integration_20261026/validation/post-rebase-8328/source-pr-paired-range-diff-review.md). 이후 문서만 추가 커밋했다.
- 새 #8184 보정 이후 36개 표 조각 입력의 Native 64쪽은 모두 raw 100%다. fresh WASM 빌드 및 해당 검증은 미완료다. 진행 기록은 최종 검증 후 정식 경로에 반영한다.
- 리뷰 문서의 기존 결과는 해당 시점의 기록이다. 최종 검증이 완료되면 20건의 리뷰 문서에 최신 결과와 해당 PR이 주장한 페이지의 근거를 반영한다.

제출 직전 최신 `upstream/devel`, 변경된 head와 필수 검사 결과를 다시 확인한다.
원격 CI와 머지는 아직 수행하지 않았다.
