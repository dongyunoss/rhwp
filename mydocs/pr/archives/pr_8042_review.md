# PR #8042 리뷰 — 묶기·풀기 뒤 필드와 회전 자식 보존

## 최종 판정

**머지 보류 — 표 셀 활성 누름틀의 본문 소유 번호가 갱신되지 않는 실제 결함을 확인했다. 공통 주소 보정, 셀·중첩·다른 문단 대조, 독립 Print·양쪽 시각 검증과 통합 필수 gate를 완료해야 한다.**

이 기록은 로컬 체리픽 통합 검토다. 원 contributor PR을 직접 merge하지 않는다.
원격 push·PR 생성·review/comment·merge는 작업지시자의 구체적인 승인 뒤에 수행한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#8042](https://github.com/edwardkim/rhwp/pull/8042) / semanticist21 / devel |
| 원 head | `9c83132affd66957e024fce12c4b63a51ab0d33c` |
| 기능 커밋 → 로컬 체리픽 | `9c83132affd66957e024fce12c4b63a51ab0d33c` → `1f821d1242` |
| 통합 후보 | `review/semanticist21-20261019`, 제품 `6fc2b3490d`, 검토 기록 직전 HEAD `a524b12732` |
| 고정 devel | `2e3b4f880bd11d35398e6ea0100846611f13a72d` |
| reviewer | jangster77의 reviewer 지정 API 응답을 보존했고 상세 검토 전 확인했다 |
| 작성 시점 참고 | 1159 재조회에서 OPEN, Draft=false; 제출 전 최신 head·CI·mergeability 재확인 |
| 관련 이슈 | 원 PR의 재현·구현 범위로 검토한다. 확인하지 않은 이슈 close를 약속하지 않는다 |

## 변경과 원 구현 검토

묶기·풀기의 컨트롤 배열 변경에 맞춰 field_ranges와 본문 활성 필드 번호를 갱신한다. 풀 때 자식의 실제 행렬이 그리는 크기·중심과 그룹 정렬을 사용하며 두 저장 형식에서 같은 기하를 유지한다.

원 4검사는 필드 목록·범위·저장 보존을 검사하지만 활성 필드를 설정하지 않는다. 별도 공개 API 진단 1178에서 본문 필드의 묶기/풀기×HWP/HWPX 4조건은 같은 활성 주소를 유지했다. 그러나 1179의 표 셀 반례는 묶기 때 표 번호 4→3, 풀기 때 3→4로 바뀌어도 활성 경로가 옛 번호에 남았다. 본문만 갱신하는 조건으로는 셀 소유 보존을 입증할 수 없다.

원 PR 본문·변경 파일·커밋은 ignored `pr8042-current-review-1159.json`과 원 history에
보존했다. 원 변경과 메인터너 통합 보정을 구분하며, helper 이름이나 테스트 통과만으로
실제 출력과 활성 필드의 의미를 모두 입증했다고 판정하지 않는다.

## 검증 입력과 결과

최종 입력·독립 PDF·전체 Native/fresh WASM TSV·대표 review/overlay는
[그룹 API 증거](../assets/semanticist21_group_geometry_20261021/README.md) 및
[출처 원문](../assets/semanticist21_group_geometry_20261021/provenance.json)에 연결했다.
원 helper를 현재 제품에 연결해 만든 HWP/HWPX 14쌍이다. HWPX는 기존 Print 입력과
바이트가 같아 SHA가 맞는 영수증을 재사용했고 HWP는 별도 실제 MCP engine 2020으로
Print했다. 실제 한컴 빌드는 11.0.0.9136, PrintMethod0·one-up이다.

- 28조건 모두 Native079/fresh WASM031 raw100%, 독립 PDF 쪽수1=1이다.
- `ungroup_content_geometry` 4 PASS. 정식 파일은 원 head와 바이트가 같다(SHA-256 `ea0c6ce8a03aff92b6757d9652e63e7bd587ee7397c5b8964ce4012c9b7df807`). 본문 활성 주소 진단은 4 PASS지만 표 셀의 활성 주소 보존은 미충족이다.
- Native079/fresh031의 기록된 제품 소스 183개는 검토 HEAD `a524b12732`와 바이트가
  같다. 실행 원문은 ignored `native079-wasm031-committed-source-equivalence-1183.json`에 있다.
