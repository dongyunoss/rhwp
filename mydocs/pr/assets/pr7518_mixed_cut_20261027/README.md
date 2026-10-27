# 기존 혼합 자식 컷 원본의 독립 Print 재확인

전체 회귀가 실패한 기존 두 `nested-mixed-cell.hwp`만 다시 확인했다. 새 HWP·합성 변형은
만들지 않았다. [입력·PDF SHA 및 실제 Print 출처](independent_print.json)를 따른다.

두 PDF는 Windows 한컴 2020 `11.0.0.9136`, PrintMethod0·1-up으로 출력했고 모두8쪽이다.
원본은 변경하지 않았으며 MCP의 `scripts_disabled` 전처리는 별도로 기록했다. 기존8쪽 PDF도
유지한다. 기준값 변경을 위해 출력 쪽수를 줄이거나 원본 내용을 고치지 않았다.

고정 base `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`는8쪽이고 Native276은6쪽이다.
새 Print와 base의 전쪽 Native 최저는99.96129%다. 현재 후보의 쪽수 불일치와4쪽 raw72.30%는
실제 회귀 근거이며 통과 증거가 아니다. 보정 후 전쪽 Native/fresh WASM·내용 소유·확대 판독은
별도로 완료해야 한다. 현재 진행 기록은 [공통 컷 보정](../../../working/task_semanticist21_20261027_mixed_cut_regression.md)에 있다.
