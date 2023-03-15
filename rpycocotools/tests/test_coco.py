import pytest
import rpycocotools


@pytest.mark.xfail(reason="Not properly implemented yet")
def test_modify_dataset_cat(coco_dataset: rpycocotools.COCO):
    assert coco_dataset.cats[1].name == "person"
    coco_dataset.cats[1].name = "elf"
    assert coco_dataset.cats[1].name == "elf"


def test_set_dataset_cats(coco_dataset: rpycocotools.COCO):
    cat = rpycocotools.Category(1, "test", supercategory="super_test")
    coco_dataset.cats = {1: cat}
    assert coco_dataset.cats == {1: cat}


def test_access_cat(coco_dataset: rpycocotools.COCO):
    assert coco_dataset.cats[2].supercategory == "vehicle"
    assert coco_dataset.cats[2].id == 2
    assert coco_dataset.cats[2].name == "bicycle"
