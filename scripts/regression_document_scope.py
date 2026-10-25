"""별도 PR에서 재등록할 문서의 회귀 fixture 수집 경계."""
from pathlib import Path

_NAMES = frozenset(
    line for line in
    (Path(__file__).resolve().parents[1] / "tests/fixtures/deferred_regression_documents.txt")
    .read_text(encoding="utf-8").splitlines()
    if line and not line.startswith("#")
)


def is_deferred_regression_input(path: str | Path) -> bool:
    name = str(path).replace("\\", "/").rsplit("/", 1)[-1]
    return any(name.startswith(value[:-1]) if value.endswith("*") else name == value
               for value in _NAMES)


def references_deferred_regression_input(text: str) -> bool:
    return any(name.removesuffix("*") in text for name in _NAMES)
