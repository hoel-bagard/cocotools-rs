from typing import Literal, overload

import numpy as np
import numpy.typing as npt

from .anns import BBox, COCO_RLE, Polygons, PolygonsRS, RLE

@overload
def decode(encoded_mask: RLE | COCO_RLE | PolygonsRS,
           width: None = None,
           height: None = None,
           ) -> npt.NDArray[np.uint8]:
    ...

@overload
def decode(encoded_mask: Polygons,
           width: int,
           height: int,
           ) -> npt.NDArray[np.uint8]:
    ...

def decode(encoded_mask: Polygons | RLE | COCO_RLE | PolygonsRS,
           width: int | None,
           height: int | None,
           ) -> npt.NDArray[np.uint8]:
    """Decode an encoded mask.

    Args:
        encoded_mask: The mask to decode. It has to be one of the 4 types of mask provided by this package.
        width: If the encoded mask is of type Polygons (the format used by COCO),
               then the width of the image must be provided.
        height: If the encoded mask is of type Polygons (the format used by COCO),
               then the height of the image must be provided.

    Returns:
        The decoded mask as a numpy array.
    """
    ...

@overload
def encode(mask: npt.NDArray[np.uint8],
           target: Literal["rle"],
           ) -> RLE:
    ...

@overload
def encode(mask: npt.NDArray[np.uint8],
           target: Literal["coco_rle"],
           ) -> COCO_RLE:
    ...

@overload
def encode(mask: npt.NDArray[np.uint8],
           target: Literal["polygons_rs"],
           ) -> PolygonsRS:
    ...

@overload
def encode(mask: npt.NDArray[np.uint8],
           target: Literal["polygons"],
           ) -> Polygons:
    ...

def encode(mask: npt.NDArray[np.uint8],
           target: Literal["polygons"] | Literal["rle"] | Literal["coco_rle"] | Literal["polygons_rs"],
           ) -> Polygons | RLE | COCO_RLE | PolygonsRS:
    """Encode/compress a mask into the desired format.

    Args:
        mask: The mask to encode, it should be a 2 dimensional array.
        target: The desired format for the encoded mask.

    Returns:
        The encoded mask.
    """
    ...

def area(encoded_mask: RLE | COCO_RLE | PolygonsRS | Polygons) -> int:
    """Compute the area of the given mask.

    Args:
        encoded_mask: The mask whose area should be computed.

    Returns:
        The area
    """
    ...


def to_bbox(encoded_mask: RLE | COCO_RLE | PolygonsRS | Polygons) -> BBox:
    """Compute the bounding box of the given mask.

    Args:
        encoded_mask: The mask whose bounding box should be computed.

    Returns:
        The bounding box
    """
    ...
