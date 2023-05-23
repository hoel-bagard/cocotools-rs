"""Module to benchmark how much time it takes to decode all the masks in a COCO dataset."""
import pycocotools.coco
import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import rpycocotools
from rpycocotools.anns import COCO_RLE, PolygonsRS, RLE


def pycocotools_decode_masks(dataset: pycocotools.coco.COCO) -> None:
    for ann in dataset.anns.values():
        segmentation = dataset.annToRLE(ann)
        pycocotools.mask.decode(segmentation)


def rpycocotools_decode_masks(dataset: rpycocotools.COCO) -> None:
    for img in dataset.get_imgs():
        for ann in dataset.get_img_anns(img.id):
            if isinstance(ann.segmentation, COCO_RLE | RLE | PolygonsRS):
                rpycocotools.mask.decode(ann.segmentation)
            else:
                rpycocotools.mask.decode(ann.segmentation, width=img.width, height=img.height)


@pytest.mark.coco2017()
def test_rpycocotools_decode_coco2017(
        benchmark: BenchmarkFixture,
        rpycocotools_coco2017_dataset: rpycocotools.COCO,
) -> None:
    benchmark.pedantic(
        rpycocotools_decode_masks,
        args=(rpycocotools_coco2017_dataset, ),
        rounds=1,
        iterations=1,
    )


@pytest.mark.coco2017()
def test_rpycocotools_decode_coco2017_rle(
        benchmark: BenchmarkFixture,
        rpycocotools_coco2017_dataset_rle: rpycocotools.COCO,
) -> None:
    benchmark.pedantic(
        rpycocotools_decode_masks,
        args=(rpycocotools_coco2017_dataset_rle, ),
        rounds=1,
        iterations=1,
    )


@pytest.mark.coco2017()
def test_pycocotools_decode_coco2017(
        benchmark: BenchmarkFixture,
        pycocotools_coco2017_dataset: pycocotools.coco.COCO,
) -> None:
    benchmark.pedantic(
        pycocotools_decode_masks,
        args=(pycocotools_coco2017_dataset, ),
        rounds=1,
        iterations=1,
    )


@pytest.mark.coco2017()
def test_pycocotools_decode_coco2017_rle(
        benchmark: BenchmarkFixture,
        pycocotools_coco2017_dataset_rle: pycocotools.coco.COCO,
) -> None:
    benchmark.pedantic(
        pycocotools_decode_masks,
        args=(pycocotools_coco2017_dataset_rle, ),
        rounds=1,
        iterations=1,
    )
