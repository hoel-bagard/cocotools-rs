use anyhow::{Context, Result};
use cocotools::errors::MaskError;
use numpy::ndarray::Array;
use numpy::ndarray::ShapeBuilder;
use numpy::IntoPyArray;
use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::pyfunction;

use cocotools::annotations::coco;
use cocotools::converters::mask;

use crate::errors::PyMaskError;

fn decode<T>(py: Python<'_>, encoded_mask: T) -> Result<&PyArray2<u8>, PyMaskError>
where
    mask::Mask: TryFrom<T>,
    <mask::Mask as TryFrom<T>>::Error: Into<PyMaskError>,
{
    match mask::Mask::try_from(encoded_mask) {
        Ok(mask) => {
            let shape = (mask.shape()[1], mask.shape()[0]);
            let mask = mask
                .into_shape(shape)
                .with_context(|| {
                        "Could not reshape the mask from shape when doing post process to convert to numpy array.".to_string()
                })
                .map_err(MaskError::Other)?;
            let mask = Array::from_shape_vec(
                mask.raw_dim().f(),
                mask.t().iter().copied().collect(),
            )
            .with_context(|| {
                "Could not convert the mask to fortran array when converting to numpy array."
                    .to_string()
            })
            .map_err(MaskError::Other)?;
            let mask = mask.into_pyarray(py);
            Ok(mask)
        }
        Err(error) => Err(error.into()),
    }
}

#[allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]
#[pymodule]
#[pyo3(name = "mask")]
pub fn py_mask(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
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
fn decode_poly_rs(py: Python<'_>, encoded_mask: coco::PolygonsRS) -> PyResult<&PyArray2<u8>> {
    Ok(decode(py, &coco::Segmentation::PolygonsRS(encoded_mask))?)
}

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
fn decode_poly(
    py: Python<'_>,
    poly: coco::Polygons,
    width: u32,
    height: u32,
) -> Result<&PyArray2<u8>, PyMaskError> {
    let mask = mask::mask_from_poly(&poly, width, height).map_err(PyMaskError::from)?;
    let shape = (mask.shape()[1], mask.shape()[0]);
    let mask = mask.into_shape(shape).with_context(|| {
            "Could not reshape the mask from shape when doing post process to convert to numpy array.".to_string()
        })
        .map_err(MaskError::Other)?;
    let mask = Array::from_shape_vec(mask.raw_dim().f(), mask.t().iter().copied().collect())
        .with_context(|| {
            "Could not convert the mask to fortran array when converting to numpy array."
                .to_string()
        })
        .map_err(MaskError::Other)?;
    Ok(mask.into_pyarray(py))
}
