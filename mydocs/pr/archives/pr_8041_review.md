# PR #8041 리뷰 — 자식 도형의 실제 틀과 겹침 순서 보존

## 최종 판정

**메인터너 보정 후 수용 가능 — 묶음 자식의 위치·크기·회전·겹침 순서 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 아래 현재 증거와
실제 검사 실행을 대조했다. 통합 PR의 정확한 code head CI·merge·후속 처리는
앞으로 수행할 조건이며 로컬 통과로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

원 head `1e22bc7d12c8587d8fc2def693bffeb848c6ff4d`가 원격과 같음을 재확인했다. 검증 코드는
`61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 제출 전 fetch에서도 같은 base를
확인했으므로 추가 리베이스하지 않았다. 이후 문서·증거 커밋은 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 원 PR의 현재 검사 | 아래 모듈·함수별 실제 PASS와 로그 행을 확인했다. 소스 존재를 실행 결과로 대신하지 않았다. |
| 전쪽 Native/fresh WASM | 20조건·각 1쪽(전체 20쪽/backend)에서 최저 100.00000% / 100.00000%, 독립 Print 쪽수 일치·90% 미만/누락 0쪽 |
| 고정 devel 보존 | 동일 입력·Print·글꼴·backend의 페이지별 비교에서 0.1%p 초과 하락 0쪽 |
| 직접 판독 | 두 형식의 자식 frame·회전 중심·겹침 순서를 확인했다. 원본 33HU 선은 Print 0.36pt와 대응하므로 래스터 농도를 맞추려고 임의 1px로 두껍게 만들지 않았다. |
| 전체 로컬 게이트 | fmt·수집/단위 정책·Release build/lib·전체 nextest·Skia 3종·Native/WASM/workspace Clippy·workspace build·doctest 완료. 실제 종료 근거는 아래 공통 기록 참조 |
| 원격 절차 | 통합 PR의 정확한 code head CI·실제 merge·후속 comment는 앞으로 수행할 조건이다. 원 contributor head를 직접 승인·merge한 기록이 아니다. |

조건 번호 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028. [전쪽 TSV·원본/Print·producer·해시와 판독](../assets/source_pr_qualified_20261026/README.md)을
이 PR의 직접 근거로 연결한다. raw는 색상 경계 보정 전 2px 실루엣이며 RGB 완전 일치나
과거 별도 지표의 통과를 뜻하지 않는다. Native255·표준 release/wasm-opt를 마친 fresh073을
사용했다. 측정 뒤 코드 변경은 없으며, 과거 제품의 표는 당시 결과로 구분한다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/source_pr_qualified_20261026/png/native/013/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/native/013/overlay_001.png) |
| fresh WASM | [review](../assets/source_pr_qualified_20261026/png/fresh-wasm/013/review_001.png) | [overlay](../assets/source_pr_qualified_20261026/png/fresh-wasm/013/overlay_001.png) |

Merge 후에는 이 대표 파일을
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/source_pr_qualified_20261026/`
위 표의 실제 상대 경로로 고정해 contributor comment에 두 이미지를 함께 표시한다.
같은 merge의 devel 포함·asset 존재·CI 결과를 먼저 확인하고 한국어 존댓말의
`--body-file` 본문을 게시한 뒤 API에서 한글·링크·실제 Markdown을 다시 확인한다.


## 접수 정보

이하 최초 접수와 중간 검토의 제품·기준 SHA 및 당시 진행 상태를 보존한다.
현재 판정은 위 「최종 로컬 판정 근거」와 아래 「통합 후보의 실제 검사 실행」이 정본이다.
과거의 진행 중·미완료 표현을 현재 로컬 게이트 상태로 사용하지 않는다.

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#8041](https://github.com/edwardkim/rhwp/pull/8041) / semanticist21 / devel |
| 원 head | `1e22bc7d12c8587d8fc2def693bffeb848c6ff4d` |
| 기능 커밋 → 로컬 체리픽 | `1e22bc7d12c8587d8fc2def693bffeb848c6ff4d` → `530bf8800c` |
| 통합 후보 | `review/semanticist21-20261019`, 제품 `6fc2b3490d`, 검토 기록 직전 HEAD `a524b12732` |
| 고정 devel | `2e3b4f880bd11d35398e6ea0100846611f13a72d` |
| reviewer | jangster77의 reviewer 지정 API 응답을 보존했고 상세 검토 전 확인했다 |
| 작성 시점 참고 | 1159 재조회에서 OPEN, Draft=false; 제출 전 최신 head·CI·mergeability 재확인 |
| 관련 이슈 | 원 PR의 재현·구현 범위로 검토한다. 확인하지 않은 이슈 close를 약속하지 않는다 |

