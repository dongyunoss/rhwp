# PR #8209 적용·보정 실행 기록

- 원 head: `01588f1e997b37795d2dbafecdf1f015341d553c`
- 통합 branch: `review/integration-20261026-document-updates`
- base: `0bb36e6f976e9ebeba0b1c48d86a2c5bdaee7f7c`
- 완료한 로컬 code/test 검증 head: `1cd656fdf9a09e88cd1ee9eb92f4c43358ea2559`
- 최종 판정·입력·명령·제한: [pr_8209_review.md](pr_8209_review.md)
- 단계 계획·독립 근거: [통합 계획·실패별 선행 분석·실제 소비 경로](<../../../mydocs/working/review_integration_20261026_document_updates_plan.md>)

## 실제 체리픽 순서

| 순서 | 원 SHA | 실제 적용 SHA / 처리 |
| ---: | --- | --- |
| 1 | `ef089bc668d64e02b10a596d927d9d3b0bc347a6` | `81dd6e099e770343f3ba7382654118c6462ac859` |
| 2 | `1091fb933df18e3171b34f048d0de24d4a054562` | `916048d29b7603305a4c22cf9bb80584e3c52e64` |
| 3 | `ca45a7162a9af5102b10ff941d92ab6625a4cfeb` | `a410ae27ab05af32132df4c086775ff36ebd8952` |
| 4 | `65f281399bae5457f021585532eaab2276e6540c` | `799894aecb949109f07a8e85de359d16c4cd0387` |
| 5 | `92c64aef7f4971a5b1fb563c7396c284f1e61a06` | `a22ed83a746c2f3995cba830d1accf309e85e0e6` |
| 6 | `01588f1e997b37795d2dbafecdf1f015341d553c` | `ca638217d50845db03df171a3ef4686e3911f3a6` |

적층 선행 commit은 patch-id로 대조했다. merge commit을 다시 체리픽하지 않았고 원 저자·`-x` 출처를 보존했다.
#8227·#8237의 사전 가상 충돌은 적층 부모를 명시한 예상 tree와 실제 결과가 일치했다. 실제 체리픽 충돌은 없었다.

## 보정·검증·후속 경계

1. 고정 원 head 적용과 예상 tree·중복을 원장으로 확인했다.
2. 승인된 기존 미달 페이지를 공통 로직에서 보정하고 단계별 일반 커밋으로 고정했다.
3. 관련 전쪽 Native/fresh WASM 선행 검증·직접 판독 뒤 정식 소유 회귀를 보강하고, 최종 최적화 WASM 전쪽·기준 대비 하락 기준을 검증했다.
4. 전체 필수 로컬 검증을 실제 완료한 뒤 원 PR별 이 기록을 작성했다.
5. 통합 PR을 제출한다. 이후 원격 CI·승인·머지·원 PR 종료는 별도 후속 조건이다.

보정 SHA: 이 PR 단독 보정 없음; 공통 통합 보정은 상위 계획 참조.

원 contributor head를 rewrite하거나 원격 브랜치를 변경하지 않았다. 되돌림은 필요 시 이 원장의 적용 commit과
관련 공통 보정을 검토해 별도 revert로 처리하며 다른 PR 변경까지 한꺼번에 롤백하지 않는다.
