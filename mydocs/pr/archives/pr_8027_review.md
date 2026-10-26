# PR #8027 리뷰 — fix(emf): EMR_POLYGON16·EMR_POLYLINE16 레코드 번호를 바로잡는다

## 최종 판정

**메인터너 보정 후 수용 가능 — EMF 16bit 레코드 종류와 OLE 원본 출력 범위의 로컬 검증 완료.**

원 contributor head가 아니라 메인터너 보정한 통합 후보의 판정이다. 아래 현재 증거와
실제 검사 실행을 대조했다. 통합 PR의 정확한 code head CI·merge·후속 처리는
앞으로 수행할 조건이며 로컬 통과로 대신하지 않는다.

## 최종 로컬 판정 근거 — 2026-10-26

원 head `1e2532049a260fef9dd814a22d8c1d1253c4b109`가 원격과 같음을 재확인했다. 검증 코드는
`61528013511c87c732524f91c6f224286a055337`, 실제 렌더 source는
`bbf3aed18dd82bbf344072b77b5ac590e7a71f3a`, 고정 devel은
`238de6de41a208648f4ade6b494fe433a81c1fce`다. 제출 전 fetch에서도 같은 base를
확인했으므로 추가 리베이스하지 않았다. 이후 문서·증거 커밋은 코드·검사를 바꾸지 않았다.

| 필수 근거 | 최종 확인 결과 |
| --- | --- |
| 원 PR의 현재 검사 | 아래 모듈·함수별 실제 PASS와 로그 행을 확인했다. 소스 존재를 실행 결과로 대신하지 않았다. |
| 전쪽 Native/fresh WASM | 기존 원본 1조건·전체 1쪽/backend에서 최저 99.28860% / 99.30234%, 독립 Print 쪽수 일치·90% 미만/누락 0쪽 |
| 고정 devel 보존 | 동일 입력·Print·글꼴·backend의 페이지별 비교에서 0.1%p 초과 하락 0쪽 |
| 직접 판독 | 열린 POLYLINE16과 닫힌 POLYGON16을 실제 parser·SVG 소비에서 검사했다. 기존 OLE 원본의 막대·축·범례·배경·외곽과 뒤 본문을 직접 판독해 #8064 기능 보존을 확인했다. |
| 전체 로컬 게이트 | fmt·수집/단위 정책·Release build/lib·전체 nextest·Skia 3종·Native/WASM/workspace Clippy·workspace build·doctest 완료. 실제 종료 근거는 아래 공통 기록 참조 |
| 원격 절차 | 통합 PR의 정확한 code head CI·실제 merge·후속 comment는 앞으로 수행할 조건이다. 원 contributor head를 직접 승인·merge한 기록이 아니다. |

기존 원본의 전체 페이지를 대조했다. [전쪽 TSV·원본/Print·producer·해시와 판독](../assets/legacy_chart_physical_layout_20261020/current-20261026/README.md)을
이 PR의 직접 근거로 연결한다. raw는 색상 경계 보정 전 2px 실루엣이며 RGB 완전 일치나
과거 별도 지표의 통과를 뜻하지 않는다. Native255·표준 release/wasm-opt를 마친 fresh073을
사용했다. 측정 뒤 코드 변경은 없으며, 과거 제품의 표는 당시 결과로 구분한다.

| 출력 경로 | 현재 대표 review | 현재 standalone overlay |
| --- | --- | --- |
| Native | [review](../assets/legacy_chart_physical_layout_20261020/current-20261026/native/candidate/review_001.png) | [overlay](../assets/legacy_chart_physical_layout_20261020/current-20261026/native/candidate/overlay_001.png) |
| fresh WASM | [review](../assets/legacy_chart_physical_layout_20261020/current-20261026/wasm/candidate/review_001.png) | [overlay](../assets/legacy_chart_physical_layout_20261020/current-20261026/wasm/candidate/overlay_001.png) |

Merge 후에는 이 대표 파일을
`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/legacy_chart_physical_layout_20261020/current-20261026/`
위 표의 실제 상대 경로로 고정해 contributor comment에 두 이미지를 함께 표시한다.
같은 merge의 devel 포함·asset 존재·CI 결과를 먼저 확인하고 한국어 존댓말의
`--body-file` 본문을 게시한 뒤 API에서 한글·링크·실제 Markdown을 다시 확인한다.


## 접수 정보

