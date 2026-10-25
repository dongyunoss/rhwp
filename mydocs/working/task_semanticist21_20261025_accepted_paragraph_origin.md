# 배제 영역이 확정한 첫 글줄 원점의 공통 소비

## 독립 근거와 선행 해결

Issue #6970의 원 PR #7118은 `aff426a709c472d812b199ef41a0968f6a96b900`,
누적 적용은 `dd25132d2deb4495a096902f9d48f136cfb7f5bd`다. 통합 #7197의
기록과 [원 검토 증적](../pr/archives/pr_7118_review_impl.md)을 대조했다.
익명화 원본 `tests/fixtures/issue_6970/synth_no_ls_square_wrap.hwp`와 독립
`pdf/synth_no_ls_square_wrap-2020.pdf`의 같은 전쪽 비교에서 고정 base와 동등한
Native194는 최저 raw 98.37264%, 현재 `ca4420fad6`의 Native240은 60.85378%다.
원본은 이번 #7445 미완료 제외 목록에 없다. 현재 회귀이므로 정확한 기존 검사를
제거하거나 진단 허용 오차를 넓히지 않는다.

## 실제 생산·소비 경로와 원인

`computed_host_frame::resolve → ParagraphFloatPlacement::ComputedEmptyHost →
TypesetState::resolve_computed_float_first_line → FloatPushedFirstLine.origin →
LayoutEngine::layout_column_item`을 추적했다. 마지막 소비는 일반 글자가 있는
문단 분기 안에만 있어 빈 문단의 장식 개체·줄 전진 경로를 누락했다.

표는 논리 호스트 글줄과 병렬로 점유하고, 뒤 글줄은 표의 배제 띠 아래에서
계속할 수 있다. 이 두 상태를 표 전체 높이의 무조건 전진으로 합치지 않는다.
해당 표의 측정은 두 버전 모두 259.12px이고 실제 표 하단도 446.3px다. 현재
paint는 표 뒤 커서를 호스트 줄 끝 203.9px로 잇고, 앞서 수용한 뒤 문단 원점을
누락해 본문이 표 안에 겹친다. 측정 높이 부족이라는 첫 가설은 이 관측으로 기각했다.

공통 항목 진입점에서 전체 문단과 첫 부분 문단의 확정 원점을 적용한다. 같은
문단의 뒤 개체나 이어지는 글줄은 이를 다시 적용하지 않는다. 배치 결과를 clamp하거나
원본 좌표로 맞추지 않고 수용한 원점을 직접 소비한다. 기존 분기 안의 중복 적용은
제거한다. 변경 뒤 기존 #6970·공통 배제·부동 표·빈 문단 대조군과 독립 PDF를
다시 확인하며, 전쪽 Native/fresh 90% 이상과 확대 판독 전 새 회귀를 추가하지 않는다.

작업 로그와 중간 트리는 ignored `output/pr-review/semanticist21-20261019/`의
`remaining-origin-3876/`, `logs/remaining-origin-*-diag-3878.log`,
`logs/remaining-origin-current-flow-3879.log`, `remaining-visual-3877-result.json`에 있다.
아직 보정 뒤 통과를 선언하지 않는다.

## #7158의 이동된 Square 띠와 접두 줄

기존 #7158 → PR #7163(`da7ec5c090`)의 줄 간격 계약을 유지한다. 기존 실패를
원본 여백 mirror 불일치로 설명한 가설은 실제 속성 setter·양쪽 mirror 변경에서도
같이 실패하므로 기각했다. 원본 HWPX의 75번 문단 표 아래 여백 숫자 하나만
+750/+1500 HWPUNIT 바꾼 대조군을 만들고, 바이트 변경 0인 정상 대조군도 확인했다.
변경 0은 원래 4줄, 두 여백 대조군은 Native240에서 3줄로 바뀌었다.

같은 편집본을 새 MCP 2020 Print로 각각 출력했다. 두 출력 모두 9쪽의 81번
문단을 4줄로 유지했다. PDF의 실제 글자 원점을 합친 기준선은 +750에서
422.9·452.9·482.8·512.9px, +1500에서 433.0·462.9·492.9·522.9px다.
앞 여백으로 원점이 이동해도 첫 줄의 Square 띠 소속은 사라지지 않는다.
+1500의 독립 Print는 전체 18쪽으로 바뀌므로 전체 쪽수를 과거 17쪽에 고정하지 않는다.

- +750 MCP job: `f9fb8704-89d4-4c83-b92c-f5d9bb5d6d34`, PDF SHA-256
  `eb6d71e41df2f419506c8b9a343654ce22b8014efef30e628b02f8cad4be9b7a`.
- +1500 MCP job: `df49aef9-ab48-4b4a-b792-ccccc2bb5da0`, PDF SHA-256
  `c6637def9fdb63b9f2cd7034ab6953413cd6ef2ffa6508aafc1727ff2d496d7c`.
- 실제 엔진: `11.0.0.9136`, 두 출력 모두 `pdf_print_method=0`,
  `hancom2020_pdf_driver_one_up`. 원본은 바꾸지 않았다.

