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

## Native042 / fresh WASM017 실행과 다음 보정

- 제품 소스는 `d2256b24e013dc6d123e13c05bae116809b67b13`에 고정했다.
  Native042 소스 125경로의 commit 바이트 동일성은
  `native042-committed-source-equivalence-242.json`으로 확인했다.
  Native 빌드 228.61초, 최적화 fresh WASM 714.64초 exit 0이며 Studio 동기화 해시도 같다.
- Native 74조건 중 70조건 수치 통과. 현대 자모 Batang/Dotum, italic/bold/bold-italic
  강조점은 100%다. 추가한 5개 분리 입력은 스타일·각주 3조건 통과, 인라인 표 2조건 미달이다.
  #7694 새 author HWP 두 파일도 별도 비교했으며 양쪽 모두 미달이다.
  점수 통과와 직접 판독·기준 대비 하락 판정은 분리한다.
- fresh WASM017은 같은 제품 소스로 위 입력을 합친 81조건을 비교 중이다.
  `wasm017-combined-252-jamo-tone-italic-fixed-wasm/progress.json`과 실행 로그를
  확인하고, 이 기록의 중간 진행값을 최종 완료로 사용하지 않는다.

### 실패 원인의 독립 근거

1. 단독 종성·결합 문자: 원본 SFNT와 Print 내장 글리프의 실제 윤곽·0 전진은 같다
   (`zero-jamo-pdf-font-audit-246.json`). Native SVG의 본문 clip x=99.4667이 실제
   잘린 왼쪽 픽셀과 일치한다(`zero-jamo-clip-diagnostic-247/`). 전진 0을 잉크 없음으로
   해석한 clip 범위를 공통 소비 지점에서 보정 중이다. `text_ink`의 보수적 em 범위는
   실제 잉크 측정값이나 배치 좌표가 아니며 원문·전진·흐름을 변경하지 않는다.
2. 빈 인라인 표: 공용 생성 API가 커서의 36pt 글자모양은 상속하지만 셀 저장 LineSeg와
   행 최소 높이는 10pt로 만든다. 원 해결 #2095 / `9494ba8c61c`의 글자모양 상속과
   현재 `create_table_native` / `create_table_ex_native`의 생산·소비를 대조했다.
   잘못된 저장 줄을 정상 한컴 저장 정보의 수용 조건 완화로 해결하지 않는다.
3. #5874 writer 원본: 원문에는 단 정의가 없다. 한컴 진단용 HWP 저장에서 유효 PageDef는
   기본 여백이며, 원문에 colPr만 추가하면 Print가 선언 여백을 적용한다
   (`italic-column-origin-control-253.json`). 두 Print의 첫 원점은 각각
   (85.0339,107.6424)pt와 (56.6093,65.0889)pt다. 원본·대조군은 구분하며 원본의
   미달을 대조군으로 대체하지 않는다. manifest 변경 두 대조군은 info에서 입력 파싱
   실패해 Print 요청하지 않았고 정상 대조군으로 세지 않는다.
4. 실제 Times italic: Native 94.84380%, fresh WASM도 미달이다. 실제 공급 프로그램·
   PDF의 한글 fallback·브라우저 face 선택을 추가 대조한다. Arial의 100%로 대신하지 않는다.
5. #7694 새 chart-before/after는 각각 Native 4.81258% / 89.01177%다.
   공개 저장 경로와 원본 OLE/OOXML 앵커·제목·축을 추가 분석한다.

추가 보정은 아직 빌드·시각 재검증 전이다. 새 회귀·fixture/golden은 해당 원본의
Native/fresh WASM 90% 이상·직접 판독을 완료한 뒤에만 추가하며 미달 기대값을 갱신하지 않는다.

### 실제 Canvas 기울임 계약 재확인

`canvas-italic-font-contract-256.json`에서 CSS 문법 수용 여부와 실제 잉크 메트릭을
분리했다. HCR 48px H의 우측 잉크는 normal 34px, 기본 italic 42px,
`oblique 18.418deg`도 42px이며 Canvas getter는 지정 각도를 버린다.
현재 CSS 각도만으로 공통 shear가 적용됐다고 주장하지 않는다. 실제 italic face의
glyph coverage를 확인해 그 경로는 유지하고 upright source에만 공통 합성을 적용하는
추가 보정이 필요하다. Times 대조군 미달도 함께 확인한다.

fresh WASM017의 81조건 실행은 종료됐다: 수치 통과 73조건, 미달 8조건, exit 1.
이전 패키지의 ExtB 상한 예외로 중단됐던 preserve-042/054/099는 실제 새 WASM에서
모두 렌더 완료했고 각각 92.56774% / 92.56774% / 96.49315%다.
이는 전체 #8064 fresh WASM 179조건의 최종 보존 판정이나 현재 미커밋 clip 보정의
검증을 대신하지 않는다. 전체 실패 목록은 `wasm017-combined-252`의 progress/results를 따른다.

## 공통 소비 경로 보정 묶음과 Native043

0 전진 글자의 잉크 범위를 본문 clip과 paint bounds가 함께 소비하도록 보정했다.
실제 source cmap와 face 스타일을 선택적으로 등록하여 정체 프로그램만 공통 -0.333
잉크 변환을 적용하고, 실제 italic/oblique는 보존한다. Canvas의 CSS 각도 지원 판정을
실제 합성의 증거로 사용하지 않는다. 미수집·자원 상한은 기존 브라우저 선택을 보존한다.

