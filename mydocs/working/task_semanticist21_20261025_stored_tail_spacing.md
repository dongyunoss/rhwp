# 저장 본문 끝 간격과 후속 표 원점 보정

## 독립 근거와 원인

원본 `samples/exam_science.hwp`의 독립 기준은
`pdf/issue7642-exam-print/exam-07-exam_science-hwp-print-2020.pdf` 전4쪽이다.
Native193(`ed427895aa80aff7045242a85cd1bbe8afd328ca`)과 고정 devel은 4쪽이나,
Native195(`f7c8f7001fd944481ded1646708aea89aaafb11a`)부터 5쪽이다.
원 해결·통합 후 저장 TAC 프레임 소유 보정의 전후를 동일 입력으로 추적했다.

문단42의 마지막 구성 높이는 18.2533333333px, 간격은 5.88px이다.
기준선 보정이 높이를 19HU 늘리고 간격을 같은 만큼 줄였으나, 후속 표의
원점 상속은 보정된 내용 하단에 원본 간격을 더해 19HU를 중복 계상했다.
문단43의 저장 원점935.7466666667px 대신936px를 강제한 차이가 다음 단으로
이어져 문항·보기 표 소속과 쪽수가 달라진다.

## 공통 결과의 생산과 소비

`FormattedParagraph.line_spacings` → `line_tail_spacing(end_line)` →
`ParagraphFragmentBounds`/`ParagraphFragment` → 실제 수용 조각의 기록 →
`previous_text_content_bottom` → `stored_tac::prepare`의 inherited origin으로 전달한다.
일반 전체 문단·분할 문단·host 본문·wrap tail은 같은 구성 간격을 사용한다.
내용 하단을 발행하지 않던 decoration/column 조각은 간격도 발행하지 않는다.
원본 저장 사다리는 소유·상대 간격의 유효성 확인에 유지한다. 문서별 상수나
clamp를 추가하지 않으며 후속 최대 원점 선택도 삭제하지 않는다.

원본과 구성 메트릭이 같을 때 `stored_content_bottom` 경로의 간격 차이는0이다.
구성 메트릭이 달라지면 같은 구성 결과의 내용 높이와 간격 차이를 함께 소비한다.
보충 폭의 수집·소비 eligibility 병합 보정5386de6e6a도 같은 후보에 포함한다.
실제 `supplemented` 경로는 준비된 전진폭을 소비하고 `em_exact`로 덮어쓰지 않는다.

## 검증 상태

수정 전 실제 trace와 원본·Print SHA, base/candidate 전쪽 결과는 ignored
`output/pr-review/semanticist21-20261019/science-flow-traces-3239-result.json` 및
`failed-input-comparison-3236-result.json`에 보존했다.
현재 변경의 Native208 빌드·실패 검사 재실행·전쪽 Native/fresh WASM 시각 검증은
아직 실행 전이다. 기존 기대값·baseline·쪽수 핀은 변경하지 않았다.
완료 판정은 새 source의 실제 실행 결과로 갱신한다.
