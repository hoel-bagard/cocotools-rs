from typing import Literal
import numpy as np
import numpy.typing as npt

from rpycocotools.rpycocotools import anns, mask as _mask


def decode(encoded_mask: anns.RLE | anns.EncodedRLE | anns.PolygonsRS | anns.Polygons, width: None | int = None, height: None | int = None):
    print(type(encoded_mask))
    if isinstance(encoded_mask, anns.RLE):
        decoded_mask = _mask.decode_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.EncodedRLE):
        decoded_mask = _mask.decode_encoded_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.PolygonsRS):
        decoded_mask = _mask.decode_poly_rs(encoded_mask)
    else:
        decoded_mask = _mask.decode_poly(encoded_mask, width=width, height=height)
    return decoded_mask


def encode(mask: npt.NDArray[np.uint8 | np.bool_], target = Literal["rle", "coco_rle", "polygons"]):
    match target:
        case "rle":
            encoded_mask =_mask.encode_to_rle(mask)
        case "coco_rle":
            pass
        case "polygons":
            pass
    return encoded_mask
