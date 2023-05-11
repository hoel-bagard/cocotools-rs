import numpy as np
import numpy.typing as npt
import pytest
from hypothesis import strategies as st

import rpycocotools
from rpycocotools import mask
from rpycocotools.anns import BBox, PolygonsRS

u32_max = 4_294_967_295
u32_st = st.integers(min_value=0, max_value=u32_max)


def test_access_mask(coco_dataset: rpycocotools.COCO) -> None:
    ann = coco_dataset.get_ann(1348739)
    assert isinstance(ann.segmentation, rpycocotools.anns.PolygonsRS)
    mask = rpycocotools.mask.decode(ann.segmentation)
    assert np.sum(mask) == 423


def test_create_mask() -> None:
    rpycocotools.anns.RLE(size=[4, 4], counts=[5, 2, 2, 2, 5])
    rpycocotools.anns.COCO_RLE(size=[4, 4], counts="52203")
    rpycocotools.anns.PolygonsRS(size=[7, 7], counts=[[2.0, 1.0, 2.0, 5.0, 4.0, 5.0, 4.0, 1.0]])
    rpycocotools.anns.Polygons([[2.0, 1.0, 2.0, 5.0, 4.0, 5.0, 4.0, 1.0]])


# @pytest.mark.xfail(reason="Not implemented")
@pytest.mark.parametrize(("rle", "expected_coco_rle"),
                         [(rpycocotools.anns.RLE(size=[4, 4], counts=[5, 2, 2, 2, 5]),
                           rpycocotools.anns.COCO_RLE(size=[4, 4], counts="52203")),
                          ])
def test_convert_mask(rle: rpycocotools.anns.RLE, expected_coco_rle: rpycocotools.anns.COCO_RLE) -> None:
    decoded_mask = mask.decode(rle)
    coco_rle = mask.encode(decoded_mask, target="coco_rle")
    assert coco_rle == expected_coco_rle


@pytest.mark.parametrize(("rle", "expected_mask"),
                         [(rpycocotools.anns.RLE(size=[7, 7], counts=[15, 5, 2, 5, 2, 5, 15]),
                           np.asarray([[0, 0, 0, 0, 0, 0, 0],
                                       [0, 0, 1, 1, 1, 0, 0],
                                       [0, 0, 1, 1, 1, 0, 0],
                                       [0, 0, 1, 1, 1, 0, 0],
                                       [0, 0, 1, 1, 1, 0, 0],
                                       [0, 0, 1, 1, 1, 0, 0],
                                       [0, 0, 0, 0, 0, 0, 0]])),
                          ])
def test_decode_rle(rle: rpycocotools.anns.RLE, expected_mask: npt.NDArray[np.uint8]) -> None:
    decoded_mask = mask.decode(rle)
    assert np.all(decoded_mask == expected_mask)


def test_import() -> None:
    from rpycocotools.mask import decode, encode  # noqa: F401 # pyright: ignore[reportUnusedImport]


@pytest.mark.parametrize(("segmentation", "expected_area"),
                         [
                             (PolygonsRS(size=[480, 640], counts=[[273.25, 300.22, 270.58, 293.11, 266.72, 291.03, 264.65, 283.32, 265.54, 278.58, 266.13, 266.13, 277.4, 239.75, 279.47, 235.89, 280.36, 235.3, 348.24, 235.6, 365.73, 255.46, 372.55, 263.46, 373.74, 264.35, 375.22, 271.17, 375.22, 278.28, 373.15, 290.44, 367.22, 292.51, 366.33, 285.7, 350.62, 290.74, 350.02, 296.07, 346.47, 299.92, 343.21, 300.52, 340.54, 296.37, 285.4, 295.18, 284.51, 300.81, 276.51, 302.59]]),  # noqa: E501
                              6040),  # 5876.319200000001),
                             (PolygonsRS(size=[388, 640], counts=[[180.56, 115.83, 191.85, 114.96, 198.79, 107.15, 217.02, 106.28, 219.62, 113.23, 229.17, 113.23, 232.64, 104.55, 238.72, 101.07, 240.45, 92.39, 236.98, 85.45, 224.83, 82.85, 221.36, 76.77, 209.21, 72.43, 176.22, 71.56, 162.33, 71.56, 157.13, 81.98, 151.92, 84.58, 148.45, 93.26, 148.45, 108.89, 148.45, 112.36, 162.33, 109.75, 162.33, 105.41, 174.49, 106.28, 178.83, 113.23]]),  # noqa: E501
                              3167),  # 3033.8891499999995),
                          ])
def test_compute_area(segmentation: PolygonsRS, expected_area: float) -> None:
    area = rpycocotools.mask.area(segmentation)
    assert area == int(expected_area)


@pytest.mark.parametrize(("segmentation", "expected_bbox"),
                         [
                             (PolygonsRS(size=[480, 640], counts=[[273.25, 300.22, 270.58, 293.11, 266.72, 291.03, 264.65, 283.32, 265.54, 278.58, 266.13, 266.13, 277.4, 239.75, 279.47, 235.89, 280.36, 235.3, 348.24, 235.6, 365.73, 255.46, 372.55, 263.46, 373.74, 264.35, 375.22, 271.17, 375.22, 278.28, 373.15, 290.44, 367.22, 292.51, 366.33, 285.7, 350.62, 290.74, 350.02, 296.07, 346.47, 299.92, 343.21, 300.52, 340.54, 296.37, 285.4, 295.18, 284.51, 300.81, 276.51, 302.59]]),  # noqa: E501
                              BBox(left=264.65, top=235.3, width=110.57, height=67.29)),
                             (PolygonsRS(size=[388, 640], counts=[[180.56, 115.83, 191.85, 114.96, 198.79, 107.15, 217.02, 106.28, 219.62, 113.23, 229.17, 113.23, 232.64, 104.55, 238.72, 101.07, 240.45, 92.39, 236.98, 85.45, 224.83, 82.85, 221.36, 76.77, 209.21, 72.43, 176.22, 71.56, 162.33, 71.56, 157.13, 81.98, 151.92, 84.58, 148.45, 93.26, 148.45, 108.89, 148.45, 112.36, 162.33, 109.75, 162.33, 105.41, 174.49, 106.28, 178.83, 113.23]]),  # noqa: E501
                              BBox(left=148.45, top=71.56, width=92, height=44.27)),
                          ])
def test_to_bbox(segmentation: PolygonsRS, expected_bbox: BBox) -> None:
    bbox = rpycocotools.mask.to_bbox(segmentation)
    assert bbox == expected_bbox