이하 최초 접수와 중간 검토의 제품·기준 SHA 및 당시 진행 상태를 보존한다.
현재 판정은 위 「최종 로컬 판정 근거」와 아래 「통합 후보의 실제 검사 실행」이 정본이다.
과거의 진행 중·미완료 표현을 현재 로컬 게이트 상태로 사용하지 않는다.

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#8027](https://github.com/edwardkim/rhwp/pull/8027) / semanticist21 |
| 고정 원 head | `1e2532049a260fef9dd814a22d8c1d1253c4b109` |
| 원격 확인 | 2026-10-26 KST: head 일치, reviewer jangster77 지정 확인; 당시 draft=false |
| 통합 브랜치 | `review/semanticist21-20261019` |
| 코드·회귀 후보 | `61528013511c87c732524f91c6f224286a055337` |
| 실제 렌더 소스·증거 등록 | `bbf3aed18dd82bbf344072b77b5ac590e7a71f3a` / `2c6393f7bb` |
| 고정 devel | `238de6de41a208648f4ade6b494fe433a81c1fce` |

## 변경과 원 커밋 적용

| 원 커밋 | 통합 후보의 대응 |
| --- | --- |
| `1e2532049a260fef9dd814a22d8c1d1253c4b109` | `12a1c9e32a831caf8bf5baa49f622389cd9a8db3` |

### 실제 소비 경로와 반례

EMF parser의 0x55/0x56/0x57를 각각 POLYBEZIER16/POLYGON16/POLYLINE16로 분리하고, dispatch에서 열린 선과 닫힌 다각형의 record 종류를 서로 바꾸지 않는다. 범주·값을 다시 구성하는 OOXML 차트 로직과 저장된 OLE 미리보기의 EMF 소비 경로를 구분한다. legacy 차트의 물리 배치·축 라벨 보정은 원 record 종류 수정과 별도의 공통 paint 문제로 검토한다.

src/emf/parser/mod.rs의 상수와 record dispatch, src/emf/tests.rs의 16bit record 검사 및 mydocs/pr/assets/legacy_chart_contracts_20261020/README.md와 legacy_chart_physical_layout_20261020/README.md의 독립 Print 기록.

### 현재 소스와 검사

- [src/emf/parser/constants/record_type.rs](../../../src/emf/parser/constants/record_type.rs)
- [src/emf/parser/mod.rs](../../../src/emf/parser/mod.rs)
- [src/emf/tests.rs](../../../src/emf/tests.rs)

위 소비 경로의 Native·브라우저 검사를 실제 실행 기록과 대조한다. 함수 존재를 실행 성공으로 세지 않는다.

삭제·교체한 옛 검사와 현재 활성 검사를 구분한다. 위 목록은 현재 source 검사이며 실행 PASS 집계가 아니다.

## 검증 입력과 결과

위 최종 절에 연결한 현재 입력·독립 Print·Native/fresh 출력으로 판정했다. helper assertion만으로 출력 일치를 선언하지 않았다.

전체 로컬 Rust 검증은 완료했다. 현재 함수별 실제 PASS와 종료 근거는 아래 「통합 후보의 실제 검사 실행」에 기록했다. 독립 시각 근거와 원격 CI는 별도로 판정한다.

통합 후보의 현재 공통 시각 비교에서는 17조건·213쪽/backend의 후보 최저가 Native 92.04708%, fresh WASM 91.53306%였다. [전쪽 TSV](../assets/semanticist21_rebase_qualified_20261026/README.md)의 수치 대응 가능한 16조건·211쪽/backend에서 0.1%p 초과 하락 0쪽을 확인했고, #6133은 base 1쪽에서 독립 Print와 같은 2쪽으로 복원된 구조 개선으로 따로 기록했다. 이는 이 PR의 별도 입력이나 아래 필수 증거가 자동으로 검증됐다는 뜻이 아니다. #8228의 변경 기능 보존과 현재 보정 대상만 확인하며 다른 HWP로 범위를 넓히지 않는다.

| 조판 검토 항목 | 현재 근거와 판정 경계 |
| --- | --- |
| 규칙·공통 소비 | 위 실제 소비 경로와 반례를 따른다. helper 뒤 덮어쓰기·다른 frame을 실제 결과와 대조한다. |
| 입력·독립 기대값 | 입력 생성 방식과 한컴 Print 출처를 증거별로 구분한다. source와 무관한 값이나 원 작성자의 주장만으로 통과 처리하지 않는다. |
| 줄·문단·개체 소유 | 현재 검사 소스의 순서·누락·중복·포함·겹침 관계를 확인한다. |
| 시각·px 자격 | 전쪽 양쪽 raw 90% 이상·쪽수 일치·직접 판독이 필요하다. ±2px는 양쪽 98% 이상을 확인한 해당 페이지의 독립 좌표에만 허용한다. |
| 리베이스 기능 보존 | 새 base와 현재 후보의 동일 입력·producer·Print 조건을 대조한다. 평균값으로 페이지별 0.1%p 초과 하락을 상쇄하지 않는다. |
| 완료된 로컬 게이트 | 아래 실제 실행 결과와 공통 종료 기록을 대조했다. 이 PR의 독립 시각 증거와 원격 CI는 별도로 판정한다. |