## 변경과 원 구현 검토

묶을 때 현재/원래 크기·회전·뒤집기를 저장 행렬로 표현하고 자식 z-order·문서 순서를 유지한다. 서로 다른 배치 기준의 실제 쪽 원점을 같은 그룹 틀로 모으며, 중첩 자식과 회전 타원을 실제 변환한 윤곽으로 그린다.

초기 004 API 저장본의 잘못된 그룹 상자를 XML로만 고친 진단을 원본 개선으로 주장하지 않았다. 메인터너 보정 `251568b063`은 실제 paint 행렬로 변환한 자식 윤곽의 합집합을 그룹 상자로 사용한다. 후속 공통 저장 `daf1f4fa36`, 반전 `4bdec8df51`·`4a089bc2c9`를 포함한 현재 API가 생성한 새 저장본으로 독립 Print를 확인했다.

원 PR 본문·변경 파일·커밋은 ignored `pr8041-current-review-1159.json`과 원 history에
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
- `group_shapes_child_geometry` 3 PASS. 정식 검사 파일은 원 head와 바이트가 같다(SHA-256 `4974f0e3b4cb13396948c4fa66b3019f8666031bf2fc4d1b95ea88f763376dfb`).
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

[구현·검증 순서](pr_8041_review_impl.md)에 따라 남은 보정과 실제 실행을 완료한다.
최신 devel과 contributor head 확인·필요한 rebase 기능 보존 검증, 전체 로컬 gate를
마친 뒤 구체적인 통합 PR 결과에 대한 작업지시자 승인을 받는다. 통합 PR 번호용 별도
review 문서나 docs PR을 만들지 않는다. 오늘할일은 통합 PR 절차의 해당 시점에 기록한다.

## 현재 후보의 실제 검사 실행 — 2026-10-26

현재 통합 head는 `d6ea10837218e7e490b166ed76af221490b61ba2`, base는 `4e690c930105fa09fef484787ed0a2e5e182d444`다. 생산 코드는
`a90a2057a7e967a617c043c62e71f4fab26186df`와 동일하다. 문서 전용 리베이스의 코드·검사 tree
동일성을 확인했다. 전체 nextest 11,716 PASS·0 FAIL, Skia lib 4,040 PASS·0 FAIL과 추가
Native 3종·문서 테스트·Studio 전체 2,597 PASS의 terminal 결과를 확인했다. 삭제·분리 뒤
이동 계약 19개는 4207에서 별도로 다시 실행했고, 4205의 전체 lint도 통과했다.

아래는 현재 source와 실제 PASS 행의 대응이다. 계약 검사 통과를 별도 입력의 독립 Print 일치나
원격 CI·merge 완료로 확대하지 않는다. 원 PR 재현의 남은 시각 확인과 최종 판정은 별도다.

- [tests/cases/active_field_group_root_owner.rs](../../../tests/cases/active_field_group_root_owner.rs): SHA-256 `bd095b50f8d4110db554606333b975aa6525b91249d40d1a3faf989aae3489f1`.
  - `group_before_active_cell_field_preserves_hwp_owner`: PASS, `logs/current-full-nextest-4184.log:9318` (`rhwp::regression_suite_020`).
  - `group_before_active_cell_field_preserves_hwpx_owner`: PASS, `logs/current-full-nextest-4184.log:9319` (`rhwp::regression_suite_020`).
  - `ungroup_before_active_cell_field_preserves_hwp_owner`: PASS, `logs/current-full-nextest-4184.log:9320` (`rhwp::regression_suite_020`).
  - `ungroup_before_active_cell_field_preserves_hwpx_owner`: PASS, `logs/current-full-nextest-4184.log:9321` (`rhwp::regression_suite_020`).
- [tests/cases/group_shapes_child_geometry.rs](../../../tests/cases/group_shapes_child_geometry.rs): SHA-256 `4974f0e3b4cb13396948c4fa66b3019f8666031bf2fc4d1b95ea88f763376dfb`.
  - `grouping_keeps_children_positioned_against_different_bases`: PASS, `logs/current-full-nextest-4184.log:4310` (`rhwp::regression_suite_002`).
  - `grouping_keeps_scaled_and_rotated_children_drawn_in_place`: PASS, `logs/current-full-nextest-4184.log:4311` (`rhwp::regression_suite_002`).
  - `grouping_keeps_stacking_order_regardless_of_target_order`: PASS, `logs/current-full-nextest-4184.log:4312` (`rhwp::regression_suite_002`).

로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.

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
