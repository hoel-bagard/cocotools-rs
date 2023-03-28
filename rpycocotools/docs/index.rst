Welcome to rpycocotools's documentation!
========================================
The `rpycocotools` package provides tools to load, manipulate, convert and visualize COCO format datasets.

This package aims to provide similar functionalities to the [pycocotools package](https://pypi.org/project/pycocotools/) / [cocoapi](https://github.com/cocodataset/cocoapi) with additionnal utilities such as conversion between dataset formats. It also aims to have a better documentation and a more readable implementation.

.. toctree::
    :maxdepth: 2
    :caption: Contents:

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`


Usage example
=============

.. code-block:: python
    :caption: Visualize image with a given `id`:

    import rpycocotools
    coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
    coco_dataset.visualize_img(174482)


COCO
----------------

.. class:: COCO(annotation_path: str, image_folder_path: str) -> None

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
