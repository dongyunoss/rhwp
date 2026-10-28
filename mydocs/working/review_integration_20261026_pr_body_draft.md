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
- 후속 보정 후 관련 선택 회귀 24개가 모두 통과했다(필터에서 제외된 794개는 재실행하지 않음). [검사명·명령과 로그 해시](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-after-targeted-nextest.json).
- #8184 후속 보정 전 측정에서 #8188 세 문서의 Native 및 fresh WASM 총 384쪽은 실제 이미지의 양방향
  반경 2px 미대응 픽셀이 0이다. [전쪽 픽셀 계약](assets/review_integration_20261026/validation/post-rebase-8328/sample16-fixed-wasm/combined-pixel-contract.json).
- #7723의 2010 저장본은 과거 PDF 기준과 새 MCP PDF 기준의 결과가 다르다.
  새 기준의 64쪽 중 2쪽만 100%여서 `Closes #7723`을 넣거나 #7445를 완료
  처리할 수 없다. 원인 조사와 확인 결과는 [통합 계획](review_integration_20261026_document_updates_plan.md)에 기록한다.
- 기존 WASM TSV 일부는 원래 실행 환경을 아직 재현하지 못한다. 동일 현재 환경의
  과거 바이너리와 후보 비교도 병행하며 전체 기준을 완료로 판정하지 않는다.
- 리베이스 시 두 #8149 후속 체리픽을 기존 묶음 뒤로 이동했다. 충돌은 없었고 리베이스 전후 최종 트리는 동일했다. [순서와 원 PR 대조](assets/review_integration_20261026/validation/post-rebase-8328/source-pr-paired-range-diff-review.md). 이후 문서만 추가 커밋했다.
- 새 #8184 보정 이후 sample16 세 문서는 Native와 fresh WASM 각각 64쪽 모두 기존의 동일 PDF 래스터와 raw 100%다. 현재 방식으로 PDF를 다시 래스터화하면 일부 99.98%로 측정되어 두 결과를 별도 TSV로 보존한다. [입력·PDF·쪽별 해시 및 TSV](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-after-sample16/evidence.json).
- 새 #8184 보정 이후 36개 표 조각 입력의 Native·fresh WASM 각 64쪽은 모두 raw 100%다. [입력·PDF 및 쪽별 결과](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-float-fragments/evidence.json). 공식 사양서 71쪽도 같은 기존 PDF 래스터 기준 두 경로 모두 보정 전과 전쪽 수치가 같으며 0.1%p 이상 하락이 없다. [전쪽 비교](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-spec-all/evidence.json).
- 후속 체리픽 뒤 기존 통합 범위 67개 입력·956쪽은 Native·fresh WASM의 전쪽 렌더 PNG가 보정 전과 각각 동일하다. 같은 기존 PDF 래스터를 기준으로 두 경로 모두 0.1%p 이상 하락은 없고 90% 미만도 없다. [Native 66건](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-original-native-png-comparison.json), [WASM 66건](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-original-wasm-png-comparison.json), [공식 사양서 71쪽](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-spec-all/evidence.json). 동일 PNG에 관한 수치는 기존 래스터에서 추론한 것으로, 새 래스터의 직접 측정과 구분한다.
- 리뷰 문서의 기존 결과는 해당 시점의 기록이다. 최종 검증이 완료되면 20건의 리뷰 문서에 최신 결과와 해당 PR이 주장한 페이지의 근거를 반영한다.

제출 직전 최신 `upstream/devel`, 변경된 head와 필수 검사 결과를 다시 확인한다.
원격 CI와 머지는 아직 수행하지 않았다.

- #8319 보존 대상 197개 입력·256쪽은 Native 렌더 PNG가 후속 보정 전과 전쪽 동일했다. 기존 PDF 래스터 기준 최저 93.22230%, 0.1%p 이상 하락과 90% 미만이 없다. WASM은 진행 중이며 새 PDF 래스터의 직접 측정과 구분한다. [Native 전쪽 원장](assets/review_integration_20261026/validation/post-rebase-8328/pr8184-incoming-native-same-raster-comparison.json).

- 시스템 글꼴 디렉터리를 가리고 Fontconfig 검색 결과를 0개로 제한한 Linux 격리 환경에서 이 브랜치의 회귀 파일 26개를 선택 실행해 110개 모두 통과했다. 저장소에 포함된 글꼴은 검사 자원으로 사용할 수 있으며, Windows·macOS나 전체 회귀의 대리 결과는 아니다. [파일별 검사와 로그 해시](assets/review_integration_20261026/validation/post-rebase-8328/fontless-branch-regression.json).
