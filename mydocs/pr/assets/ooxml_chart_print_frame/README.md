# 인라인 이동 차트의 독립 Print 프레임 회귀

새 검사는 기존 `inline_control_move` 검사를 교체하지 않는다. 같은 기존 HWP/HWPX의 이동
전후 네 입력과 한컴 2020 Print PDF를 재사용한다. 새 실제 문서는 추가하지 않았다.
[독립 기준과 실행 명령](independent-oracle.json)에 원본·PDF·producer·TSV·PNG 해시를 기록했다.

공유 target에서 다른 worktree의 오래된 차트 의존성이 연결되면 축약 RenderTree JSON은
같아도 실제 SVG 플롯·제목·외곽선은 달라질 수 있다. source 파일 해시만으로 실제 linked
의존성의 동일성을 선언하지 않고 새 Native276과 최적화 fresh086의 출력으로 확인했다.

## 시각 자격과 직접 판독

네 입력 모두 독립 Print와 같은 한 쪽이며 Native/fresh WASM raw가 각각 98% 이상이다.
원문 TSV는 각 backend 하위 경로에 바이트 그대로 보존했다. 같은 96dpi·Print profile·기존
compat 2022·동일 글꼴 공급 조건이다. compat 표기와 기준 출력 제품의 연도를 혼동하지 않는다.
이동 전후의 제목·플롯·세 계열·네 항목·범례·앞뒤 본문을 review와 standalone overlay에서
직접 대조했다. 글자 가장자리·선 농도의 잔차는 남으며 raw를 RGB 완전 일치로 부르지 않는다.

## 독립 기대값과 관계 검사

좌표는 RHWP의 SVG나 구현 계산식에서 복사하지 않고 독립 Print의 96dpi raster에서 읽었다.
원래 PDF는 차트를 여러 이미지 띠로 출력하므로 PDF 텍스트/벡터 drawing 목록만으로
차트 위치를 판정하지 않는다. 외곽 네 변, 값축 x와 플롯 위·아래의 좌표를 ±2 CSSpx로
보호한다. 플롯 끝의 래스터 위상이나 글꼴 baseline을 임의로 새로운 상수로 고정하지 않는다.
차트 하나·네 항목·세 범례·열두 막대의 보존, 제목/플롯/범례의 포함·순서와 막대의 같은
기준선 관계도 함께 검사한다. 실제 공개 `export-svg --profile print`의 RawSvg를 검사하므로
payload를 생략하는 축약 RenderTree만 같다고 통과하지 않는다.

정식 Rust 실행과 같은 검사 바이너리의 이전 CLI FAIL/현재 CLI PASS 결과는 실행 종료 후
연결한다. 시각 선행 자료만으로 전체 Rust나 원격 CI 완료를 주장하지 않는다.

## 대표 review와 overlay

Native 이동 전:

![Native review](native/chart-before-hwp/review_001.png)
![Native overlay](native/chart-before-hwp/overlay_001.png)

fresh WASM 이동 후:

![fresh WASM review](wasm/chart-after-hwp/review_001.png)
![fresh WASM overlay](wasm/chart-after-hwp/overlay_001.png)
