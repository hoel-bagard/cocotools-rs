from rpycocotools.rpycocotools import anns, mask
# from rpycocotools.rpycocotools.anns import RLE, EncodedRle, PolygonRS, Polygons
# from rpycocotools.rpycocotools.mask import decode_rle, decode_encoded_rle, decode_poly_rs, decode_poly

# def decode():
#     return 1
def decode(encoded_mask: anns.Rle | anns.EncodedRle | anns.PolygonsRS | anns.Polygons, width: None | int = None, height: None | int = None):
    print(type(encoded_mask))
    if isinstance(encoded_mask, anns.Rle):
        decoded_mask = mask.decode_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.EncodedRle):
        decoded_mask = mask.decode_encoded_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.PolygonsRS):
        decoded_mask = mask.decode_poly_rs(encoded_mask)
    else:
        decoded_mask = mask.decode_poly(encoded_mask, width=width, height=height)
    return decoded_mask
