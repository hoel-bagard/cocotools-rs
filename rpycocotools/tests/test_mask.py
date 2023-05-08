import numpy as np
import numpy.typing as npt
import pytest
import rpycocotools
from hypothesis import strategies as st
from rpycocotools import mask

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
