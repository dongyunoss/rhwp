# PR #8042 리뷰 — 묶기·풀기 뒤 필드와 회전 자식 보존

## 최종 판정

**머지 보류 — 자식 기하와 활성 셀 주소의 Native 집중 검증을 완료했으며, 새 후보의 fresh WASM·전쪽 시각·전체 통합 필수 검증을 계속한다.**

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

## 활성 셀 주소 보정의 현재 검증

같은 본문 문단의 컨트롤 번호 remap이 본문 필드의 지역 번호와 셀 필드의 본문 소유
번호를 구분하도록 보정했다. 셀 경로의 첫 컨트롤 번호만 옮기고 지역 필드 번호·나머지
경로는 보존한다. 공통 삽입 helper와 도형 제거·묶기·풀기가 같은 변환을 소비한다.
기존 #7595 → #7601 통합의 본문 필드 계약도 유지했다.

새 정식 검사 [active_field_group_root_owner.rs](../../../tests/cases/active_field_group_root_owner.rs)의
HWP/HWPX×묶기/풀기4개는 Native079에서4FAIL, Native081에서4PASS다. 양쪽 strict build는
성공했고 같은 활성 주소의 재설정·필드 안 입력·양쪽 저장 내용 보존을 검사한다.
[원본·독립 Print·양쪽 전쪽100% 선행 증거](../assets/active_field_group_owner_20261021/README.md)를
먼저 확보했다. 기존 필드·그룹 검사와 기대값을 삭제하거나 px 허용치를 넓히지 않았다.
현재 source는 이 검토 기록과 함께 일반 커밋으로 고정한다. Native081의 실제 제품/소스
바이트와 수정 전후 실행은 ignored `char-overlap-native-build-081-result.json`,
`emphasis-metric-current-failure-focus-1214.json`에 있다.

앞선 현재 API28조건은 고정 devel의 실제 Native/WASM 출력과 직접 비교를 완료했다.
각각 전28쪽100%, 쪽수1=1,0.1%p 이상 하락0쪽이다. 기준은 `2e3b4f880b`, 실제 기준 제품은
인쇄 경로 동등성을 입증한 `d7d4d2ea6e`이며 새2e3 빌드로 바꿔 설명하지 않는다.
`group-api-base-comparison-1199.json`에 원본·PDF·TSV SHA와 페이지별 비교를 보존했다.

위 결과는 이 보정의 Native 계약과 저장본 시각 선행 증거다. 새 fresh WASM에서의 API·
출력 및 통합 전체 회귀·전쪽·확대 판독을 완료하기 전에는 PR을 제출하거나 승인하지 않는다.

## 후속 실행과 원격 조건

[구현·검증 순서](pr_8042_review_impl.md)에 따라 남은 보정과 실제 실행을 완료한다.
최신 devel과 contributor head 확인·필요한 rebase 기능 보존 검증, 전체 로컬 gate를
마친 뒤 구체적인 통합 PR 결과에 대한 작업지시자 승인을 받는다. 통합 PR 번호용 별도
review 문서나 docs PR을 만들지 않는다. 오늘할일은 통합 PR 절차의 해당 시점에 기록한다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [tests/cases/active_field_group_root_owner.rs](../../../tests/cases/active_field_group_root_owner.rs): SHA-256 `bd095b50f8d4110db554606333b975aa6525b91249d40d1a3faf989aae3489f1`.
  - `group_before_active_cell_field_preserves_hwp_owner`: PASS, `logs/final-full-nextest-4052.log:8781` (`rhwp::regression_suite_018`).
  - `group_before_active_cell_field_preserves_hwpx_owner`: PASS, `logs/final-full-nextest-4052.log:8782` (`rhwp::regression_suite_018`).
  - `ungroup_before_active_cell_field_preserves_hwp_owner`: PASS, `logs/final-full-nextest-4052.log:8783` (`rhwp::regression_suite_018`).
  - `ungroup_before_active_cell_field_preserves_hwpx_owner`: PASS, `logs/final-full-nextest-4052.log:8784` (`rhwp::regression_suite_018`).
- [tests/cases/group_shapes_child_geometry.rs](../../../tests/cases/group_shapes_child_geometry.rs): SHA-256 `4974f0e3b4cb13396948c4fa66b3019f8666031bf2fc4d1b95ea88f763376dfb`.
  - `grouping_keeps_scaled_and_rotated_children_drawn_in_place`: PASS, `logs/final-full-nextest-4052.log:5792` (`rhwp::regression_suite_007`).
  - `grouping_keeps_stacking_order_regardless_of_target_order`: PASS, `logs/final-full-nextest-4052.log:5791` (`rhwp::regression_suite_007`).
  - `grouping_keeps_children_positioned_against_different_bases`: PASS, `logs/final-full-nextest-4052.log:5789` (`rhwp::regression_suite_007`).
- [tests/cases/ungroup_content_geometry.rs](../../../tests/cases/ungroup_content_geometry.rs): SHA-256 `ea0c6ce8a03aff92b6757d9652e63e7bd587ee7397c5b8964ce4012c9b7df807`.
  - `ungroup_plain_child_keeps_drawn_position_and_roundtrips`: PASS, `logs/final-full-nextest-4052.log:9052` (`rhwp::regression_suite_018`).
  - `ungroup_rotated_child_keeps_drawn_position_and_roundtrips`: PASS, `logs/final-full-nextest-4052.log:9053` (`rhwp::regression_suite_018`).
  - `ungroup_keeps_following_field_live_and_in_both_formats`: PASS, `logs/final-full-nextest-4052.log:9051` (`rhwp::regression_suite_018`).
  - `group_keeps_following_field_live_and_in_both_formats`: PASS, `logs/final-full-nextest-4052.log:9050` (`rhwp::regression_suite_018`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `9c83132affd66957e024fce12c4b63a51ab0d33c` | `e1df04c3bae432c02e96112fce99305b7e52d3af` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.

## Merge 후 contributor PR comment 계획

승인받은 통합 PR이 merge된 뒤 실제 merge SHA·CI URL·원 기여와 메인터너 보정 범위를
한국어 존댓말로 설명한다. Visual Sweep [정본](../../manual/verification/visual_sweep_guide.md)의
전쪽 지표와 직접 판독·남은 농도 차이를 연결한다. 대표 이미지는 같은 asset의
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/semanticist21_group_geometry_20261021/` 아래
Native/fresh review·overlay URL로 고정한다. 본문 파일을 `--body-file`로 전달하고 API로
게시 내용을 재확인한다. 이 계획은 사전 comment나 원 PR close의 실행 승인이 아니다.

## 보정 커밋의 Native·fresh 활성 셀 검증

`ae6d24e744`에서 셀 누름틀의 루트 소유 주소를 공통 보정했다. 새 관계형 정식 검사4개는
Native079에서4FAIL, Native081에서4PASS다. 실제 공개 WASM 편집 API도 fresh031에서
4FAIL, 같은 보정 소스의 최적화 fresh032에서4PASS로 바뀌었다. 필드 재활성화의
idempotence·표 소유 번호 이동·지역 주소 보존·입력과 두 저장 형식의 내용을 확인했다.
[검증 출처와 입력](../assets/active_field_group_owner_20261021/README.md)에 소스·제품 해시와
실제 결과를 연결했다. 현재 Rust lint/build4단계는 모두 성공했다. 새 전쪽 fresh와
전체 Rust 실패·차트 등 통합 필수 항목은 계속 검증하며, 최종 판정은 **머지 보류**다.
