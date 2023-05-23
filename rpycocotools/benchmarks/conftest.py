import pytest


@pytest.fixture()
def sample_annotations_path() -> str:
    return "../data_samples/coco_25k/annotations.json"


@pytest.fixture()
def coco2017_annotations_path() -> str:
    return "../data_samples/instances_train2017.json"
