from typing import Generic, Iterator, Sequence, TypeAlias, TypeVar

from typing_extensions import Self

from . import COCO

_TSegmentation = TypeVar("_TSegmentation", Polygons, PolygonsRS, RLE, COCO_RLE)

class Annotation(Generic[_TSegmentation]):
    id: int
    image_id: int
    category_id: int
    segmentation: _TSegmentation
    area: float
    bbox: BBox
    iscrowd: int
    def __init__(
            self: Self,
            id: int,
            image_id: int,
            category_id: int,
            segmentation: _TSegmentation,
            area: float,
            bbox: BBox,
            iscrowd: int,
    ) -> None: ...

AnnotationAny: TypeAlias = Annotation[Polygons] | Annotation[PolygonsRS] | Annotation[RLE] | Annotation[COCO_RLE]

class Category:
    id: int
    name: str
    supercategory: str
    def __init__(self: Self, id: int, name: str, supercategory: str) -> None: ...

class BBox:
    left: float
    top: float
    width: float
    height: float
    def __init__(self: Self, left: float, top: float, width: float, height: float) -> None: ...
    def __iter__(self: Self) -> Iterator[float]: ...

class Image:
    id: int
    width: int
    height: int
    file_name: str
    def __init__(self: Self, id: int, width: int, height: int, file_name: str) -> None: ...

Polygons: TypeAlias = list[list[float]]

class PolygonsRS:
    size: list[int]
    counts: list[list[float]]
    def __init__(self: Self, size: list[int], counts: list[list[float]]) -> None: ...

class RLE:
    size: list[int]
    counts: list[int]
    def __init__(self: Self, size: list[int], counts: list[int]) -> None: ...

class COCO_RLE:  # noqa: N801
    size: list[int]
    counts: str
    def __init__(self: Self, size: list[int], counts: str) -> None: ...

def from_dataset(
        images: Sequence[Image],
        annotations: Sequence[AnnotationAny],
        categories: Sequence[Category],
        image_folder_path: str,
) -> COCO:
    """Construct a COCO dataset from its components and the image folder.

    Args:
        images: The image entries composing the data.
        annotations: The annotations entries composing the data.
        categories: The categories entries composing the data.
        image_folder_path: Path to the folder with the images.

    Errors:
        Will return `Err` if there is an annotation with an image id X, but no image entry has this id.
    """
    ...
