# PR #7589의 입력 상자 인쇄 대조 자료

- 원 PR의 `form_multiline_rendering::fixture`가 `samples/hwpx/form-01.hwpx`에 `FIRST\nSECOND`와 `MultiLine=1/0`을 넣어 저장한 HWPX입니다. 개행과 두 속성값은 다시 열어 확인합니다. 원본 양식의 스크립트는 변경하지 않았으며 실행하지 않습니다.
- 사용자가 두 HWPX를 내용을 바꾸지 않고 한컴의 파일 → 인쇄(Print) → PDF로 출력했습니다. `win10-ted`의 `C:\temp`에서 SCP로 원본 바이트를 가져왔습니다.
- 두 PDF의 실제 Creator는 `Hwp 2022 12.0.0.4605`, Producer는 `Hancom PDF 1.3.0.550`입니다. 파일명의 `2020`과 실제 제품 메타데이터를 구분합니다. 두 파일 모두 A4 1쪽입니다.
- 두 Print의 래스터는 동일하며 Edit에는 `FIRST`만 보입니다. 원문 HWPX의 `SECOND`를 삭제하지 않습니다. 저장값 보존과 표시 문자열을 구분합니다.
- Print에서 확인한 표시 계약의 독립 자료입니다. Native/fresh WASM 전쪽 TSV와 직접 판독은 해당 후보에서 별도로 완료한 뒤 검토 기록에 연결합니다.

| 파일 | 바이트 | SHA-256 |
| --- | ---: | --- |
| `pr7589-multiline-true.hwpx` | 11274 | `8b3d341410df07fc32f0909b5b84f274ec48e61bb70bf1bcc6050c2e62599457` |
| `pr7589-multiline-true-2020.pdf` | 22106 | `21482ec033b610614f675e32bb983cb3d75f9acbbf7ca7bbe00702f61f8f7868` |
| `pr7589-multiline-false.hwpx` | 11273 | `7bfb9ff9e8a1d047ddc2cb306da35b8e390779e1e56d347e18d67f9a9274e69e` |
| `pr7589-multiline-false-2020.pdf` | 22106 | `1a7972848d4e954d1cd4205f7bf16eb50676edbcc747524d7e1282bfb9b8071d` |
