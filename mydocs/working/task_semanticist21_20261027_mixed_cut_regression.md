# 전체 회귀 실패의 공통 컷 보정

시작 head는 `3ab9cd41ef`, 검증 base는
`0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`다. 기존 원 PR45개와 리베이스로 반영한
기능의 검증 범위를 유지한다. 전체 회귀가 이미 실행한 #7518의 두 기존 원본을 확인하며
새 HWP나 다른 실문서를 추가하지 않는다.

## 독립 근거와 실제 소비 경로

4580 전체 nextest는 11,809 PASS / 6 FAIL / 45 skipped로 종료했다. #7518 실패3건은
같은 혼합 자식 분할을 검출한다. 기존 두 원본의 새 MCP 2020 Print는 모두8쪽이고,
고정 base도8쪽이지만 Native276은6쪽이다. Windows 한컴 `11.0.0.9136`,
PrintMethod0·`hancom2020_pdf_driver_one_up`의 실제 응답과 입력·PDF 해시는4590에 있다.
새 Print와 base의 전쪽 Native 최저는99.96129%다. 현재6쪽과 Print8쪽의 불일치는
실패로 보존했으며, 따로 직접 합성한 현재4쪽 review·standalone overlay의 raw는72.30%다.

base와 현재의 CellUnit 진단204개는 같다. 잘못된 내용 생성이 아니라 선택한 컷의
해석이 바뀌었다. 혼합 자식의 유닛은 `nested_row=None`, `mixed_nested_fragment=true`로
같은 자식의 부분 소스를 나타낸다. 새 `reflow_cut_content_height`는 행/재귀 컷만
조회하고 이 경로를 통째 표로 해석해 한 줄 선택에 전체 높이1239.09px를 예약한다.
그 결과 예산에 맞는 줄 분할 대신 과대 조각을 강제로 놓아 나머지 내용을 다른 쪽에 밀었다.

공통 소스 흐름은 다음과 같다.

- 생산: `cell_units`의 혼합 소스 유닛과 `mixed_nested_split_from_cut`의 시작·끝 소유,
  `visible_height`·`flow_height`.
- 측정: `reflow_cut_content_height`가 같은 혼합 컷 결과를 `Occupied`/`Capacity`에
  맞게 소비하고, 그 문단의 혼합 유닛 밖 일반 글줄도 유지한다.
- 예산: `row_cut_height_for`와 `cell_cut_visible_height`가 같은 결과로 수용·이월을 정한다.
- 배치: `layout_table_cells`가 이미 사용하는 `mixed_nested_split_from_cut`의 동일한
  소스 컷으로 자식을 배치한다. 전체 본체 높이로 다시 바꾸지 않는다.

부분 컷의 시작·종료와 마지막 일반 문단까지 검사한다. 기존 #7518의8쪽·원문 보존·
소유·중복 방지 검사는 변경하지 않는다. 같은 일반 규칙을 쓰는 기존6쪽 nested-split 및
최소 셀·차트·#8188의 정상 대조군도 보존한다.

## 독립 계약과 충돌한 합성 검사 정리

- 원 PR #2512의 이상 행 폭 목적은 유지한다. 최소 물리 폭보다 작은100/150HU 사례를
  1000/1500HU의 유효한 격자로 바꾸고, 정상 대조군과 같은 열 경계·원본 합계·원시 값
  불변을 새 이름으로 검사한다. 실제 페이지 좌표를 새로 고정하지 않는다.
- 원 PR #5709·통합 `1139f28d17d55b499f553354f8711ecc60b110dd`의 두 줄 소유 계약을
  유지한다. 예전 합성 Document는 글자모양 테이블이 없고 서로 다른 표 슬롯도 명시하지
  않았다. 유효한 글자모양·선행 저장 줄과 원시 UTF-16 슬롯을 갖춘 초안은 현재 라이브러리에서
  2 PASS다(4594/4597). 실제 원 기관 문서의 시각 통과로 확대하지 않는다.
- `compound_fraction_color_keeps_the_same_frame_and_advance`는 빈 버전 COLOR를 현대
  RGB 래퍼로 가정했다. 기존 독립 Print·[완료 증거](../pr/assets/legacy_literal_operator_plane_20261025/README.md)는
  그 빈 출력을 확인한다. 이미 검증한 같은 legacy-color 원본의 빈 표시를 새 이름으로
  보호하고, 현대 `Equation Version 60`의 실제 색상·간격 검사는 유지한다.

## 보정 전 전체 측정과 재검증 계획

Native276/fresh086의 기존169조건은 각각384쪽을 모두 측정했다. 최저 raw는
92.05207/91.53306%이며 독립 PDF 쪽수가 일치한다. 이169조건에는 실패한 혼합 원본
두 개가 없으므로 그 통과가 #7518 실패를 면제하지 않는다. 해당 결과와 판독은 보정 전
증거로 보존하고 최종 후보의 통과로 재사용하지 않는다.

아직 시작하지 않았던 옛 제품의 Studio·#8188 WASM 후속 작업만 중지하고 superseded로
기록했다(4598). 현재 수정 후 Native 경계 대조를 먼저 완료하고 최적화 fresh WASM·
영향 원본 전쪽·기존 전체 범위·lint·전체 회귀·Skia·문서·Studio를 검증한다. 공유 target은
계속 `/home/tsjang/rhwp/target/pr-review`를 쓰고 Cargo8 jobs·테스트12 threads를 유지한다.
독립 기대값·허용치를 완화하지 않으며 실제 보정 후 PASS 전에는 해결로 판정하지 않는다.

## 첫 경계 검증에서 확인한 추가 예약 계약

`1797d7357f`의 Native277은8쪽을 복구했지만4쪽에 한 유닛을 더 수용하고,7/8쪽의
raw가81.11723/56.07603%라 실패했다(4600/4607). 따라서 fresh WASM과 전체 후속
검사를 시작하지 않았다. 페이지 수 복구만으로 완료 처리하지 않는다.

이 소스는 자식 행/유닛 커서가 없는 scalar 혼합 스트림이다. `mixed_nested_split_from_cut`의
paint용 높이를 부모 수용량으로 직접 바꾸면, 기존 `mixed_nested_flow_extra_for_height`가
예약하던 시작 물리 띠가 사라진다. 커서가 없는 스트림의 수용·점유는 기존 공통 CellUnit
장부가 계속 처리하게 한다. 명시적 재귀 커서를 가진 행 분할은 현재의 공통 프레임 계산을
유지한다. 파일명·문서 ID·특정 높이로 분기하지 않으며,소스 표현이 실제로 소유하는 컷
계약을 구분한다. 아직 새 제품의 PASS는 없고 두 원본과 정상 대조군에서 재검증한다.
