# 구형 문자 평면과 현대 COLOR 경계 보정

## 구현 전 근거와 범위

이전 단계는 `2439af4e79`에서 커밋했다. 제품 기준은 `bc1e99abf036ef6febc551437962ef20fb726939`,
정책 base는 `9418ed0c7cf289c5c4ac2b3bb8cdc18a1b637a9a`다. 새 PR을 체리픽하지 않는다.
계속 실패 중인 구형 literal·CJK와 현대 COLOR의 공통 글리프·원점·간격 경로를 보정한다.
관련 검사 기대값·baseline은 같은 원본·독립 Print의 새 Native/fresh 검증 전에 바꾸지 않는다.

독립 한컴2020 11.0.0.9136의 PrintMethod0·1-up 출력을 사용했다. 원본·요청·job·PDF·SHA와
실행 원장은 ignored `output/pr-review/semanticist21-20261019/`에 보존한다.

| 근거 | 관측과 적용할 규칙 |
| --- | --- |
| 3301~3305, 108조건·324glyph | 구형 `%`·`△`·`․`는 Type3 원문 문자다. 자연 전진은1em, root 대비 논리 전진은9/2이며 ink 크기를 늘리지 않는다. 원점 모델 최대 차이0.33973CSSpx. |
| 3330~3332·3342, 현대4입력 | 일반/그룹 `=`의 간격은 같지만 COLOR는 양쪽 자동 여백이 없다. 현재 일반/그룹은 양쪽100%, COLOR는 Native81.25%·frozen fresh05675.52743%다. |
| 3343~3346, CJK·연산자·그룹72조건 | CJK와 =/+/− 전진9/2, = 앞5/8·뒤3/4, +/− 양쪽1/2, / 전진9/4. 그룹은 기존 root1과 경계−1/4·1/2를 소유한다. 최대 원점 차이0.61168CSSpx. |
| 3348~3350, 영문/CJK 혼재54조건 | 기존 영문 전진을 유지한 같은 모델의 최대 차이1.19568CSSpx. 글리프는 자연 크기로 그린다. |
| 3357, 혼재 baseline108관측 | 선형 CHAR 평면의 Haansoft Batang CJK baseline은 Type3 수학 baseline보다0.06em 낮다. 최대 차이0.08018CSSpx. 논리 글줄 baseline과 물리 ink baseline을 구분한다. |
| 3358~3360, 그룹·연산자9반례 | 그룹 경계가 자동 연산자 여백보다 우선한다. 두 여백의 합산은3px 이상 틀린 반례가 있다. 빈 그룹도 논리 점유를 유지한다. |

## 생산 결과와 실제 소비 경로

`EqLayout::for_equation`의 구형 CHAR·절대 선언 너비 → `EqLayout::layout`의 같은
`legacy_coordinate_plane` 계획 → 자식 GlyphRun의 논리 폭·원점 및 root 너비 →
SVG·Native Skia·Web Canvas의 같은 LayoutBox 소비 경로다.
`stored_horizontal_scale`에서 root 너비를 다시 보며, 계획이 수용한 선언 너비와 같으므로
ink에 전체 가로 확대를 재적용하지 않아야 한다. 문단의 저장 개체 너비·높이·baseline은 바꾸지 않는다.

구형 문자 공급은 `legacy_glyphs`의 같은 자연 전진 결과를 확장한다. 사설 glyph 바이트·윤곽을
코드에 넣지 않는다. 준비한 로컬78자 후보(3340)는 기존75자의 CFF 명령열·전진이 같고,
등록본은 아직 기존75자임을3357 이전에 확인했다. 교체 후에는 실제 font SHA를 따로 기록한다.

CJK는 기존 Text의 자연 측정을 유지하고, 독립 근거가 있는 같은 선형 평면을 수용할 때만
실제 글리프 원점·Haansoft Batang·물리 baseline을 공통 GlyphRun으로 발행한다.
분수·첨자·다른 줄·미확인 글꼴/스타일은 기존 원점을 유지한다. 문서 ID나 우연한 크기로 분기하지 않는다.
그룹과 연산자의 원점은 하나의 Plan에서 계산하고, 그 뒤 개별 painter에서 보정하지 않는다.

현대 `%`는 HyhwpEQ U+E042와 그853/1024 논리 전진을 소비한다. 구형 Type3 `%`와 구분한다.
현대 COLOR는 `symbol_spacing::boundary_element`가 바깥 행으로 연산자 분류를 투명하게
전파하지 않게 한다. 내부 행과 일반 그룹의 연산자 간격은 유지한다.

## 검증 계획과 현재 상태

한 번의 Native와 fresh 빌드 뒤 위 입력과 기존 구형 문자·그룹·분수·baseline·현대 기호·색상
대조군을 함께 실행한다. 전쪽 TSV, 대표 review·overlay와 직접 확대 판독을 완료한다.
현재는 구현 동결 전 계획이며 새 수식 runtime의 통과나 PR 준비 완료를 주장하지 않는다.
기존 helper의 char_count2와 원문 자리표 불일치, 빈 버전 COLOR의 무효 구문은 별도로 보존하고
유효한 재생성 대조군과 섞지 않는다. 수동 메타데이터를 위해 엔진 수용을 완화하지 않는다.

## 적용한 공통 변경과 동결

다섯 파일에 묶어서 적용했다. `legacy_glyphs`는 세 Type3 literal의 자연1em을 발행한다.
`legacy_coordinate_plan`은 기존 문자 전진을 유지하고 CJK·연산자의 논리 전진과 그룹 우선
경계를 함께 계산한다. `legacy_coordinate_plane`은 같은 글줄의 CJK Text까지 소유를 대조한 뒤,
수용한 CHAR 평면에서만 Haansoft Batang GlyphRun 원점과0.06em 물리 baseline을 발행한다.
분수·첨자 등 수용하지 않은 트리의 기존 자연 Text 측정·배치는 바꾸지 않는다.
현대 퍼센트는 같은 전진 결과 안에 실제 U+E042를 담고, COLOR는 외부 연산자 분류를 전파하지 않는다.

빈 그룹·연산자 인접9반례의 우선 경계 모델은 합산 모델과 구분했다. 특히 `{가}=나`의
합산은2.76CSSpx, `{A}=B`는3.28CSSpx까지 틀려 채택하지 않았다. 그룹 경계 우선 모델은
9반례에서 최대1.02938CSSpx였다. 한글 baseline108관측의0.06em 모델 최대 오차는0.08018CSSpx다.

표 여백 단계의 Native/fresh·base 대조와 이미지 판독 종료를 먼저 확인한 뒤 로컬 글꼴을
등록했다(3361). 등록 전 SHA는 `4592a1dfa915f3b6321508cd77971b62279d68751e8230c58d2fa3231d506020`,
등록 후 SHA는 `e7a47a66a224c6a05acc10cbb5d7d018a5104194bb30fb84e75520bb16309c02`다.
이전75자 파일은 output에 보존했다. 기존75자의 CFF 명령열·전진이 같은 후보에 세 문자만
추가한 것이며, `fc-cache` 성공과 `fc-match`의 정확한 등록 경로도 확인했다.
글꼴 파일·윤곽은 Git에 포함하지 않았다. 이전 검증을 새 글꼴 공급의 결과로 다시 표기하지 않는다.

수정 파일 rustfmt·diff check는 통과했다. 다음 실행은 이 소스를 고정한 실제 Native/fresh
빌드와 위 입력의 재출력이다. 아직 새 runtime의 컴파일·회귀·시각 통과를 주장하지 않는다.