sample16·편람의 기존 삭제를 유지하며 삭제 파일을 향하던 파생 참조도 재생성하지 않는다. 미해결 이관 문서를 완료 범위나 이슈 닫기 목록에 넣지 않는다.

## 현재 OLE 차트 원본의 직접 증거

[현재 원본 검증](../assets/legacy_chart_physical_layout_20261020/current-20261026/README.md)에서
같은 HWP·독립 Print의 한 쪽을 다시 비교했다. Native255는 base 94.51108% → 99.28860%,
최적화 fresh WASM073은 base 94.08200% → 99.30234%다. 쪽수 일치와 양쪽 90% 이상,
0.1%p 초과 하락 0쪽을 확인했다. 두 출력의 review·standalone overlay를 직접 판독해
막대·축·범례·차트 외곽·배경 색조와 뒤 본문 보존을 확인했다. #8064에서 반영한 기능과
통합 후보의 공통 OLE paint 보정이 함께 유지된다. 본문 일부의 미세 간격 잔차는 별도로 남긴다.
이는 이 원본의 완료 증거이며 전체 PR의 미완료 로컬 게이트를 대신하지 않는다.

## 통합 후보의 실제 검사 실행 — 2026-10-26

코드·회귀 후보 `61528013511c87c732524f91c6f224286a055337`의 실행 결과를
아래 현재 소스와 모듈·함수 이름으로 대조했다. 이후 증거·문서 커밋은 코드와 검사를
바꾸지 않았다. 아래 결과는 실행한 계약 검사의 통과이며 별도 입력의 독립 Print 일치나
원격 CI·merge 완료를 뜻하지 않는다. 과거 제품의 집중 검사는 당시 결과로 구분한다.

- [src/emf/tests.rs](../../../src/emf/tests.rs): SHA-256 `e75f445d26957a1a6443012901c0cacd32a4bbed68d9cfdeed079f7170eb1232`.
  - `parses_polyline16_with_points`: PASS, `logs/final-full-nextest-4052.log:1005` (`rhwp`).
  - `polyline16_rejects_unbounded_point_count`: PASS, `logs/final-full-nextest-4052.log:1008` (`rhwp`).
  - `convert_to_svg_polyline_and_ellipse`: PASS, `logs/final-full-nextest-4052.log:991` (`rhwp`).
  - `convert_to_svg_polygon_closes_shape`: PASS, `logs/final-full-nextest-4052.log:990` (`rhwp`).

공통 로그 루트는 ignored `output/pr-review/semanticist21-20261019/`다.
[전체 Rust 종료 결과·실행 경계](../../working/task_semanticist21_20261026_regression_contracts.md#후속-보정과-최종-rust-검증--2026-10-26)의
전체 nextest 11,708 PASS·0 FAIL, Native Skia 라이브러리 4,040 PASS·0 FAIL,
Native/WASM/workspace Clippy·workspace build·문서 테스트의 실제 완료 근거를 재사용했다.
전체 nextest 바깥 driver의 SIGTERM과 명시적 child terminal summary를 구분했다.
공유 `target/pr-review`를 사용했고 검사·기대값을 통과 목적으로 바꾸지 않았다.

이 원 PR의 기능 커밋과 현재 통합 대응은 다음과 같다. 과거 rebase 이전 SHA와 구분한다.

| 원 커밋 | 현재 통합 대응 |
| --- | --- |
| `1e2532049a260fef9dd814a22d8c1d1253c4b109` | `12a1c9e32a831caf8bf5baa49f622389cd9a8db3` |

sample16·편람의 삭제를 유지했다. 현재 수집 소스를 읽기만 했으며 이전 fixture·보관본·
파생 참조는 만들지 않았다. 필수 독립 시각 근거와 실제 통합 head CI는 이 실행표와
별도로 판정한다.

## Merge 후 contributor PR comment 계획

[Visual Sweep merge comment 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)에 따라 실제 통합 PR·merge SHA·CI 결과와 이 PR의 보정 범위를 존댓말로 설명한다. 시각 검증에 사용한 대표 review·overlay는 위 안정 경로에서 실제 merge SHA로 고정한 raw URL로 함께 표시한다. 아직 확정하지 않은 수치·이미지 URL이나 미실행 경로를 만들지 않는다.

실제 asset의 devel 반영을 확인한 뒤 `--body-file`로 게시하고 API로 본문·이미지를 재조회한다. contributor branch·fork를 삭제하거나 원 history를 rewrite하지 않는다. [개별 구현·후속 절차](pr_8027_review_impl.md)를 따른다.
