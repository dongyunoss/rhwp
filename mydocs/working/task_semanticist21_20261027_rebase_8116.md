# 새 base #8116의 기능 보존과 통합 검증

## 범위와 이전 단계의 실제 상태

새 devel은 `faa885e8b54ffc7cdf4196cd43a3103f362499a2`(#8116, #8080)다.
이전 base `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`의 통합 head
`7b5299787e15c345da13f92f0b3fcc462df55eeb`는
`backup/review-semanticist21-before-faa885-20261027`에 보존했다.
390개 커밋을 리베이스한 직후 head는 `7d9d2b8ae9600279350a6892c875a7b6b1e6ee20`이다.
원 PR 45개를 유지하며 새 외부 PR을 체리픽하거나 다른 실문서를 수집하지 않는다.

이전 source의 실제 결과는 다음과 같으며 새 source의 통과로 대신하지 않는다.

- 집중 검사 48개 PASS. 원본 #7518의 6→8쪽, 앞쪽 여백과 차트의 세 동일 실행물
  RED(exit 101)/GREEN(exit 0)도 확인했다.
- Native 169조건·384쪽과 #8188 네 입력·256쪽이 통과했다. 후자는 raw 100%와
  정수 미일치 0/0이다. 두 혼합 원본의 Native/fresh WASM 전체 32쪽도 98% 이상이다.
- release lib 자식 작업의 명시적 terminal summary는 3,983 PASS / 0 FAIL /
  12 ignored다. 리베이스 대기 전 부모만 종료했으므로 이 자식의 정확한 exit code는
  별도로 포착하지 않았다. 테스트 summary와 exit code를 혼동하지 않는다.
- 전체 nextest·후속 Skia·문서·Studio는 시작 전이었다. 새 base가 들어와 대기 작업을
  superseded로 기록했으며 실행 중인 Cargo·rustc는 종료하지 않았다.
- 이전 source의 광범위 WASM은 디스크 부족으로 기록된 127조건 중 117 PASS,
  10개 측정 불가 뒤 종료했다. fontTools와 journal 쓰기의 `OSError 28` 및 실제
  세션 exit 1을 확인했다. 이를 renderer 회귀나 전쪽 통과로 판정하지 않는다.

원본·PDF·글꼴·Git worktree가 없는 과거 렌더 중간 폴더, 사용이 끝난 실행물과
동일한 중복 raster만 정리했다. 현재 증적과 RED 실행물, 공유 target은 유지했고
약 9.1GB 여유를 확보했다.

## 충돌과 공통 결과의 통합

리베이스의 세 충돌은 다음과 같이 해소했다.

1. `reflow_empty_table_host`: 기존의 쪽/종이 기준 비-TAC 개체와 새 base의 글앞/글뒤
   비흐름 장식을 모두 빈 표 호스트 판정에서 제외한다. 두 helper를 현재 NoLS 경로에서
   함께 소비하며, 저장 줄 앵커의 다른 호출부로 조건을 넓히지 않는다.
2. `split_float_successor_spacing_before`와 `is_non_flow_decoration`: 같은 삽입 위치의
   독립 함수이므로 둘 다 유지한다.
3. `reflow_float_host_line`: 새 base의 장식 import와 기존 `neutral_anchor_metrics` 및
   `ResolvedParaStyle`을 모두 유지한다.

자동 병합 뒤 실제 소비 경로를 대조하니, 새 base가 표를 비흐름 제어에 포함하면서
기존 `neutral_anchor_metrics`도 NoLS overlay 표 호스트에 진입하게 됐다. 여기서는 첫
글자 모양을 선택하지만 새 base의 실제 layout은 `unstored_overlay_table_host_line_metrics`의
최대 글꼴을 선택한다. 두 선택을 독립적으로 유지하면 측정·조판·배치의 줄이 달라질 수 있다.

독립 근거는 #8116의 같은 원본·웹한글기안기 Print와 기존 NoLS 표 호스트 계약이다.
NoLS overlay 표 호스트만 새 base의 결과를 직접 소비하도록 통합했다. 공통 결과에
글꼴 크기를 함께 반환하고 layout, 빈 앵커 측정·조판·문단 layout이 같은 줄높이·줄간격·
글꼴 크기를 사용한다. 다른 부동 앵커의 기존 font 선택과 HWP3 cap은 유지한다.
문서 번호·임의 좌표·높이 clamp를 추가하지 않았다.

## 새 base 기능의 검증 범위

원 PR #8116이 devel에 추가한 다음 네 기존 원본·Print만 기능 보존 대상으로 확인한다.
새로운 사용자 문서를 요청하거나 별도 변형 HWP를 수집하지 않는다. 제공된 기준은
웹한글기안기 Print이며 MCP 2020/2024의 새 출력으로 바꿔 기록하지 않는다.

| 원본 (`samples/issue8080/`) | 독립 Print (`pdf/issue8080/`) | 전체 쪽수 |
| --- | --- | ---: |
| `cell_nested_table_rows.hwp` | `cell_nested_table_rows-webhwp-print.pdf` | 1 |
| `overlay_table_beside_title.hwp` | `overlay_table_beside_title-webhwp-print.pdf` | 5 |
| `picture_cell_row_height.hwp` | `picture_cell_row_height-webhwp-print.pdf` | 5 |
| `title_between_floating_shapes.hwp` | `title_between_floating_shapes-webhwp-print.pdf` | 4 |

기존 `issue_8080_no_lineseg_overlay_table_flow` 15개 검사를 실행하고, 새 base와 후보의
같은 원본·PDF·글꼴·DPI에서 Native/fresh WASM 전체 15쪽씩을 비교한다. 현재 개선한
두 혼합 원본과 차트·셀 그림의 정상 대조군도 먼저 확인한다. base 대비 페이지별 raw
하락 제한은 이 작업의 사용자 지시인 0.1%p이며, 정확히 90%는 통과한다.

이후 기존 169조건·384쪽과 두 혼합 원본, #8188 네 입력을 새 source에서 확인한다.
완료 페이지의 98% 자격도 유지해야 한다. source가 바뀌었으므로 이전 PNG/TSV를
현재 재출력으로 표시하지 않는다. Cargo는 공용 `target/pr-review`에서 8 jobs로
한 작업씩 실행하고 테스트 12 threads를 사용한다. 새 base·후보의 실제 검증, 전체
Rust·Skia·문서·Studio와 개별 리뷰 갱신이 끝나야 PR 준비 완료로 판정한다.