빈 셀의 상속 글자모양과 줄 메트릭은 일반 재조판 생산자를 사용한다. 읽은 원본의 빈
한 줄 em 캐시는 합성 입력뿐 아니라 독립 한컴 저장의 3600/3060/2160HU와 대조했다
(`table-effective-line-metrics-262.json`). 원본을 바꾸지 않고 측정과 배치의 공통
FrameGeometry를 갱신한다. 비어 있지 않은 압축 줄·다중 안내 줄·0높이는 제외한다.

단 정의가 없는 writer HWPX의 유효 기본 용지는 원본 Print와 colPr만 추가한 반례의
실제 원점 차이에 따라 parser에서 결정한다. 단 정의가 있는 정상 문서는 보존한다.
위 보정을 한 제품으로 동결해 Native043으로 빌드한다. 아직 성공·시각 통과를 주장하지
않으며, 원본과 독립 Print의 재검증을 마친 뒤 회귀의 추가·변경을 결정한다.

### Native043 관측과 잔여 범위

최초 빌드의 Arc 타입 오류를 수정했고 두 번째 빌드가 215.43초·exit 0으로 끝났다.
`logs/char-overlap-native-build-043-attempt02.log`와 제품 소스 지문을 보존했다.
`ink-face-critical-265-ink-face-fixed-native`는 15조건 중 13조건이 90% 이상이다.
0폭 자소, 현대 자모 양 계열, bold-italic, 실제 Arial/Times italic, writer 원본 두
engine과 colPr 반례는 100%다. 소스 경계 혼합 입력은 99.18718%로 개선됐다.
빈 인라인 표 두 원본은 본체 크기만 맞고 부모 줄 기준선이 낡아 73.79481%와
62.64501%다. 이 원본의 최종 통과나 새 회귀 추가를 주장하지 않는다.

Times 한글의 Print 내장 글리프와 실제 HBATANG.TTF의 윤곽·전진·em이 같다
(`physical-italic-hangul-source-audit-264.json`). 별도 공급 폴더에서 사용하지 않는
Malgun을 제외한 62,697,380바이트를 공급해 Native의 64MiB 한도를 지켰다. 과거
글꼴 조건을 덮어쓰지 않았으며 같은 공급으로 frozen Native042도 100%임을 확인했다.

#7694는 별도 진단용 한컴 저장본을 얻었다(`mcp-chart-effective-save-269`). 원본
Print를 기준으로 유지한다. chart-before의 실제 저장 줄은 18750/15938HU로 바뀌며
원본의 1000/850HU 캐시가 개체 높이를 담지 못함을 확인했다. 정규화 저장본도 Native
89%대로 차트 내부 라벨·간격이 남아 있어, 호스트 줄과 차트 내부를 모두 보정해야 한다.
현재 묶음을 로컬 중간 커밋으로 고정한 뒤 후속 공통 프레임 보정을 계속한다.
fresh WASM018·전체 Rust·전체 시각 검증·최종 확대 판독은 아직 완료하지 않았다.

## 단일 인라인 상자의 저장 호스트 프레임 보정

`composer/inline_box_rows.rs`는 실제 인라인 상자를 담지 못하는 단일 저장 줄을
기존 렌더 정규화의 inline_frames 경로에 투영한다. 한 개체와 중립 표지만 허용하고
여러 줄·캡션·고정 줄간격·다른 세로 정렬은 기존 소유자에게 둔다. 표는 활성 글꼴보다
작은 빈 셀 캐시가 있을 때 기존 HeightMeasurer의 실제 본체 높이를 소비하며, 표의
바깥 여백을 같은 줄 상자에 포함한다. 개체 높이와 줄 기준선은 일반 재조판의 같은
apply_inline_control_line_height에서 발행한다. 단순 paint 좌표 clamp를 추가하지 않는다.
기존 정규화가 후속 원점과 공통 composed 결과를 함께 갱신하며 편집 원문은 보존한다.

Native043에서 실패한 표 두 조건과 #7694의 원본을 Native044로 직접 비교한다.
정식 회귀 추가·기대값 변경은 Native/fresh WASM 조건 충족 후에만 수행한다.

### Native044 관측과 직접 판독

Native044는 218.82초·exit 0이다. `ink-box-critical-277-ink-box-fixed-native`의
17조건 중 15조건이 수치 통과했고, 표 선두/중간 두 원본은 모두 100%로 개선됐다.
새 review·standalone overlay는 `ink-box-critical-panels-279`에 보존했다. 직접
판독에서 글자·결합 부호의 소속과 표 본체·호스트 기준선이 원본 Print와 맞는 것을
확인했다. 100%는 2px raw 실루엣이며 RGB/획 AA 완전 일치라고 보고하지 않는다.

#7694 chart-before는 이전 4.81258%에서 89.38709%로 개선됐다. 위쪽 잘림은
해소됐지만 차트 내부 축·라벨 간격이 남았다. chart-after도 89.00396%로 미달이다.
원본 OOXML·Contents·OlePres000을 `pr7694-chart-source-268`에 보존했다.
전체 전역 fontdb와 명시 HCR 두 파일의 선택을 독립 resvg 진단으로 대조한 결과
같은 HCR Dotum 프로그램을 골랐고 렌더 PNG도 바이트상 같았다. 이 입력의 내부
차이를 글꼴 누락으로 처리하거나 무관한 글꼴 조달 변경으로 해결하지 않는다.
`logs/native-svg-font-probe-278-attempt02.log`를 참고한다.

다음 단계는 같은 Native044 제품 소스의 최적화 fresh WASM018이다. Native 결과만으로
새 회귀·최종 통합 판정을 만들지 않는다. 남은 차트 내부와 다른 필수 항목의 보정도
계속하며, 원격 push/PR은 준비 완료 후 작업지시자의 승인 전에 수행하지 않는다.
