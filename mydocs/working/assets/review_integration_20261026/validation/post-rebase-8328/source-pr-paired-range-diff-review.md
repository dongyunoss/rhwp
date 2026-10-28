# 원 PR 재베이스 커밋 대조 — 2026-10-28

[원 head·patch-id 원장](source-pr-patch-refresh-20261028.json)에서 같은 제목의 이전·현재 커밋 13쌍을 `git range-diff <old>^! <new>^!`로 비교했다. [명령 결과](source-pr-paired-range-diff-20261028.json)는 각 전체 SHA와 출력을 보존한다. 명령은 모두 성공했다. 이 비교의 `!`는 수정된 patch를 뜻하며 구현 동등성이나 검증 통과를 단독으로 증명하지 않는다.

- #8202: 현재 브랜치의 도형 삽입은 `reflow_body_inline_object_change`를 호출한다. 최신 원 PR 커밋 `163a24ce99`과 현재 후보의 `shape.rs`, `common.rs`는 바이트 단위로 같다. 공용 호출은 문단 재조판과 후속 문단 위치 갱신을 차례로 수행한다. 원 PR 적용 확인과 시각 수용 판정은 별개다.
- #8205: 수식 캐시 키의 `legacy_source_frame_seal`과 `document_script::parse` 사용은 현재 브랜치에 있다. 원본 단위 검사는 후속 `c28d998091`에서 공개 API 검사 `tests/cases/equation_render_cache_contract.rs`로 옮겨졌다. 공개 API 검사는 script·fontSize·color·baseline을 다루며 모든 키 필드의 검사를 의미하지는 않는다.
- #8210·#8213: 새 기반의 저장 첫 행 프레임 fallback이 patch에 포함된 것을 확인했다. 현재 `table_layout.rs`의 대상 행은 우선 캐시 행에서 해당 행을 찾고 없으면 저장 첫 행 프레임에서 찾는다. 최신 원 PR은 먼저 행 배열을 선택한 뒤 해당 행을 찾는다. 두 경로의 행 범위를 확인했다. 전체 행 프레임이 `Some`이면 길이가 `table.row_count`와 같고(`stored_plain_object_row_frame_heights`), 첫 행 프레임은 그 이하로 잘린다(`stored_plain_opening_row_frame`). 그러므로 전체 프레임에서 없는 행은 첫 행 프레임에도 없어 두 순서 모두 `None`이다. 캐시도 같은 전체 행 함수를 호출한다. 이 조건하에서 평가 순서 차이는 선택 높이를 바꾸지 않는다. #8213 글꼴 접미사 fallback은 현재 최근 이름 캐시를 거치는 구현으로 존재한다. 접미사 fallback은 최신 원 PR의 직접 alias·색인 조회 대신 `resolve_metric_name(family)`를 사용한다. 해당 함수는 동일한 alias·색인 함수를 호출하고 캐시 키를 `Box<str>`로 소유하며 결과는 정적 alias·색인 참조를 담는다. 따라서 호출 수는 달라도 선택 alias·색인·굵기 조건은 같다.
- #8128, #8133, #8184의 기존 3개 패치와 #8283은 range-diff에 주변 코드·수집 경로 차이가 나타난다. 각 원 PR의 최신 기반과 현재 구현의 실제 호출·검사 결과를 대조하기 전에는 일괄 동등으로 판정하지 않는다.
- #8184의 추가 세 커밋 중 두 체리픽과 충돌 보존 범위는 [별도 구현 검토](../../../../../pr/archives/pr_8184_review_impl.md)에 기록한다. 이 기록은 전쪽 시각 검증을 대체하지 않는다.
