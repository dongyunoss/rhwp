# PR #8042 구현·검증 순서 — 묶기·풀기 뒤 필드와 회전 자식 보존

## 범위와 현재 상태

[개별 리뷰](pr_8042_review.md)의 원 head·기능 SHA·로컬 체리픽을 기준으로 한다.
45개 원 PR과 Draft10개를 포함한 통합 범위는 유지한다. 이 문서는 원 contributor
브랜치의 rewrite·직접 merge를 승인하지 않는다.

## 단계와 검증

1. reviewer 지정과 원 head·본문·변경 파일을 확인했다. 원 기능을 최신 고정 devel 위에
   체리픽했고 원 작성자의 기여와 공통 변환·직렬화·원점 보정을 구분했다.
2. 원 정식 helper에서 현재 API 입력을 생성했다. HWP/HWPX14쌍의 독립 Print와 양쪽
   전쪽 TSV, 대표 PNG를 확보했다. 관련 입력28조건의 raw는 양쪽 모두100%다.
3. 원 정식 검사를 현재 제품에서 실행했다. 활성 필드를 설정하지 않는 원 검사의
   범위도 확인했다. 본문 활성 주소4조건은 유지됐지만, 같은 문단의 표 셀 활성 주소는
   묶기·풀기에서 옛 표 번호에 남는 실제 반례를 확인했다.
4. [공통 주소 보정 계획](../../working/task_semanticist21_20261021_group_active_field_owner.md)에 따라
   셀 소유 번호의 생산·갱신·소비를 수정한다. 본문·셀·중첩 경로·다른 문단 대조와
   독립 Print·관련 전쪽 양쪽90% 이상·직접 판독을 완료한 뒤 정식 회귀를 추가한다.
5. 현재 API28조건의 고정 devel 직접 대조와 전체 통합의 Rust·lint·Native Skia·fresh
   WASM·기능 보존 검증을 완료한다. 필수 미해결 항목을 Draft나 CI 대기로 넘기지 않는다.
6. 제출 직전 devel과 원 head를 다시 확인하고 바뀌었다면 새 기준과 후보를 검증한다.
   원 PR별 archive와 증거를 같은 통합 PR에 넣고, 작업지시자의 승인 뒤 remote 단계로 간다.
7. 정확한 통합 head의 CI를 확인하고 승인 범위에 따라 merge한다. 실제 merge SHA를
   확인한 뒤 원 PR별 contributor comment·close·문서·이슈 후속 처리를 실행·재확인한다.

## 되돌리기와 보존

메인터너 보정은 별도 일반 커밋으로 고정하며 원 contributor 커밋을 amend·force-push하지
않는다. 보정이 실패하면 해당 보정 커밋의 원인을 수정한다. 원본·독립 Print·실패 증거를
유지하고 테스트·golden·허용치를 완화해 완료하지 않는다. 공유 `target/pr-review`와
다른 작업의 변경을 보존한다. 실행 로그·중간 JSON·다음 실행 인계는 ignored output에 둔다.