`wrap_match::classify → wrap_flow의 prefix format → suffix_metrics →
commit_wrap_absorption/commit_wrap_tail → layout_wrap_around_paras/부분 문단`을
대조했다. 실제 활성 Square 띠와 저장 폭을 매칭한 뒤에도 prefix를 일반 format으로
구성해 그 근거를 누락했다. 일반 본문이 이미 쓰는 known-square-band format 경로를
prefix 측정에서도 사용한다. 올바른 기존 관계 검사는 유지하고 실제 보정 결과를
다시 확인한다. 독립 Print·현재 전쪽 TSV와 확대 판독을 완료하기 전 새 픽스처나
회귀를 등록하지 않는다. 임시 편집본·Print·로그는 `margin-7158-3880/`와
`mcp-7158-margin{750,1500}-3881/`에 있다.

## #8228 병합 후 리베이스와 재검증

사용자 지시에 따라 #8228의 머지 `3e02fe2749866a8453af3eb3901b9c1a674cccaf` 위로
기존 320개 커밋을 리베이스했다. 이전 HEAD는 `dd25d5e71dec421399629cbb60aec883ef468bdd`,
리베이스 직후 HEAD는 `b90b0741b69c2da4eddb03552d998848fbc5e251`이다.
미커밋 원점·Square 보정과 이 기록은 별도로 보존한 뒤 복원했다. 충돌은 두 동작을
함께 유지하고, 양립하지 않는 기존 프레임 판정은 사용자 지시에 따라 #8228을 우선했다.

전체 fallback 별칭·원 프로그램 굵기, 수학 기호·보충 문자 메트릭, 명시적 전체/부분
문단 소유, 재조판 셀 프레임 무효화, 완결 셀 슬롯과 첫 조각 바깥 여백을 함께 보존했다.
현재 글줄 메트릭 투영은 #8228의 구판 HWPX 여백 문맥을 유지한 단어 꼬리 투영으로 전달한다.
컴파일에서 드러난 다섯 호출 호환성 오류는 소유한 표·셀의 재조판 상태와 수용 구성을
실제 호출부에서 전달하는 방식으로 보정했다. 기대값·허용치는 변경하지 않았다.

삭제된 sample16·편람 검사는 복원하지 않았다. 실제 Rust 검증 worktree에서 파생 참조
1,770개를 확인했고, 삭제된 검사 52개로의 참조·누락 원본·sample16 이름 참조·삭제 파일
복원은 모두 0건이다. 남아 있는 정상 소스만 suite에 연결했다. 파생 파일은 PR에 포함하지 않는다.

Native243 빌드는 `target/pr-review`, `CARGO_BUILD_JOBS=8`과 `release-test`를 사용해
265.80초, exit0으로 종료했다. 빌드 중 소스 불변을 확인했고 실행 파일·라이브러리를 보존했다.
Studio 글꼴·AI·패널·장치 최소 선폭 집중 29건은 모두 통과했다. 새 잠금 파일 의존성을
설치한 뒤 TypeScript 검사도 exit0이다. 현재 전쪽 Native 비교와 Rust 집중 검사를 실행하며,
이전 fresh WASM068 결과를 리베이스 후 후보의 검증으로 재인용하지 않는다.

이관 #8230은 사용자 대기 요청 전 생성·#7445 하위 연결까지 된 상태다. 활성 검사 삭제나
추가 이관은 하지 않았으며 새 기준의 결과를 먼저 재검토한다. Native/fresh WASM의 새 기준
대조·쪽수·직접 판독과 전체 필수 게이트 전에는 PR 준비 완료로 판정하지 않는다.

로그와 입력·출력 해시는 ignored `output/pr-review/semanticist21-20261019/`의
`rebase-8228-3899/`, `rebase-native243-build-3904-result.json`,
`logs/rebase-8228-focused-rust-3906.log`, `logs/rebase-8228-visual-3907.log`에 기록한다.

### 리베이스 후 첫 실행의 실제 판정

- Rust 집중 검사: 144건 중 142 PASS·2 FAIL, exit100. 기존 ClickHere의 80행
  물리 분할 검사와 #7158의 이동 앵커 검사가 남았다. #7158 known-square-band 변경만으로
  실패를 해소하지 못했으며 해당 변경은 실험 중인 미커밋 변경으로 남긴다.
- Studio 전체 단위 검사: 2,597 PASS·실패 0, exit0.
- Native 12조건·26쪽: #1510 HWP/HWPX·#1535·#6133은 전쪽100%, #6970은
  3쪽 최저98.22185%. #8228의 field-reset·field-plain·saved-host·omitted-after·
  symbol-slot 두 Print 조건도 전쪽100%다.
- #8228의 `titled-host.hwpx`는1/3쪽43.94386%/40.24697%로 후퇴했다.
  2/4쪽은100%다. 새 PR·이관의 완료 조건을 충족하지 못했다.

동일 입력을 Native240·241·243으로 대조한 결과 T3/T4와 뒤 본문 원점은 같다.
따라서 이번 #6970의 공통 첫 줄 원점 적용이 이 차이의 원인이라는 가설은 기각했다.
#8157·#8159의 원 검토 기록과 독립 Print의 실제 선 좌표를 읽었다. K1의 T3 상단은
356.569 CSSpx인데 현재492.9px다. 앞 문단 오른쪽 T2와 뒤 문단 왼쪽 T3는 세로 구간이
겹쳐도 독립 Print에서 보존된다. 본문의 배제 띠와 수용된 흐름 표의 원점을 같은 조건으로
다시 미는 경로를 우선 대조한다. 현재144건에는 #7687 모듈 자체가 빠져 있었으므로
후속 집중 실행 계획에 기존 두 검사를 명시적으로 추가했다. 이 범위를 통과로 확대하지 않는다.
