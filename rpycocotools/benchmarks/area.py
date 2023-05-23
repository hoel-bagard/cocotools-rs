"""Module to benchmark how much time it takes to compute the total number of mask pixels in a COCO dataset."""
from pathlib import Path

import pycocotools.coco
import pytest
from pytest_benchmark.fixture import BenchmarkFixture  # pyright: ignore[reportUnknownVariableType]

import rpycocotools


@pytest.fixture()
def sample_annotations_total_area() -> float:
    return 293163.80235


@pytest.fixture()
def coco2017_annotations_total_area() -> float:
    return 10342252910.807093


def pycocotools_sum_dataset_masks_areas(annotations_path: Path | str, expected_area: float) -> None:
    dataset = pycocotools.coco.COCO(str(annotations_path))
    total_area = 0
    for ann in dataset.anns.values():
        segmentation = dataset.annToRLE(ann)
        total_area += pycocotools.mask.area(segmentation)
    assert .99*expected_area < total_area < 1.01*expected_area


def rpycocotools_sum_dataset_masks_areas(annotations_path: Path | str, expected_area: float) -> None:
    dataset = rpycocotools.COCO(str(annotations_path), "")
    total_area = 0
    for ann in dataset.get_anns():
        total_area += ann.area
    assert .99*expected_area < total_area < 1.01*expected_area


def test_rpycocotools_area(
        benchmark: BenchmarkFixture,
        sample_annotations_path: str,
        sample_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        rpycocotools_sum_dataset_masks_areas,
        args=(sample_annotations_path, sample_annotations_total_area),
        rounds=500,
        iterations=1,
    )


def test_pycocotools_area(
        benchmark: BenchmarkFixture,
        sample_annotations_path: str,
        sample_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        pycocotools_sum_dataset_masks_areas,
        args=(sample_annotations_path, sample_annotations_total_area),
        rounds=500,
        iterations=1,
    )


@pytest.mark.coco2017()
def test_rpycocotools_area_coco2017(
        benchmark: BenchmarkFixture,
        coco2017_annotations_path: str,
        coco2017_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        rpycocotools_sum_dataset_masks_areas,
        args=(coco2017_annotations_path, coco2017_annotations_total_area),
        rounds=10,
        iterations=1,
    )


@pytest.mark.coco2017()
def test_pycocotools_area_coco2017(
        benchmark: BenchmarkFixture,
        coco2017_annotations_path: str,
        coco2017_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        pycocotools_sum_dataset_masks_areas,
        args=(coco2017_annotations_path, coco2017_annotations_total_area),
        rounds=10,
        iterations=1,
    )
