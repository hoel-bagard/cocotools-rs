use numpy::IntoPyArray;
use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::pyfunction;

use cocotools::annotations::coco;
use cocotools::converters::masks;

use crate::errors::PyMaskError;

#[pymodule]
pub fn mask(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_rle, m)?)?;
    m.add_function(wrap_pyfunction!(decode_encoded_rle, m)?)?;
    m.add_function(wrap_pyfunction!(decode_poly_rs, m)?)?;
    m.add_function(wrap_pyfunction!(decode_poly, m)?)?;
    Ok(())
}

#[pyfunction]
fn decode_rle(py: Python<'_>, encoded_mask: coco::Rle) -> PyResult<&PyArray2<u8>> {
    Ok(
        masks::Mask::try_from(&coco::Segmentation::Rle(encoded_mask))
            .map_err(PyMaskError::from)?
            .into_pyarray(py),
    )
}

#[pyfunction]
fn decode_encoded_rle(py: Python<'_>, encoded_mask: coco::EncodedRle) -> PyResult<&PyArray2<u8>> {
    Ok(
        masks::Mask::try_from(&coco::Segmentation::EncodedRle(encoded_mask))
            .map_err(PyMaskError::from)?
            .into_pyarray(py),
    )
}

#[pyfunction]
fn decode_poly_rs(py: Python<'_>, encoded_mask: coco::PolygonRS) -> PyResult<&PyArray2<u8>> {
    Ok(
        masks::Mask::try_from(&coco::Segmentation::PolygonRS(encoded_mask))
            .map_err(PyMaskError::from)?
            .into_pyarray(py),
    )
}

#[pyfunction]
fn decode_poly(
    py: Python<'_>,
    poly: coco::Polygon,
    width: u32,
    height: u32,
) -> PyResult<&PyArray2<u8>> {
    Ok(masks::mask_from_poly(&poly, width, height)
        .map_err(PyMaskError::from)?
        .into_pyarray(py))
}

// fn decode<T>(encoded_mask: T) -> masks::Mask
// where
//     T: TryInto<masks::Mask>,
//     <T as TryInto<masks::Mask>>::Error: std::fmt::Debug,
// {
//     match encoded_mask.try_into() {
//         Ok(mask) => mask,
//         Err(error) => panic!("Error when decoding mask: {:?}", error),
//     }
// }

// #[pyfunction]
// pub fn decode_rle(py: Python<'_>, encoded_mask: coco::Rle) -> &PyArray2<u8> {
//     decode(&coco::Segmentation::Rle(encoded_mask)).into_pyarray(py)
// }
