# PR #7588 구현·검증 순서 — 묶음 해제의 누름틀과 회전 자식 재현

## 범위와 현재 상태

[개별 리뷰](pr_7588_review.md)의 원 head·기능 SHA·로컬 체리픽을 기준으로 한다.
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

## Native081까지 완료한 항목

활성 셀 주소의 공통 remap과 새 관계형 검사4개를 구현했다. strict before/after의
실제 결과는4FAIL→4PASS이며 형식별 입력·독립 Print와 양쪽 전쪽100% 증거를 보존했다.
기존 현재 API28조건의 고정 devel Native/WASM 직접 비교도 각각 전28쪽·하락0쪽이다.
현재 후보의 fresh·전체 통합 검증은 계속하며 이 기록을 전체 완료로 해석하지 않는다.

## 보정 커밋의 Native·fresh 활성 셀 검증

`ae6d24e744`에서 셀 누름틀의 루트 소유 주소를 공통 보정했다. 새 관계형 정식 검사4개는
Native079에서4FAIL, Native081에서4PASS다. 실제 공개 WASM 편집 API도 fresh031에서
4FAIL, 같은 보정 소스의 최적화 fresh032에서4PASS로 바뀌었다. 필드 재활성화의
idempotence·표 소유 번호 이동·지역 주소 보존·입력과 두 저장 형식의 내용을 확인했다.
[검증 출처와 입력](../assets/active_field_group_owner_20261021/README.md)에 소스·제품 해시와
실제 결과를 연결했다. 현재 Rust lint/build4단계는 모두 성공했다. 새 전쪽 fresh와
전체 Rust 실패·차트 등 통합 필수 항목은 계속 검증하며, 최종 판정은 **머지 보류**다.

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
| `428728a0154aebfa3cf80a8734109c9dcabedc82` | `6e89f8179d7fa3827da610090b5369abdb57bbe6` |
| `4fd99f4f3c1e173d6840c19a4ef0cab3f75cf388` | 원 devel merge; 기능 체리픽에서 제외 |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.

## 최종 로컬 판정과 원격 잔여 — 2026-10-26

[개별 최종 근거](pr_7588_review.md#최종-로컬-판정-근거--2026-10-26)에 따라
묶음 해제의 자식 배치·필드 내용 보존 범위를 `메인터너 보정 후 수용 가능`으로 판정했다.
검사·출력 source는 위와 같고 마지막 fetch에서 devel이 전진하지 않았음을 확인했다.
이전 단계의 미완료 문장은 당시 상태이며 현재 로컬 게이트의 미실행을 뜻하지 않는다.
통합 PR의 실제 code CI 성공 뒤 옵션 2의 self-review·오늘할일 후속 PR을 처리한다.
정확한 통합 head의 CI·merge·devel 포함 확인 전에는 원 PR을 close하지 않는다.
이 판정을 다른 원 PR의 미완료 항목이나 sample16·편람의 검증으로 확대하지 않는다.
