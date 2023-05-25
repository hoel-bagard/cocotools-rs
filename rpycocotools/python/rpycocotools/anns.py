"""Module providing COCO annotations classes."""
from typing import Generic, TypeVar

from _rpycocotools.anns import Annotation, BBox, Category, COCO_RLE, from_dataset, Image, Polygons, PolygonsRS, RLE

_TSegmentation = TypeVar("_TSegmentation", Polygons, PolygonsRS, RLE, COCO_RLE)


class Annotation(Annotation, Generic[_TSegmentation]):
    pass
