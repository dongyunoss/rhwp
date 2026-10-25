# 확장한 인라인 호스트 프레임과 뒤 본문의 공간 검사

## 원 해결과 독립 근거

전체 Rust `3991`에서 기존 `issue_6448_tac_cell_uses_measured_band_and_moves_tail_to_page3`가
3쪽 대신 2쪽으로 실패했다. 원 이슈 #6448, PR #6472와 통합 PR #6480을 다시 대조했다.
실제 통합 `19b89d967b1505cd4bdcdbba7d1f1413f32a1505`는 넓은 declared-fit 특례를
복원하지 않고, 독립 Print의 HEAD / 표 / TAIL 3쪽 흐름을 보호한다. 이후 #8191의
`6097253468`은 셀 글자 높이 캐시의 정상/불일치 구분을 추가했다. 기대값을 완화하지 않는다.

같은 `samples/issue6448/tac_cell_leftover_fits.hwpx`와
`pdf/tac_cell_leftover_fits-2020.pdf`를 고정 base
`238de6de41a208648f4ade6b494fe433a81c1fce`와 후보
`20e49891adf822c320ee2ddb3a3f40ff3bdf6e0f`에서 대조했다. base Native247은
전 3쪽 최저 raw 99.04290%, fresh WASM070은 99.30014%다. 후보 Native248과
fresh WASM071은 모두 2쪽이어서 전쪽 TSV를 산출할 수 없다. 두 후보 제품은 삭제한
실행 불가능 분기 이전 runtime이므로 진단 제품으로 구분하며 최종 head 제품으로 표기하지 않는다.
입력/PDF SHA, 실제 명령과 로그는 ignored `output/pr-review/semanticist21-20261019/`의
`6448-base-candidate-3992-result.json`에 있다.

## 생산 결과와 실제 소비

`inline_box_rows::normalized_host → rendering의 inline_frames 투영 →
RenderNormalizationOverlay::register_rendered_text_partitions → TAC 표 배치 →
뒤 일반 문단 fit`을 추적했다. 원본 host 높이 1,000HU가 실제 표 높이 68,864HU로 바뀌고
뒤 문단의 원점도 같은 증가분을 소비한다. 이 자체는 실제 표 공간을 보존하는 보정이다.
그러나 기존의 짧은 host 판정은 투영된 큰 줄을 정상 저장 프레임으로 오인하여 거짓이 된다.
결과적으로 표 뒤의 strict fit이 사라지고, 높이 933.5733px인 본문에
947.52px를 수용한다. 원본의 높이 누락이라는 출처를 투영된 숫자만으로 다시 추정한 것이 원인이다.

원본 저장 줄이 단일 TAC 표의 선언 상자를 담지 못했고 실제 투영 줄이 커진 경우를
overlay에 원본·투영 포인터 모두로 보존한다. 문자 재줄바꿈 여부와 구분하며, 표의
셀 내용·물리 컷을 무조건 dirty로 만들지 않는다. 실제 TAC 배치가 이 출처를 소비해
바로 뒤 본문의 전체 줄 전진을 검사한다. 문서 번호·행 수·목표 화면 좌표의 분기는 추가하지 않는다.
정상 저장 줄과 확장이 없는 표는 기존 경로를 유지한다.

## 검증 순서

현재 실행 중인 `3991`의 검증 worktree와 source는 고정한 채 끝까지 실행한다.
수정 후 기존 #6448의 수정 전 FAIL / 수정 후 PASS, 정상 인라인·저장 줄·공동 앵커
대조군과 같은 입력의 전쪽 Native/fresh WASM·직접 판독을 확인한다. 새 입력이나 골든을
추가하지 않으며, 그 뒤 변경 범위의 필수 lint·전체 회귀를 완료한다. sample16·편람의
삭제된 원본·검사·파생 참조는 재생성하지 않는다. 아직 보정 성공이나 PR 준비 완료를 선언하지 않는다.
