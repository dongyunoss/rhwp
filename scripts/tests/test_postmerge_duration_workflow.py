"""#7070: no validation workflow may subscribe to devel branch pushes."""
import pathlib
import re
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / '.github/workflows'

class PostmergeDurationWorkflowTests(unittest.TestCase):
    def test_refresh_push_skips_only_documentation_changes(self):
        text = (WORKFLOWS / 'refresh-nextest-duration.yml').read_text(encoding='utf-8')
        push = re.search(r'^  push:(.*?)(?=^  [a-z_]+:|^\S|\Z)', text, re.M | re.S)[1]
        self.assertIn('branches: [devel]', push)
        self.assertNotRegex(push, re.compile(r'^    paths:', re.M),
                            '제외 목록으로 알 수 없는 변경의 실행을 보존한다')
        ignored = re.search(r'^    paths-ignore:\n((?:^      .*\n)+)', push, re.M)
        self.assertIsNotNone(ignored)
        patterns = re.findall(r"^      - '([^']+)'$", ignored[1], re.M)

        def ignored_path(path):
            for pattern in patterns:
                glob = re.escape(pattern).replace(r'\*\*', '.*').replace(r'\*', '[^/]*')
                if re.fullmatch(glob, path):
                    return True
            return False

        def runs(paths):
            return any(not ignored_path(path) for path in paths)

        # 추가·수정·삭제된 문서, 문서 경로 간 이동 및 문서 증적 묶음.
        for paths in [
            ['README.md'], ['AGENTS.md', 'CLAUDE.md', 'llms.txt', 'LICENSE'],
            ['mydocs/manual/mcp_hwp2024Convert_usage.md'],
            ['mydocs/pr/assets/issue_8277/native.tsv', 'mydocs/pr/assets/issue_8277/review.png'],
            ['mydocs/old.md', 'mydocs/archive/new.md'],
        ]:
            with self.subTest(skipped=paths):
                self.assertFalse(runs(paths))
        # 혼합 변경, 코드에서 문서로 이동, 알 수 없는 경로는 실행을 유지한다.
        for path in ['src/lib.rs', 'Cargo.toml', 'Cargo.lock', 'tests/cases/example.rs',
                     '.github/workflows/refresh-nextest-duration.yml',
                     'scripts/collect-postmerge-duration-data.mjs', 'unknown/file',
                     'samples/example.hwpx', 'src/README.md']:
            with self.subTest(runs=path):
                self.assertTrue(runs([path]))
                self.assertTrue(runs(['mydocs/manual/example.md', path]))
        self.assertIn('\n  workflow_dispatch:\n', text)

    def test_devel_push_subscribers_are_metadata_only(self):
        # An allowlist makes newly introduced implicit/all-branch push triggers fail too.
        for path in WORKFLOWS.iterdir():
            if path.suffix not in {'.yml', '.yaml'}:
                continue
            text = path.read_text()
            match = re.search(r'^  push:(.*?)(?=^  [a-z_]+:|^\S|\Z)', text, re.M | re.S)
            if not match:
                continue
            block = match[1]
            if re.search(r'branches:\s*\[main\]', block) or ('tags:' in block and 'branches:' not in block):
                continue
            if path.name == 'oracle-public-advisory.yml':
                self.assertNotIn('devel', block)
                self.assertIn("'task_m100_*'", block)
                continue
            self.assertIn(path.name, ['refresh-nextest-duration.yml', 'close-issues-on-devel-push.yml'])

    def test_pr_checks_and_tag_release_are_preserved(self):
        for filename in ['ci.yml', 'codeql.yml', 'adapter-diff.yml', 'proptest-roundtrip.yml']:
            text = (WORKFLOWS / filename).read_text()
            self.assertIn('  pull_request:', text)
            self.assertIn('  workflow_dispatch:', text)
            if filename != 'ci.yml':
                self.assertNotIn('\n  push:', text)
        self.assertIn("tags: ['v*']", (WORKFLOWS / 'ci.yml').read_text())

    def test_refresh_has_only_metadata_job_and_no_ci_fallback(self):
        text = (WORKFLOWS / 'refresh-nextest-duration.yml').read_text()
        jobs = text.split('\njobs:\n', 1)[1]
        self.assertEqual(re.findall(r'^  ([\w-]+):', jobs, re.M), ['refresh-nextest-target-duration-data'])
        for forbidden in ['cargo ', 'npm ', 'workflow_run:', 'actions: write', 'trusted-postmerge-ci-reuse', 'gh run', 'workflow-dispatch', 'pull_request:']:
            self.assertNotIn(forbidden, text)
        self.assertIn('branches: [devel]', text)
        self.assertIn("steps.collect.outputs.ready == 'true'", text)
        self.assertIn('"${current_devel_sha}" != "${GITHUB_SHA}"', text)
        self.assertIn('HEAD:refs/heads/${METRICS_BRANCH}', text)
        self.assertIn('collectPostmergeDurations', text)
        collector = (ROOT / 'scripts/collect-postmerge-duration-data.mjs').read_text()
        for forbidden in ['createWorkflowDispatch', 'reRunWorkflow', 'execFile', 'extractall']:
            self.assertNotIn(forbidden, collector)
