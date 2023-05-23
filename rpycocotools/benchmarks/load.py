"""Module to benchmark how much time it takes to load a COCO dataset."""
from pathlib import Path

import pycocotools.coco
import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import rpycocotools


def rpycocotools_load_dataset(annotations_path: Path | str) -> None:
    rpycocotools.COCO(str(annotations_path), "")


def pycocotools_load_dataset(annotations_path: Path | str) -> None:
    pycocotools.coco.COCO(str(annotations_path))


@pytest.mark.coco2017()
def test_rpycocotools_area_coco2017(benchmark: BenchmarkFixture, coco2017_annotations_path: str) -> None:
    benchmark.pedantic(rpycocotools_load_dataset, args=(coco2017_annotations_path, ), rounds=10, iterations=1)


@pytest.mark.coco2017()
def test_pycocotools_area_coco2017(benchmark: BenchmarkFixture, coco2017_annotations_path: str) -> None:
    benchmark.pedantic(pycocotools_load_dataset, args=(coco2017_annotations_path, ), rounds=10, iterations=1)
