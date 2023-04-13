from rpycocotools.rpycocotools import anns, mask

def decode(encoded_mask: anns.RLE | anns.EncodedRLE | anns.PolygonsRS | anns.Polygons, width: None | int = None, height: None | int = None):
    print(type(encoded_mask))
    if isinstance(encoded_mask, anns.RLE):
        decoded_mask = mask.decode_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.EncodedRLE):
        decoded_mask = mask.decode_encoded_rle(encoded_mask)
    elif isinstance(encoded_mask, anns.PolygonsRS):
        decoded_mask = mask.decode_poly_rs(encoded_mask)
    else:
        decoded_mask = mask.decode_poly(encoded_mask, width=width, height=height)
    return decoded_mask