- 같은 제품의 표·양식·선·언어 원본 53조건526쪽은 양쪽 전쪽90% 이상이며, 고정 devel
  대비0.1%p 이상 하락0쪽이다. 이 범위는 현재 API28조건의 devel 출력 대조를 대신하지 않는다.
- Rust lint/build4항목은 `a524b12732`에서 성공했다. 전체 Rust1180은 같은 Rust 소스를
  고정해 실행 중이다. 이전 focused 통과를 전체 회귀 완료로 표현하지 않는다.

현재 API28조건의 고정 devel 직접 대조, 남은 활성 셀 소유 반례와 전체 통합 필수
검증은 끝까지 수행한다. 필수 자료가 부족하다는 기록만 남겨 PR을 제출하지 않는다.

## 조판 원칙과 직접 판독

| 항목 | 판정과 근거 |
| --- | --- |
| 공통 측정·배치 | 자식의 실제 paint 행렬과 저장 행렬을 대조하고 변환한 윤곽의 합집합으로 그룹 틀을 정했다. 문서명·좌표 clamp를 추가하지 않았다 |
| 저장 조판 정보 | 현재 공개 API가 생성한 HWP/HWPX를 별도 Print와 대조했다. 수동 XML 진단을 실제 원본 완료 증거로 대신하지 않았다 |
| 구현 주장·검사 | 필드 목록·범위와 활성 주소 검사를 구분했다. 본문 정상 대조와 표 셀 반례의 결과를 [분석 기록](../../working/task_semanticist21_20261021_group_active_field_owner.md)에 연결했다 |
| 분할·이어받기 | 이 단계의 그룹 편집은 표 분할 컷·예약 높이를 바꾸지 않는다. 통합의 별도 표 보정은 해당 문서·검사에서 검증한다 |
| 회귀 추가 조건 | 현재 단계는 원 검사와 기대값을 유지했다. 새 셀 주소 검사는 독립 Print·관련 전쪽 양쪽90%·직접 판독 뒤 새 이름으로 추가한다 |
| 입력 커밋 | 현재28입력·PDF·원문 TSV·대표 PNG를 이 통합 브랜치의 같은 증거 묶음으로 보존한다. 실행 로그·중간 JSON은 ignored output에 둔다 |

30도 자식 묶음(HWPX), 크기·회전 자식 묶음(HWP)의 같은 쪽을 Native와 fresh의
review/standalone overlay로 직접 열어 위치·회전·외곽·겹침 순서를 확인했다. 검은
테두리의 약한 농도도 조사했다. 실제 stroke0.48 CSS px와 PDF0.36pt는 같은 물리 굵기이며
Native/Canvas에 선이 존재한다. 저해상도 raster 농도 차이는 남는다. 600DPI 추가 진단을
정식96DPI 점수나 허용치 변경에 사용하지 않았다. raw100%를 RGB 동일로 설명하지 않는다.

## 후속 실행과 원격 조건

[구현·검증 순서](pr_8042_review_impl.md)에 따라 남은 보정과 실제 실행을 완료한다.
최신 devel과 contributor head 확인·필요한 rebase 기능 보존 검증, 전체 로컬 gate를
마친 뒤 구체적인 통합 PR 결과에 대한 작업지시자 승인을 받는다. 통합 PR 번호용 별도
review 문서나 docs PR을 만들지 않는다. 오늘할일은 통합 PR 절차의 해당 시점에 기록한다.

## Merge 후 contributor PR comment 계획

승인받은 통합 PR이 merge된 뒤 실제 merge SHA·CI URL·원 기여와 메인터너 보정 범위를
한국어 존댓말로 설명한다. Visual Sweep [정본](../../manual/verification/visual_sweep_guide.md)의
전쪽 지표와 직접 판독·남은 농도 차이를 연결한다. 대표 이미지는 같은 asset의
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/semanticist21_group_geometry_20261021/` 아래
Native/fresh review·overlay URL로 고정한다. 본문 파일을 `--body-file`로 전달하고 API로
게시 내용을 재확인한다. 이 계획은 사전 comment나 원 PR close의 실행 승인이 아니다.
