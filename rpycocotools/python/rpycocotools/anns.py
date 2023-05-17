"""Module providing COCO annotations classes."""
from typing import Generic, TypeVar

from _rpycocotools.anns import *

_TSegmentation = TypeVar("_TSegmentation", Polygons, PolygonsRS, RLE, COCO_RLE)


class Annotation(Annotation, Generic[_TSegmentation]):
    pass
