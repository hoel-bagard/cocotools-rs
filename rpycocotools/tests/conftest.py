import pytest
import rpycocotools


@pytest.fixture()
def coco_dataset() -> rpycocotools.COCO:
    return rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
