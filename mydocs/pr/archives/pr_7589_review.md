---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7589_review.md
last_verified: 2026-10-21
---

# PR #7589 검토 — 입력 상자의 저장 개행과 Print 표시

## 최종 판정

**머지 보류 — 원 head의 출력 차이 기대값은 독립 한컴 Print와 다르며, 보정한 통합 후보의
최종 로컬 검증을 진행 중이다.** 이 입력의 표시 계약과 원문 보존은 독립 자료와 기존 집중
검사로 확인했지만, 최신 통합 head의 필수 전체 회귀·lint·Native/fresh WASM·devel 대조가
끝나기 전에는 수용 완료로 판정하지 않는다. 원격 review·push·PR 생성·merge는 하지 않았다.

## 접수 정보와 적용 계보

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7589](https://github.com/edwardkim/rhwp/pull/7589), semanticist21, devel 대상 Draft |
| 2026-10-21 조회 head | `300d1af385064eb32bb85c94f3a40ab7213d2d2a` |
| 기능 commit | `b03f3072bf81ffa83ee43e6cc77cf8c7f2e10aef` |
| 로컬 cherry-pick | `cb0118ccf1`; head의 나머지 commit은 devel merge이므로 체리픽하지 않음 |
| reviewer | jangster77; 요청 API 영수증으로 지정 확인 |
| 메인터너 보정 | `7ee2327f7c87a5d1263b480b3ad27025874bc96a`, `612c09bae4453bc3747816c51f4c4a9cfff3581d` |
| 현재 통합 branch | `review/semanticist21-20261019` |

최신 PR 본문·commit·파일 목록은 ignored output의
`pr7589-current-review-1106.json`, reviewer 영수증은 `pr-7589-reviewers.json`이다.
원 PR의 본문에 적힌 예전 test head와 실제 조회한 merge head를 구분했다.

## 구현 주장과 독립 기대값

원 PR은 같은 `FIRST\nSECOND`에서 MultiLine=1/0의 SVG가 달라야 한다고 주장한다.
파서·직렬화가 개행과 속성을 보존하는지 검사한 점은 유지한다. 그러나 그 뒤의
`assert_ne!`는 이 입력의 한컴 인쇄 동작을 입증하지 않는다.

사용자가 두 HWPX를 수정 없이 한컴의 파일 → 인쇄 → PDF로 출력했고,
win10-ted의 `C:\temp`에서 원본 바이트를 가져왔다. 두 PDF는 각각 A4 한 쪽이며
실제 Creator는 `Hwp 2022 12.0.0.4605`다. 파일명의2020과 실제 제품을 구분한다.
두 Print의 래스터는 동일하고 입력 상자에는 `FIRST`만 표시된다.

공통 `FormObjectNode`의 표시 문자열에서 첫 저장 줄을 선택하되 모델 원문과
MultiLine 속성은 보존하도록 보정했다. SVG·Canvas·Native가 같은 표시 결과를
소비하며 특정 문서나 좌표에 따른 예외는 없다. 모든 입력 상자의 편집 중 화면까지
MultiLine이 무의미하다고 일반화하지 않는다.

## 입력·시각 증거와 회귀 검사

네 입력/PDF와 각 SHA·바이트 수·생산자 해시·원문 TSV·review/overlay는
[입력 상자 Print 증거](../assets/pr_7589_form_multiline/README.md)에 있다.
기존의 동일한 커밋 파일을 재사용하며 별도 사본을 추가하지 않았다.

증적 source `fca7ff627b7d05d6edb8dade112ead8b43c2db6f`에서 두 입력 모두
Native/fresh WASM raw100%, 전체 쪽수1=1이고 같은 영역의 직접 판독도 완료했다.
이 과거 생산 증거를 최신 통합 head의 재검증 결과로 바꾸어 쓰지 않는다.

`tests/cases/form_multiline_rendering.rs`는 원문·속성 보존과 FIRST 표시,
SECOND 미표시, 두 결과의 동일성을 함께 검사한다. 동일성만 검사하면 이전의 잘못된
연결 출력도 통과하므로 단독 검출 근거로 쓰지 않는다. 절대 px 위치는 고정하지 않는다.

`775de4c16fa2cc5e36fc588e7f500625e7a4939c`의 전체 Rust1085에서는 이 파일의
두 검사 모두 PASS했다. 전체 실행은 별도62실패가 있어 통과로 보고하지 않는다.
이후 공통 줄 투영 보정 `96038d8647`의 실제 소스 검증과 최종 시각 대조를 계속한다.

## 조판 원칙과 남은 필수 검증

| 항목 | 판정·근거 |
| --- | --- |
| 원 주장·기대값 대조 | 원 SVG 차이 기대값은 독립 Print와 불일치; 보정 검사에서 표시와 저장을 구분 |
| 측정·표시 소비 | 공통 FormObjectNode의 표시 문자열을 각 backend가 소비 |
| 일반성·반례 | MultiLine=0/1, 저장 개행 있음/없음; 원문은 보존 |
| 분할·이어받기 | 이 PR 자체는 비해당; 통합 후보의 공통 줄 변경은 별도 필수 검증 |
| 시각 선행 조건 | 기존 증적 source에서 양쪽100%·직접 판독 확인; 최신 head의 최종 검증은 진행 중 |
| 최종 통합 gate | 미완료; 전체 필수 검증과 최신 devel 대비 하락 검사를 마친 뒤 판정을 갱신 |

## Merge 후 contributor comment 계획

실제 통합 merge가 승인·완료된 뒤 원 재현 검사의 기여, 독립 Print로 기대값을 보정한
이유와 범위를 한국어 존댓말로 설명한다. 실제 merge SHA·검증 head CI와 원 PR의 연결을
적고 위 대표 Native/WASM review·overlay를 merge SHA 고정 raw URL의 이미지로 표시한다.
그 뒤 원 PR을 통합 PR과 연결해 처리한다. 현재는 문안 게시나 PR close를 실행하지 않았다.
