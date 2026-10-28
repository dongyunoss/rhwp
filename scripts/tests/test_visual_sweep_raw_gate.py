"""raw 미달을 색상 경계 보정값으로 통과시키지 않는 검토 계약."""

import csv
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from PIL import Image
from test_visual_sweep import SWEEP


class RawGateTests(unittest.TestCase):
    def test_adjusted_score_cannot_rescue_raw_failure(self):
        gate = SWEEP.pr_review_gate([{
            "page": 130, "silhouette_raw_match_percent": 79.94718,
            "tolerant_content_match_percent": 99.10104,
        }])
        self.assertEqual(gate["status"], "re_review_required")
        self.assertEqual(gate["metric"], "silhouette_raw_match_percent")
        self.assertIn("79.94718", SWEEP.review_gate_reason(gate))

    def test_exactly_ninety_raw_passes(self):
        gate = SWEEP.pr_review_gate([{
            "page": 1, "silhouette_raw_match_percent": 90.0,
            "tolerant_content_match_percent": 99.0,
        }])
        self.assertEqual(gate["status"], "passed")

    def test_adjusted_only_is_unmeasured(self):
        gate = SWEEP.pr_review_gate([{
            "page": 1, "tolerant_content_match_percent": 100.0,
        }])
        self.assertEqual(gate["status"], "re_review_required")
        self.assertEqual(gate["unavailable_metric_pages"], [1])

    def test_threshold_boundary_is_diagnostic_in_tsv_and_labels(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            native, pdf = root / "rhwp_001.png", root / "pdf-001.png"
            Image.new("RGB", (8, 8), (231, 231, 231)).save(native)
            Image.new("RGB", (8, 8), (233, 233, 233)).save(pdf)
            result = SWEEP.write_silhouette_tsv([(1, native, pdf)], root, "boundary")
            with (root / "silhouette.tsv").open(encoding="utf-8") as stream:
                row = next(csv.DictReader(stream, delimiter="\t"))
            self.assertEqual(float(row["tolerant_content_match_percent"]), 100.0)
            self.assertEqual(float(row["silhouette_raw_match_percent"]), 0.0)
            self.assertEqual(row["below_90"], "1")
            self.assertEqual(result["pr_review_gate"]["status"], "re_review_required")
            label = SWEEP.review_comment_line(result["metrics"][0])
            self.assertIn("판정값 raw 실루엣(2px 이웃) = 0.00%", label)
            self.assertNotIn("100.00%", label)

    def test_overlay_label_matches_raw_tsv_not_adjusted_score(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            native, pdf = root / "rhwp_001.png", root / "pdf-001.png"
            Image.new("RGB", (16, 16), (231, 231, 231)).save(native)
            Image.new("RGB", (16, 16), (233, 233, 233)).save(pdf)
            overlay = SWEEP.make_overlay_page(native, pdf, root / "overlay_001.png",
                                               "boundary", 0, pixel_diff_threshold=32)
            result = SWEEP.write_silhouette_tsv([(1, native, pdf)], root, "boundary")
            with (root / "silhouette.tsv").open(encoding="utf-8") as stream:
                row = next(csv.DictReader(stream, delimiter="\t"))
            label = SWEEP.review_comment_line(overlay)
            self.assertIn(f"= {row['silhouette_raw_match_percent']}%", label)
            self.assertNotIn("100.00%", label)
            self.assertEqual(result["metrics"][0]["silhouette_raw_match_percent"],
                             overlay["silhouette_raw_match_percent"])

    def test_rounded_tsv_does_not_change_precise_metrics_or_gate(self):
        cases = [
            (1, {"silhouette_raw_match_percent": 89.99911,
                 "tolerant_content_match_percent": 99.99911,
                 "silhouette_boundary_reconciled_pixels": 1}, {"page": 1}),
            (2, {"silhouette_raw_match_percent": 99.99911,
                 "tolerant_content_match_percent": 100.0,
                 "silhouette_boundary_reconciled_pixels": 1}, {"page": 2}),
        ]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            pairs = [(1, root / "a", root / "b"), (2, root / "c", root / "d")]
            with patch.object(SWEEP, "ordered_page_map", return_value=iter(cases)):
                manifest = SWEEP.write_silhouette_tsv(pairs, root, "rounding")
            with (root / "silhouette.tsv").open(encoding="utf-8") as stream:
                rows = list(csv.DictReader(stream, delimiter="\t"))
            self.assertEqual(rows[0]["silhouette_raw_match_percent"], "90.00")
            self.assertEqual(rows[0]["below_90"], "1")
            self.assertEqual(rows[1]["silhouette_raw_match_percent"], "100.00")
            self.assertEqual(manifest["metrics"][0]["silhouette_raw_match_percent"], 89.99911)
            self.assertEqual(manifest["metrics"][1]["silhouette_raw_match_percent"], 99.99911)
            self.assertEqual(manifest["pr_review_gate"]["status"], "re_review_required")


if __name__ == "__main__":
    unittest.main()
