# Dependabot 21개 PR 통합 체리픽 검토 계획

- 작업 브랜치: `review/integration-20261026-dependabot-updates-r2`
- 기준: 최신 `upstream/devel` `01ced3d42724285c8f9765a453393ab6af326617`
- 사용자 지시: Dependabot PR을 체리픽 통합한 뒤 검토한다. 봇이므로 개별 PR 문서는 만들지 않는다.
- 개별 review 문서를 요구하는 일반 규칙 대신 이 계획과 통합 PR의 검토 기록 한 묶음으로 각 PR의
  원 head·적용 SHA·충돌·검증·판정을 추적한다. 별도 대표 이슈가 없어 날짜 기반 integration 이름을 사용한다.
- 원 PR 21개 모두 `jangster77` reviewer를 지정하고 API의 users에서 확인했다. 이 gate 뒤 상세 metadata와 patch를 수집했다.
- 원 head는 변경하지 않는다. 아래 고정 SHA를 번호 순서대로 `git cherry-pick -x`한다.

## 고정한 원 PR과 범위

| 원 PR | 고정 head SHA | 변경 파일 |
| --- | --- | --- |
| [#8241](https://github.com/edwardkim/rhwp/pull/8241) | `6abde855288dad72b42be68ac9f2a25fc5c29867` | `rhwp-studio/package-lock.json`, `rhwp-studio/package.json` |
| [#8242](https://github.com/edwardkim/rhwp/pull/8242) | `ea4ee557ec1dddeb99219f90a7774c2201ef80c6` | `Cargo.lock`, `Cargo.toml` |
| [#8243](https://github.com/edwardkim/rhwp/pull/8243) | `d9f5713786817202a400dfac553be5f503065bb7` | `rhwp-vscode/package-lock.json`, `rhwp-vscode/package.json` |
| [#8244](https://github.com/edwardkim/rhwp/pull/8244) | `d579b8d47929346e89e2d812ce15838402cd9421` | `rhwp-studio/package-lock.json`, `rhwp-studio/package.json` |
| [#8245](https://github.com/edwardkim/rhwp/pull/8245) | `97894d388b61344334c012480a2bdf52bcd7576a` | `Cargo.lock` |
| [#8246](https://github.com/edwardkim/rhwp/pull/8246) | `6a205a14f5890b2cde35ee10b4d2d606c9c0f252` | `rhwp-chrome/package-lock.json`, `rhwp-chrome/package.json` |
| [#8247](https://github.com/edwardkim/rhwp/pull/8247) | `1c2f11fbefea498251f45b19eba2fb0e3ecd245c` | `rhwp-firefox/package-lock.json`, `rhwp-firefox/package.json` |
| [#8248](https://github.com/edwardkim/rhwp/pull/8248) | `8612331ffb3167a77dbf55e8a338f74a063e98b5` | `Cargo.lock`, `Cargo.toml` |
| [#8249](https://github.com/edwardkim/rhwp/pull/8249) | `f0a9c0c71eaa7bd00b91d9603e78debfeae3329a` | `rhwp-studio/package-lock.json`, `rhwp-studio/package.json` |
| [#8250](https://github.com/edwardkim/rhwp/pull/8250) | `be55d654a85340383771e4d7b5725f56f7cddd4c` | `rhwp-chrome/package-lock.json`, `rhwp-chrome/package.json` |
| [#8251](https://github.com/edwardkim/rhwp/pull/8251) | `4886fb003aa40a5a9075ce61cc06d749f3a3b8e6` | `Cargo.lock` |
| [#8252](https://github.com/edwardkim/rhwp/pull/8252) | `b284c60df1cf3fad0d69c478be103f4a3c41f2c4` | `rhwp-studio/package-lock.json`, `rhwp-studio/package.json` |
| [#8253](https://github.com/edwardkim/rhwp/pull/8253) | `b9f9a7b9361bc4de771d2c8e575acb838036de24` | `Cargo.lock` |
| [#8254](https://github.com/edwardkim/rhwp/pull/8254) | `81abf49cd2a1ade119e5ae22d24b3cc75d5966b5` | `.github/workflows/release-binary.yml`, `.github/workflows/release-operations-contracts.yml`, `.github/workflows/skill-router-gate.yml` |
| [#8255](https://github.com/edwardkim/rhwp/pull/8255) | `b6646a38774d54fcb3f8bf91ca5543020027052c` | `Cargo.lock` |
| [#8256](https://github.com/edwardkim/rhwp/pull/8256) | `97b73c91bb54853ff1930874241931a8c595f634` | `Cargo.lock` |
| [#8257](https://github.com/edwardkim/rhwp/pull/8257) | `45b87dcac8a8cb794c798a368c46cd1fafd884e4` | `Cargo.lock` |
| [#8258](https://github.com/edwardkim/rhwp/pull/8258) | `37df0d004768590dd531c718b075bd86be38fceb` | `.github/workflows/codeql.yml` |
| [#8259](https://github.com/edwardkim/rhwp/pull/8259) | `cc6a5a10e972502160cb2a6dd9e75250d0ae36c4` | `.github/workflows/codeql.yml` |
| [#8260](https://github.com/edwardkim/rhwp/pull/8260) | `2f2c5f3b9ae9a0c75863dc2007f83f19aa89142f` | `.github/workflows/build-nextest-archives.yml`, `.github/workflows/run-nextest-archives.yml` |
| [#8261](https://github.com/edwardkim/rhwp/pull/8261) | `101f2ae74f552f264ea6ac0c4904260858b5ca71` | `.github/workflows/adapter-diff.yml`, `.github/workflows/build-nextest-archives.yml`, `.github/workflows/ci.yml`, `.github/workflows/fuzz-smoke.yml`, `.github/workflows/layout-anomaly-advisory.yml`, `.github/workflows/oracle-public-advisory.yml`, `.github/workflows/proptest-roundtrip.yml` |

## 검증과 판정

1. 각 고정 ref를 fetch하고 실제 SHA를 확인한 뒤 오래된 번호부터 누적 적용한다. 적용 SHA·순서와
   conflict 및 보정은 이 기록에 남긴다. 이미 merged된 source 변경을 되돌리지 않는다.
2. Rust 8개, npm 8개, Actions 5개의 업데이트와 공식 변경 근거를 확인한다. read-fonts/skera의
   버전·타입 연결과 Anthropic SDK major 변경은 실제 호출부·빌드·행동 검사로 확인한다.
3. 관련 모든 lockfile의 정합성, action pin, Cargo lint/focused/full nextest·Native 및
   npm TypeScript·단위·extension/Studio 실제 browser 경로를 변경 범위대로 검증한다.
4. 폰트·PDF 의존성이 실제 조판/출력을 바꾸는지 Native/fresh WASM으로 확인한다. 기존 sample16
   세 문서 각 64쪽의 100%와 독립 Print·입력/PDF·기존 회귀 기대값을 보존한다.
   실패를 없애기 위해 검사·공차·문서 출력 기준을 완화하지 않는다.
5. 코드 보정과 필수 검증을 마친 후보를 통합 PR로 제출하고 정확한 최신 head CI를 확인한다.
   봇 원 PR별 문서 없이 통합 검토에서 수용/보류와 적용·보정 SHA를 구분한다.

## 이전 작업과 자원 경계

#8188·#8273·옵션 2 문서 #8274는 이미 merged이고 devel에 포함됐다. 원본·PDF·768개 PNG·
개인 글꼴과 복구 자료를 보존하고, 승인된 이전 작업 branch/worktree/output 정리로 검증 공간을 확보한다.
공유 `/home/tsjang/rhwp/target/pr-review`와 사용자·다른 작업 자료는 보존한다.
문서 전용 merge에서도 metadata job의 checkout이 실행되는 점은 확인해 사용자에게 보고했으며,
이번 의존성 업데이트와 섞어 수정하지 않는다.

## 재시작 기록

사용자가 임시 output을 삭제한 뒤 처음부터 다시 체리픽하도록 지시했다. 원 PR 21개의 metadata와
patch를 재수집했고 reviewer 지정·각 head SHA가 모두 동일함을 확인했다.
기존 통합 branch는 사용자 지시로 삭제하며, 기존 head `9f6c373ad246e3454325a71eaa0ae10c7d48533f`의
patch는 로컬 영구 복구 경로에 보관했다. 새 branch에서 21개 원 commit을 다시 번호 순서로 적용한다.
