__version__ = "0.0.7"  # noqa: Y052, RUF100

import numpy as np
import numpy.typing as npt
from typing_extensions import Self

from . import mask  # noqa: F401  # pyright: ignore[reportUnusedImport]
from .anns import _AnnotationAny, Category, Image

class COCO:
    def __init__(self: Self, annotation_path: str, image_folder_path: str) -> None: ...
    def get_ann(self: Self, ann_id: int) -> _AnnotationAny: ...
    def get_anns(self: Self) -> list[_AnnotationAny]: ...
    def get_cat(self: Self, cat_id: int) -> Category: ...
    def get_cats(self: Self) -> list[Category]: ...
    def get_img(self: Self, img_id: int) -> Image: ...
    def get_imgs(self: Self) -> list[Image]: ...
    def get_img_anns(self: Self, img_id: int) -> list[_AnnotationAny]: ...
    def visualize_img(self: Self, img_id: int) -> None: ...
    def draw_anns(self: Self, img_id: int, draw_bboxes: bool) -> npt.NDArray[np.uint8]:
        """Draw the annotations on the image and returns it as a (RGB) numpy array."""
        ...
    def json(self: Self) -> str:
        """Return the dataset as a json string."""
        ...
    def __len__(self: Self) -> int:
        ...
