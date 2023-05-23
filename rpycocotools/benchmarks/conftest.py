import pytest

import rpycocotools
import pycocotools.coco


@pytest.fixture()
def sample_annotations_path() -> str:
    return "../data_samples/coco_25k/annotations.json"


@pytest.fixture()
def coco2017_annotations_path() -> str:
    return "../data_samples/instances_train2017.json"


@pytest.fixture()
def pycocotools_sample_dataset(sample_annotations_path: str) -> pycocotools.coco.COCO:
    return pycocotools.coco.COCO(sample_annotations_path)


@pytest.fixture()
def pycocotools_coco2017_dataset(coco2017_annotations_path: str) -> pycocotools.coco.COCO:
    return pycocotools.coco.COCO(coco2017_annotations_path)


@pytest.fixture()
def rpycocotools_sample_dataset(sample_annotations_path: str) -> rpycocotools.COCO:
    return rpycocotools.COCO(sample_annotations_path, "")


@pytest.fixture()
def rpycocotools_coco2017_dataset(coco2017_annotations_path: str) -> rpycocotools.COCO:
    return rpycocotools.COCO(coco2017_annotations_path, "")
