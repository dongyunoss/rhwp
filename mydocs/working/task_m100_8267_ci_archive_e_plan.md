# #8267 CI Archive E 분산과 지침 문서 fast-pass 계획

- 이슈: [#8267](https://github.com/edwardkim/rhwp/issues/8267)
- 작업 브랜치: `task/8267-nextest-archive-e`
- 시작 base: `upstream/devel`의 `4e690c930105fa09fef484787ed0a2e5e182d444`
- 범위: GitHub 운영 O3. 제품 Rust·렌더링·sample16 기준 자료는 변경하지 않는다.
- 사용자 지시: 새 브랜치에서 구현·로컬 검증·개선 PR을 준비한다. 모든 PR 머지는 대기한다.

## 원인과 선행 근거

PR #8188의 CI 실행 `40276896106`, job `121425844331`은 Archive C의
3,604개 검사 중 3,603개가 통과한 상태에서 worker의 30분 제한으로 취소됐다.
실패 assertion은 없었다. 사용자에 의한 failed jobs 재실행은 별도이며 중복 실행하지 않는다.
기존 duration selector는 integration target을 B/C/D에 배정하고, 집계·trusted evidence·갱신도
세 integration archive를 전제한다. E 실행만 추가하면 누락·중복 또는 증거 불일치가 생긴다.
`AGENTS.md`·`CLAUDE.md`는 impact classifier에는 이미 문서로 등록돼 있으나 workflow의
review-only 허용 경로와 trusted policy에는 빠져 있다.

## 단계와 완료 조건

1. Archive A의 lib 분리를 유지하고 B/C/D/E로 integration target을 배정한다. E builder와
   worker, 각 archive의 예상 검사 수·실행 수 대조, required job 감사, duration 생산·소비를
   함께 수정한다. worker의 유한 timeout은 30분에서 60분으로 늘린다.
2. 루트 `AGENTS.md`·`CLAUDE.md`의 문서 변경을 각 workflow와 trusted policy에서 일관되게
   허용한다. 코드와 섞인 변경은 Full CI를 유지하고 rename의 이전 경로도 검사한다.
3. 실제 Cargo metadata와 현재 manifest를 이용해 배정의 전체 포함·서로 배타성·결정성을 확인한다.
   affected Node/Python 계약, actionlint, duration 수집·부분 재실행·위조 거부 및 fast-pass의
   양성·음성 사례를 실행한다. 실패는 원인을 보정하고 관련 검증을 다시 실행한다.
4. 단계별 일반 커밋에 구현과 검사 결과를 기록한다. 최종 diff와 한국어 PR 본문을 준비하고
   승인된 범위에서 제출한다. 머지는 수행하지 않는다.

## 증적 및 보존

임시 실행 로그는 Git 제외 `output/pr-review/issue8267/`에 저장한다.
기존 #8160 브랜치 `task/8160-sample16-raw100`과 head
`a01381d0e2a29f94dcdc567319fd61ade08039db`, PDF·TSV·폰트 증적 및 공유
`target/pr-review`는 보존한다. 과거 세 archive 실측의 호환 여부는 명시적으로 검사하며
새 네 archive 실행에서 E 누락을 허용하지 않는다.

## 1단계 결과: Archive E와 실행 제한

- A는 lib 전용으로 유지하고 B/C/D/E로 integration target을 분산했다. E builder·worker,
  예상 수와 실제 수 집계, CI Impact Policy의 REST alias 감사, release promotion 증거를 연결했다.
- worker 제한은 60분으로 변경했다. builder는 기존 profile별 30/60분 정책을 유지한다.
- duration 생산·정규화·발행·갱신은 E를 포함한다. 부분 재실행에서는 worker의 원래 attempt를
  대조하고, 이전 B/C/D 실행은 trusted Jobs API에 E가 없는 경우에 한해 명시적 구 layout으로
  검증한다. 새 layout의 E artifact 누락은 거부한다.
- 로컬 전체 nextest 명령은 동일하다. 분할 selector의 `integration-e`, duration 수집·갱신과
  workflow 계약을 갱신했으며, 기존 captured API JSON은 수정하지 않았다.
- 검증: Archive 관련 Node 311개 통과. 이후 역사적 layout 검사를 추가한 duration 수집기
  43개 통과(추가 1개 포함). Python 61개 통과. 변경한 5개 workflow actionlint 통과.
- 실제 Cargo metadata의 integration target을 현재 source manifest로 배정하고, 네 집합의
  전체 포함·중복 없음·결정성·비어 있지 않음과 각 CLI 출력을 대조했다. E의 검사 수 불일치,
  artifact 누락, builder/worker 실패·취소·skip은 실제 aggregate shell에서 실패함을 확인했다.
- 기존 1,000줄 초과 policy 파일은 기존 신뢰 로딩 구조를 보존하기 위해 job identity 최소
  추가만 수행했다. 새 코드 파일은 1,000줄 이하로 유지한다.
