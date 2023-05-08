Segmentation masks
==================

Mask types
----------
There are 3 ways a segmentation mask can be encoded in the annotations json file: :py:class:`Polygons`, :py:class:`RLE` or :py:class:`COCO_RLE`.
Examples of what each segmentation type looks like in the JSON file:

* :py:class:`Polygons`: `"segmentation": [[510.66, 423.01, 511.72, 420.03, ..., 510.45, 423.01]]`
* :py:class:`RLE`: `"segmentation": {"size": [40, 40], "counts": [245, 5, 35, 5, ..., 5, 35, 5, 1190]}`
* :py:class:`COCO_RLE`: `"segmentation": {"size": [480, 640], "counts": "aUh2b0X...BgRU4"}`

On top of those 3 segmentation types, this package introduces a fourth one called :py:class:`PolygonsRS`.
It follows the same format as the :py:class:`RLE` and :py:class:`COCO_RLE` types, but uses the polygons for the `counts` field:

* :py:class:`PolygonsRS`: `"segmentation": {"size": [480, 640], "counts": [[510.66, 423.01, 511.72, 420.03, ..., 510.45, 423.01]]}`

The advantage of this format if that he polygons can be decoded into a mask of the same as the input image without having to look up its size. However it should not be written to a json file (as it is non-standard).

Decode masks
----------------

.. function:: rpycocotools.mask.decode(encoded_mask: RLE | COCO_RLE | Polygons | PolygonsRS, width: None | int, height: None | int) -> npt.NDArray[np.uint8]

  Decode a mask to a :class:`numpy.ndarray`.

  :param RLE | COCO_RLE | Polygons | PolygonsRS encoded_mask: The encoded mask.
  :param int width: Use only when the mask is a :py:class:`Polygon`. The width of the image corresponding to the polygons.
  :param int height: Use only when the mask is a :py:class:`Polygon`. The height of the image corresponding to the polygons.
  :raise ValueError: If the mask conversion failed.
  :return: The decoded mask as a NumPy array.
  :rtype: ``npt.NDArray[np.uint8]``


Encode masks
----------------

.. function:: rpycocotools.mask.encode(mask: npt.NDArray[np.uint8], target: Literal["polygons"] | Literal["rle"] | Literal["coco_rle"] | Literal["polygons_rs"]) -> Polygons | RLE | COCO_RLE | PolygonsRS:

  Encode/compress a :class:`numpy.ndarray` mask to the desired format.

  :param npt.NDArray[np.uint8] encoded_mask: The uncompressed mask.
  :raise ValueError: If the mask conversion failed.
  :return: The compressed mask.
  :rtype: ``Polygons | RLE | COCO_RLE | PolygonsRS``
