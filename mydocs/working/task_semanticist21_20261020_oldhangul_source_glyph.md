# 옛한글 원본 글꼴의 가시 글리프와 강조점 장평 경계 보정

## 독립 근거와 적용 범위

`d4ad938b3c`의 Native040/fresh WASM015 강조점 검증은 각각 64조건 중 63조건이
90% 이상이다. 옛한글·결합 문자 입력은 87.99371% / 73.64128%로 미달했다.
장평 50/150 조건의 성조 강조점도 숫자 통과와 별개로 가로 원점 차이가 남았다.
이 단계는 이 두 결함을 공통 글리프 선택과 강조점 기하에서 보정한다.

- #2099 → 원 해결 PR #2128은 Hanyang PUA를 표준 자소로 확장해 공개 글꼴에서도
  읽을 수 있게 했다. 원문·표시 문자열·캐럿 주소의 확장은 유지한다.
- 실제 공급 HCR Batang은 F53A에 가시 윤곽선을 갖지만 같은 표준 자소를 GSUB하면
  빈 글리프가 된다. 별도 한컴 Print에서 원 PUA와 대응 표준 자소 모두 F53A로
  인쇄됨을 확인했다. 등록 글꼴 바이트의 cmap·가시 윤곽선으로만 호환 후보를 선택한다.
- 성조 강조점의 첫 원점은 36pt 글자에서 장평 50/100/150 모두 약 32.0226pt다.
  글리프의 가로 배율과 글자 원점으로부터의 앵커 이동량을 구분한다.

근거는 ignored `output/pr-review/semanticist21-20261019/`의
`issue2099-old-hangul-history-189.json`, `pr2128-old-hangul-resolution-190.json`,
`old-hangul-print-source-proof-187.json`, `old-hangul-harfbuzz-probe-181.json`,
`emphasis-tone-ratio-print-188.json`에 보존한다. 글꼴 프로그램은 커밋하지 않는다.

## 생산과 실제 소비

표준 자소→유일한 PUA 후보는 기존 공개 매핑의 역조회다. 모호한 역매핑은 제외한다.
Native는 실제 선택 face의 glyph/path를 확인하고, Canvas2D/SVG는 호스트가 적재한
프로그램의 cmap 범위를 소비한다. CanvasKit은 paint 후보와 실제 선택 font의 glyph
coverage를 대조한 로컬 재생 뷰를 만든다. 원 paint op의 텍스트·위치·문자 주소와
편집 정보는 변경하지 않는다. 가시 PUA가 없는 글꼴은 표준 자소 경로를 유지한다.

Native의 nominal 재생과 complex shaping 선택, Canvas2D의 본문·효과 재생,
SVG의 일반·그림자 출력, CanvasKit의 positioned 재생을 같은 조건으로 확인한다.
글꼴 등록은 새 API로 분리해 기존 HFT 전용 등록 API 계약을 유지한다.

## 검증 순서와 판정

먼저 형식·TypeScript와 Native041을 빌드하고 기존 64조건 및 별도 옛한글 대조군을
같은 원본·독립 Print로 재검증한다. fresh WASM016도 같은 소스로 빌드해 비교한다.
숫자와 직접 확대 판독을 모두 충족한 뒤 정식 회귀를 추가한다. 이전 출력의 점수를
현재 소스의 완료 근거로 재사용하지 않는다. #8064 기능 보존과 나머지 원 PR 검토도
계속하며 이 단계만으로 통합 완료를 선언하지 않는다.
