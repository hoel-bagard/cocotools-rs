use numpy::ndarray::Array;
use numpy::ndarray::ShapeBuilder;
use numpy::IntoPyArray;
use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::pyfunction;

use cocotools::annotations::coco;
use cocotools::converters::masks;

use crate::errors::PyMaskError;

fn decode<T>(py: Python<'_>, encoded_mask: T) -> Result<&PyArray2<u8>, PyMaskError>
where
    T: TryInto<masks::Mask>,
    <T as TryInto<masks::Mask>>::Error: Into<PyMaskError>,
{
    match encoded_mask.try_into() {
        Ok(mask) => {
            let shape = (mask.shape()[1], mask.shape()[0]);
            let mask = mask.into_shape(shape).unwrap();
            let mask =
                Array::from_shape_vec(mask.raw_dim().f(), mask.t().iter().copied().collect())
                    .unwrap();
            let mask = mask.into_pyarray(py);
            Ok(mask)
        }
        Err(error) => Err(error.into()),
    }
}

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
    Ok(decode(py, &coco::Segmentation::Rle(encoded_mask))?)
}

#[pyfunction]
fn decode_encoded_rle(py: Python<'_>, encoded_mask: coco::EncodedRle) -> PyResult<&PyArray2<u8>> {
    Ok(decode(py, &coco::Segmentation::EncodedRle(encoded_mask))?)
}

#[pyfunction]
fn decode_poly_rs(py: Python<'_>, encoded_mask: coco::PolygonRS) -> PyResult<&PyArray2<u8>> {
    Ok(decode(py, &coco::Segmentation::PolygonRS(encoded_mask))?)
}

#[pyfunction]
fn decode_poly(
    py: Python<'_>,
    poly: coco::Polygon,
    width: u32,
    height: u32,
) -> PyResult<&PyArray2<u8>> {
    let mask = masks::mask_from_poly(&poly, width, height).map_err(PyMaskError::from)?;
    let shape = (mask.shape()[1], mask.shape()[0]);
    let mask = mask.into_shape(shape).unwrap();
    let mask =
        Array::from_shape_vec(mask.raw_dim().f(), mask.t().iter().copied().collect()).unwrap();
    Ok(mask.into_pyarray(py))
}
