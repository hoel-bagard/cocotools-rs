from pathlib import Path

import rpycocotools
import pycocotools.coco
import pytest
from pytest_benchmark.fixture import BenchmarkFixture  # pyright: ignore[reportUnknownVariableType]



@pytest.fixture
def annotations_path() -> str:
    return "../data_samples/coco_25k/annotations.json"
    # return "../data_samples/instances_train2017.json"

def pycocotools_sum_dataset_masks_areas(annotations_path: Path | str):
    dataset = pycocotools.coco.COCO(str(annotations_path))
    total_area = 0
    for ann in dataset.anns.values():
        segmentation =  dataset.annToRLE(ann)
        total_area += pycocotools.mask.area(segmentation)
    assert int(total_area) == 293245

def rpycocotools_sum_dataset_masks_areas(annotations_path: Path | str):
    dataset = rpycocotools.COCO(str(annotations_path), "")
    total_area = 0
    for ann in dataset.get_anns():
        total_area += ann.area
    assert int(total_area) == 293163


def test_rpycocotools_area(benchmark: BenchmarkFixture, annotations_path: str) -> None:
    benchmark.pedantic(rpycocotools_sum_dataset_masks_areas, args=(annotations_path, ), rounds=500, iterations=1)


def test_pycocotools_area(benchmark: BenchmarkFixture, annotations_path: str) -> None:
    benchmark.pedantic(pycocotools_sum_dataset_masks_areas, args=(annotations_path, ), rounds=500, iterations=1)
