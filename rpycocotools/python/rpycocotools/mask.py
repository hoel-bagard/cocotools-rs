"""Module providing functions to decode and encode masks."""
from typing import Literal

import numpy as np
import numpy.typing as npt

from rpycocotools._rpycocotools import anns
from rpycocotools._rpycocotools import mask as _mask


def decode(encoded_mask: anns.RLE | anns.COCO_RLE | anns.PolygonsRS | anns.Polygons,
           width: None | int = None,
           height: None | int = None,
           ) -> npt.NDArray[np.uint8]:
    """Decode an encoded mask.

    Args:
        encoded_mask: The mask to decode. It has to be one of the 4 types of mask provided by this package.
        width: If the encoded mask of type Polygons (the format used by COCO),
               then the width of the image must be provided.
        height: If the encoded mask of type Polygons (the format used by COCO),
               then the height of the image must be provided.

    Returns:
        The decoded mask as a numpy array.
    """
    if isinstance(encoded_mask, anns.RLE):
        decoded_mask = _mask.decode_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.COCO_RLE):
        decoded_mask = _mask.decode_coco_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.PolygonsRS):
        decoded_mask = _mask.decode_poly_rs(encoded_mask)
    else:
        decoded_mask = _mask.decode_poly(encoded_mask, width=width, height=height)
    return decoded_mask


def encode(mask: npt.NDArray[np.uint8],
           target: Literal["rle", "coco_rle", "polygons", "polygon_rs"],
           ) -> anns.RLE | anns.COCO_RLE | anns.PolygonsRS | anns.Polygons:
    """Decode an encoded mask.

    Args:
        mask: The mask to encode, it should be a 2 dimensional array.
        target: The desired format for the encoded mask.

    Returns:
        The encoded mask.
    """
    match target:
        case "rle":
            encoded_mask = _mask.encode_to_rle(mask)
        case "coco_rle":
            encoded_mask = _mask.encode_to_coco_rle(mask)
        case "polygons":
            encoded_mask = _mask.encode_to_polygons(mask)
        case _:  # "polygons_rs"
            encoded_mask = _mask.encode_to_polygons_rs(mask)
    return encoded_mask
