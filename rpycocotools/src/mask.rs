use numpy::IntoPyArray;
use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::pyfunction;

use cocotools::annotations::coco;
use cocotools::converters::masks;

fn decode<T>(encoded_mask: T) -> masks::Mask
where
    T: TryInto<masks::Mask>,
    <T as TryInto<masks::Mask>>::Error: std::fmt::Debug,
{
    match encoded_mask.try_into() {
        Ok(mask) => mask,
        Err(error) => panic!("Error when parsing: {:?}", error),
    }
}

#[pyfunction]
pub fn decode_rle(py: Python<'_>, encoded_mask: coco::Rle) -> &PyArray2<u8> {
    decode(&coco::Segmentation::Rle(encoded_mask)).into_pyarray(py)
}

#[pyfunction]
pub fn decode_poly(py: Python<'_>, encoded_mask: coco::PolygonRS) -> &PyArray2<u8> {
    decode(&coco::Segmentation::PolygonRS(encoded_mask)).into_pyarray(py)
}
