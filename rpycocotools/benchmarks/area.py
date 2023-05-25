"""Module to benchmark how much time it takes to compute the total number of mask pixels in a COCO dataset."""
import pycocotools.coco
import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import rpycocotools


@pytest.fixture()
def sample_annotations_total_area() -> float:
    return 293163.80235


@pytest.fixture()
def coco2017_annotations_total_area() -> float:
    return 10342252910.807093


def pycocotools_sum_dataset_masks_areas(dataset: pycocotools.coco.COCO, expected_area: float) -> None:
    total_area = 0
    for ann in dataset.anns.values():
        segmentation = dataset.annToRLE(ann)
        total_area += pycocotools.mask.area(segmentation)
    assert .99*expected_area < total_area < 1.01*expected_area


def rpycocotools_sum_dataset_masks_areas(dataset: rpycocotools.COCO, expected_area: float) -> None:
    total_area = sum([ann.area for ann in dataset.get_anns()])
    assert .99*expected_area < total_area < 1.01*expected_area


def test_rpycocotools_area(
        benchmark: BenchmarkFixture,
        rpycocotools_sample_dataset: rpycocotools.COCO,
        sample_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        rpycocotools_sum_dataset_masks_areas,
        args=(rpycocotools_sample_dataset, sample_annotations_total_area),
        rounds=500,
        iterations=10,
    )


def test_pycocotools_area(
        benchmark: BenchmarkFixture,
        pycocotools_sample_dataset: pycocotools.coco.COCO,
        sample_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        pycocotools_sum_dataset_masks_areas,
        args=(pycocotools_sample_dataset, sample_annotations_total_area),
        rounds=500,
        iterations=10,
    )


@pytest.mark.coco2017()
def test_rpycocotools_area_coco2017(
        benchmark: BenchmarkFixture,
        rpycocotools_coco2017_dataset: rpycocotools.COCO,
        coco2017_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        rpycocotools_sum_dataset_masks_areas,
        args=(rpycocotools_coco2017_dataset, coco2017_annotations_total_area),
        rounds=10,
        iterations=1,
    )


@pytest.mark.coco2017()
def test_pycocotools_area_coco2017(
        benchmark: BenchmarkFixture,
        pycocotools_coco2017_dataset: pycocotools.coco.COCO,
        coco2017_annotations_total_area: float,
) -> None:
    benchmark.pedantic(
        pycocotools_sum_dataset_masks_areas,
        args=(pycocotools_coco2017_dataset, coco2017_annotations_total_area),
        rounds=10,
        iterations=1,
    )
