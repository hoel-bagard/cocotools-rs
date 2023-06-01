import pycocotools.coco
import pytest

import rpycocotools
from rpycocotools.anns import Annotation, COCO_RLE, PolygonsRS, RLE


@pytest.fixture()
def sample_annotations_path() -> str:
    return "../data_samples/coco_25k/annotations.json"


@pytest.fixture()
def coco2017_annotations_path() -> str:
    # return "../data_samples/instances_train2017.json"
    return "../data_samples/val2017/instances_val2017.json"


@pytest.fixture()
def pycocotools_sample_dataset(sample_annotations_path: str) -> pycocotools.coco.COCO:
    return pycocotools.coco.COCO(sample_annotations_path)


@pytest.fixture()
def rpycocotools_sample_dataset(sample_annotations_path: str) -> rpycocotools.COCO:
    return rpycocotools.COCO(sample_annotations_path, "")


@pytest.fixture()
def pycocotools_coco2017_dataset(coco2017_annotations_path: str) -> pycocotools.coco.COCO:
    return pycocotools.coco.COCO(coco2017_annotations_path)


@pytest.fixture()
def pycocotools_coco2017_dataset_rle(pycocotools_coco2017_dataset: pycocotools.coco.COCO) -> pycocotools.coco.COCO:
    for ann_id, ann in pycocotools_coco2017_dataset.anns.items():
        rle = pycocotools_coco2017_dataset.annToRLE(ann)
        pycocotools_coco2017_dataset.anns[ann_id]["segmentation"] = rle  # pyright: ignore[reportGeneralTypeIssues]
    for img_id, anns_list in pycocotools_coco2017_dataset.imgToAnns.items():
        rle_anns_list = []
        for ann in anns_list:
            ann["segmentation"] = pycocotools_coco2017_dataset.annToRLE(ann)  # pyright: ignore[reportGeneralTypeIssues]
            rle_anns_list.append(ann)
        pycocotools_coco2017_dataset.imgToAnns[img_id] = rle_anns_list
    return pycocotools_coco2017_dataset


@pytest.fixture()
def rpycocotools_coco2017_dataset(coco2017_annotations_path: str) -> rpycocotools.COCO:
    return rpycocotools.COCO(coco2017_annotations_path, "")


@pytest.fixture()
def rpycocotools_coco2017_dataset_rle(rpycocotools_coco2017_dataset: rpycocotools.COCO) -> rpycocotools.COCO:
    new_anns = []
    for img in rpycocotools_coco2017_dataset.get_imgs():
        for ann in rpycocotools_coco2017_dataset.get_img_anns(img.id):
            if isinstance(ann.segmentation, COCO_RLE | RLE | PolygonsRS):
                mask = rpycocotools.mask.decode(ann.segmentation)
            else:
                mask = rpycocotools.mask.decode(ann.segmentation, width=img.width, height=img.height)
            rle = rpycocotools.mask.encode(mask, target="rle")
            new_anns.append(Annotation(ann.id, ann.image_id, ann.category_id, rle, ann.area, ann.bbox, ann.iscrowd))
    return rpycocotools.anns.from_dataset(
        rpycocotools_coco2017_dataset.get_imgs(),
        new_anns,
        rpycocotools_coco2017_dataset.get_cats(),
        "",
    )
