Segmentation masks
==================

Mask types
----------
There are 3 ways a segmentation mask can be encoded in the annotations json file: Polygons, RLE or encoded RLE.
Examples of what each segmentation type looks like in the JSON file:
- `Polygons`: `"segmentation": [[510.66, 423.01, 511.72, 420.03, ..., 510.45, 423.01]]`
- `RLE`: `"segmentation": {"size": [40, 40], "counts": [245, 5, 35, 5, ..., 5, 35, 5, 1190]}`
- `EncodedRle`: `"segmentation": {"size": [480, 640], "counts": "aUh2b0X...BgRU4"}`

On top of those 3 segmentation types, this package introduces a fourth one called `PolygonsRS`.
It follows the same format as the `RLE` and `EncodedRle` types, but uses the polygons for the `counts` field: `{"size": [480, 640], "counts": [[510.66, 423.01, 511.72, 420.03, ..., 510.45, 423.01]]}`.
The advantage of this format if that he polygons can be decoded into a mask of the same as the input image without having to look up its size.

Mask conversions
----------------

.. function:: decode_rle(encoded_mask: Rle) -> npt.NDArray[np.uint8]

  Decode an RLE mask to a numpy array.

  :param Rle encoded_mask: The run-length encoded mask.
  :return: The decoded mask as a NumPy array.
  :rtype: npt.NDArray[np.uint8]

.. function:: decode_encoded_rle(encoded_mask: EncodedRle) -> npt.NDArray[np.uint8]

  Decode an encoded RLE mask to a numpy array.

  :param EncodedRle encoded_mask: The encoded run-length encoded mask.
  :return: The decoded mask as a NumPy array.
  :rtype: npt.NDArray[np.uint8]

.. function:: decode_poly_rs(encoded_mask: PolygonsRS) -> npt.NDArray[np.uint8]

  Decode a polygons mask (including image size) representation to a numpy array.

  :param PolygonsRS encoded_mask: The polygon in RLE format.
  :return: The decoded mask as a NumPy array.
  :rtype: npt.NDArray[np.uint8]

.. function:: decode_poly(poly: Polygons, width: int, height: int) -> npt.NDArray[np.uint8]

  Decode a polygons mask representation to a numpy array.

  :param Polygons poly: The `Polygons` to composing the mask.
  :param int width: The width of the image corresponding to the polygons
  :param int height: The height of the image corresponding to the polygons
  :return: The binary mask of the decoded `Polygons`.
  :rtype: npt.NDArray[np.uint8]
