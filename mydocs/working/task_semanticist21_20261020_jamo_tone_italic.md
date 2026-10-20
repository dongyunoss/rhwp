# 자소의 실제 폭·성조 정렬·합성 기울임의 공통 보정

## 독립 근거와 기존 구현 대조

- 옛한글 원 해결 이슈 #2099 / PR #2128의 표준 자소·원문 주소 보존은 유지한다.
  현대 NFD는 Unicode Hangul의 L/V/T 합성으로 같은 음절을 식별한다. 가시 glyph가 있는
  실제 source face에서만 paint용 음절을 선택하고 원문·편집 좌표는 변경하지 않는다.
- `oldhangul-modern-font-advances-218.json`과 `hangul-jamo-metric-source-224.json`은
  HCR source 4개 프로그램의 cmap/hmtx를 확인했다. 중성·종성의 실제 0폭을 미수집으로
  취급한 1em 폴백과 현대 NFD의 폭·glyph 선택을 함께 보정한다. Print의 낱자 원점도
  같은 0폭을 입증한다(`oldhangul-modern-print-trace-213.json`).
- 굵은 성조의 빈 문맥 자소는 일반체와 전진이 다르다. `emphasis-canvas-tone-bold-probe-226.json`
  의 실제 Canvas 측정에서 문자열 끝점 정렬은 양쪽 glyph 원점을 맞춘다. 반 em 같은
  고정 보정량 대신 Canvas의 right 정렬과 SVG의 end anchor를 같은 역할로 사용한다.
- `emphasis-italic-print-matrices-208.json`의 한컴 Print 합성 기울기는 0.333이다.
  기존 Native 0.25·CanvasKit 0.2·브라우저 기본 합성을 같은 본문 계약에 맞춘다.
  실제 italic face는 추가 shear를 적용하지 않는다. 별도 Arial/Times italic 입력의
  Print가 실제 italic face와 한글 fallback의 합성을 구분하는 대조군이다.

## 실제 소비와 반례

측정의 문자 폭·trace와 배치 advance는 같은 자소 메트릭을 소비한다. Native/SVG/Canvas2D/
CanvasKit의 paint용 문자열은 같은 유일한 PUA 또는 현대 합성 음절 후보와 실제 가시 glyph를
대조한다. 원 paint op의 표시 문자열·주소·전진은 유지하며 재생 뷰에서만 glyph를 바꾼다.

공통 성조 geometry는 기준점과 문자열 정렬 역할을 제공하고 실제 glyph는 각 backend의
같은 글꼴·굵기에서 그린다. 본문 synthetic italic은 실제 italic 프로그램과 구분한다.
SVG/PDF 호환 경로와 브라우저의 CSS 표현도 확인하며 수식 전용 italic 근사는 이 본문
기울기의 근거로 대신하지 않는다.

기존 66조건, 현대 자소 2조건, 실제 italic 대조군과 #7694의 새 저장 경로를 batch로
검증한다. 새 #7694 HWP 두 파일의 Print 원문은 적용한 source head와 SHA가 일치함을
`pr7694-update-applied-227.json`에서 확인했다. 전쪽 수치와 직접 확대 판독, 수정 전후
검출을 확인한 뒤 정식 회귀를 추가·수정한다. 현재 미세 차이는 수치 통과로 면제하지 않는다.

## 소스 자소와 실제 실행

강조점은 실제 0 전진과 독립 결합 문자도 표시한다. 원문을 스타일별로 나누기 전의
자소 소속을 복원하되 인라인 개체·주석은 별도 소유자로 취급한다. 일반 줄 경로는
실제 논리 슬롯과 선두 주석 prefix 생략을 투영하고, 전용 인라인 표 경로는 이미
보유한 원문 scalar 위치를 사용한다. 좌표 일치나 run 첫 글자의 Unicode 범주만으로
소속을 추정하지 않는다. 생성 표시값은 원문 대응 확인 없이 합치지 않는다.

- 동결 Native041의 #8064 보존: 179조건·2,082쪽 전부 90% 이상, 최저 90.16447%,
  기준 대비 0.1%p 이상 하락 0건. 전쪽 TSV 해시 재확인은
  `output/pr-review/semanticist21-20261019/pr8064-native041-final-audit-239.json`에 있다.
  현재 보정 소스나 fresh WASM의 최종 검증으로 승격하지 않는다.
- TypeScript는 Studio 작업 디렉터리의 `npx --no-install tsc --noEmit`으로 통과했다.
  루트 npx 호출은 프로젝트 TypeScript를 선택하지 못했으며 정식 타입 검사 통과로
  기록하지 않는다. 실제 성공 로그는 `logs/jamo-tone-italic-typescript-236-attempt02.log`다.
- Native042는 공유 `target/pr-review`, 컴파일 잡 8개로 전체 보정 소스를 함께 빌드했다.
  이어서 독립 Print의 현대 자모·성조·italic·0폭·소스 경계 대조를 수행하며,
  fresh WASM과 직접 판독·기준 대조 전에는 완료 판정이나 회귀 기대값 변경을 하지 않는다.
