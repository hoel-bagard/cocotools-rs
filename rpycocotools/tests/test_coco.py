import pytest
import rpycocotools


@pytest.mark.xfail(reason="A dataset is readonly for now")
def test_modify_dataset_cat(coco_dataset: rpycocotools.COCO) -> None:
    assert coco_dataset.cats[1].name == "person"
    coco_dataset.cats[1].name = "elf"
    assert coco_dataset.cats[1].name == "elf"


@pytest.mark.xfail(reason="A dataset is readonly for now")
def test_set_dataset_cats(coco_dataset: rpycocotools.COCO) -> None:
    cat = rpycocotools.Category(1, "test", supercategory="super_test")
    coco_dataset.cats = {1: cat}
    assert coco_dataset.cats == {1: cat}


def test_access_cat(coco_dataset: rpycocotools.COCO) -> None:
    assert coco_dataset.cats[2].supercategory == "vehicle"
    assert coco_dataset.cats[2].id == 2
    assert coco_dataset.cats[2].name == "bicycle"
