COCO
====

.. class:: rpycocotools.COCO(annotation_path: str, image_folder_path: str) -> None

    Create the COCO dataset object from the annotation file and the image folder.

    :param str annotation_path: The path to the json annotation file.
    :param str image_folder_path: The path to the image folder.
    :raises ValueError: If the json file does not exist/cannot be read or if an error happens when deserializing and parsing it.
    :raises ValueError: If there is an annotation with an image id X, but no image entry has this id.

    .. method:: get_ann(ann_id: int) -> Annotation

    Return the `Annotation` corresponding to the given annotation id.

    :param int ann_id: The `id` of the `Annotation` to retrieve.
    :return: The annotation.
    :rtype: Annotation
    :raises KeyError: If there is no entry in the dataset corresponding to `ann_id`.

    .. method:: get_anns() -> list[Annotation]

    Return all the annotations of the dataset.

    :return: The annotations.
    :rtype: list[Annotation]

    .. method:: get_cat(cat_id: int) -> Category

    Return the `Category` corresponding to the given category id.

    :param int cat_id: The `id` of the `Category` to retrieve.
    :return: The category.
    :rtype: Category
    :raises KeyError: If there is no entry in the dataset corresponding to `cat_id`.

    .. method:: get_cats() -> list[Category]

    Return all the categories in the dataset.

    :return: The categories.
    :rtype: list[Category]

    .. method:: get_img(img_id: int) -> Image

    Return the `Image` corresponding to the given image id.

    :param int img_id: The `id` of the `Image` to retrieve.
    :return: The image.
    :rtype: Image
    :raises KeyError: If there is no entry in the dataset corresponding to `img_id`.

    .. method:: get_imgs() -> list[Image]

    Return all the image entries in the dataset.

    :return: The images.
    :rtype: list[Image]

    .. method:: get_img_anns(img_id: int) -> list[Annotation]

    Return the annotations for the given image id.

    :param int img_id: The `id` of the `Image` whose annotations should be retrieved.
    :return: The annotations for the image.
    :rtype: list[Annotation]
    :raises KeyError: If there is no entry in the dataset corresponding to `img_id`.

    .. method:: visualize_img(img_id: int) -> None

    Visualize an image and its annotations.

    :param int img_id: The `id` of the `Image` to whose annotations should be visualized.
    :raises ValueError: If the image cannot be drawn (potentially due to it not being in the dataset) or cannot be displayed.


.. class:: rpycocotools.anns.Annotation(id: int, image_id: int, category_id: int, segmentation: Polygons | PolygonsRS | Rle | CocoRle, area: float, bbox: Bbox, iscrowd: int) -> None

    Create an annotation used for object detection tasks.

    Each object instance annotation contains a series of fields, including the category id and segmentation mask of the object.\
    In [the original COCO dataset](https://cocodataset.org/#home), the segmentation format depends on whether the instance represents a single object (`iscrowd=0` in which case polygons are used) or a collection of objects (`iscrowd=1` in which case RLE is used). Note that a single object (iscrowd=0) may require multiple polygons, for example if occluded.\
    Crowd annotations (`iscrowd=1`) are used to label large groups of objects (e.g. a crowd of people). In addition, an enclosing bounding box is provided for each object (box coordinates are measured from the top left image corner and are 0-indexed).\
    Finally, the categories field of the annotation structure stores the mapping of category id to category and supercategory names.

    :param int id: The id of the annotation.
    :param int image_id: The id of the image corresponding to this annotation.
    :param int category_id: The id of the category corresponding to this annotation.
    :param Polygons | PolygonsRS | Rle | CocoRle segmentation: The segmentation data for the annotation, which can be of type Polygons, PolygonsRS, Rle or CocoRle.
    :param float area: The area of the annotation bounding box.
    :param Bbox bbox: The bounding box of the annotation.
    :param int iscrowd: The iscrowd flag for the annotation, which indicates if the annotation represents a group of objects or not.

.. class:: rpycocotools.anns.Category(id: int, name: str, supercategory: str) -> None

    Creates a category used for COCO object detection tasks.

    :param int id: The id of the category.
    :param str name: The name of the category.
    :param str supercategory: The supercategory of the category.

    .. attribute:: id

        The id of the category.

        :type: int

    .. attribute:: name

        The name of the category.

        :type: str

    .. attribute:: supercategory

        The supercategory of the category.

        :type: str

.. class:: rpycocotools.anns.Bbox(left: float, top: float, width: float, height: float) -> None

    A bounding box used for object detection tasks.

    :param float left: The top-left x coordinate of the bounding box.
    :param float top: The top-left y coordinate of the bounding box.
    :param float width: The width of the bounding box.
    :param float height: The height of the bounding box.

    .. attribute:: left

        The top-left x coordinate of the bounding box.

        :type: float

    .. attribute:: top

        The top-left y coordinate of the bounding box.

        :type: float

    .. attribute:: width

        The width of the bounding box.

        :type: float

    .. attribute:: height

        The height of the bounding box.

        :type: float

.. class:: rpycocotools.anns.Image(id: int, width: int, height: int, file_name: str) -> None

    A COCO image entry.

    :param int id: The id of the image.
    :param int width: The width of the image.
    :param int height: The height of the image.
    :param str file_name: The file name of the image.

    .. attribute:: id

        The id of the image.

        :type: int

    .. attribute:: width

        The width of the image.

        :type: int

    .. attribute:: height

        The height of the image.

        :type: int

    .. attribute:: file_name

        The file name of the image.

        :type: str

.. class:: rpycocotools.anns.PolygonsRS(size: list[int], counts: list[list[float]]) -> None

    Polygon(s) representing a segmentation mask.
    A Segmentation mask might require multiple polygons if the mask is in multiple parts (in case of partial occlusion for example).

    :param list[int] size: List with two elements, the height and width of the image associated to the segmentation mask.
    :param list[list[float]] counts`:
      Each list[float] represents an enclosed area belonging to the segmentation mask.
      The length of each list must be even. Every 2*n value represents the x coordinates of the nth point, while the 2*n+1 represents its y coordinates.

    .. attribute:: size

        List with two elements, the height and width of the image associated to the segmentation mask.

        :type: list[int]

    .. attribute:: counts

        The polygons that constitute the mask.

        :type: list[list[float]]

.. class:: rpycocotools.anns.RLE(size: list[int], counts: list[int]) -> None

    Segmentation mask compressed as a [Run-Length Encoding](https://en.wikipedia.org/wiki/Run-length_encoding).

    :param list[int] size: List with two elements, the height and width of the image corresponding to the segmentation mask.
    :param list[int] counts: The rle representation of the mask.

    .. attribute:: size

        List with two elements, the height and width of the image corresponding to the segmentation mask.

        :type: list[int]

    .. attribute:: counts

        The RLE representation of the mask.

        :type: list[int]

.. class:: rpycocotools.anns.COCO_RLE(size: list[int], counts: str) -> None

    Segmentation mask compressed as a [Run-Length Encoding](https://en.wikipedia.org/wiki/Run-length_encoding) and then further encoded into a string.

    :param list[int] size: List with two elements, the height and width of the image corresponding to the segmentation mask.
    :param str counts: The COCO RLE representation of the mask.

    .. attribute:: size

        List with two elements, the height and width of the image corresponding to the segmentation mask.

        :type: list[int]

    .. attribute:: counts

        The COCO RLE representation of the mask.

        :type: str
