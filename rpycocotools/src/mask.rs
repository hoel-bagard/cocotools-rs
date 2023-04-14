use anyhow::{Context, Result};
use cocotools::errors::MaskError;
use numpy::ndarray::Array;
use numpy::ndarray::ShapeBuilder;
use numpy::IntoPyArray;
use numpy::PyArray2;
use numpy::PyReadonlyArray2;
use pyo3::prelude::*;
use pyo3::pyfunction;

use cocotools::annotations::coco;
use cocotools::converters::mask;

use crate::coco::PyPolygons;
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
    m.add_function(wrap_pyfunction!(encode_to_rle, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_encoded_rle, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_polygons, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_polygons_rs, m)?)?;
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

#[pyfunction]
fn encode_to_rle(py: Python<'_>, mask: PyReadonlyArray2<u8>) -> PyResult<Py<coco::Rle>> {
    let mask = mask.to_owned_array();
    let encoded_mask = coco::Rle::from(&mask);
    Py::new(py, encoded_mask)
}

#[pyfunction]
fn encode_to_encoded_rle(
    py: Python<'_>,
    mask: PyReadonlyArray2<u8>,
) -> PyResult<Py<coco::EncodedRle>> {
    let mask = mask.to_owned_array();
    let encoded_mask = coco::EncodedRle::try_from(&mask).map_err(PyMaskError::from)?;
    Ok(Py::new(py, encoded_mask)?)
}

#[pyfunction]
fn encode_to_polygons(
    py: Python<'_>,
    uncompressed_mask: PyReadonlyArray2<u8>,
) -> PyResult<Py<PyPolygons>> {
    let uncompressed_mask = uncompressed_mask.to_owned_array();
    let encoded_mask = PyPolygons(mask::poly_from_mask(&uncompressed_mask));
    Py::new(py, encoded_mask)
}

#[pyfunction]
fn encode_to_polygons_rs(
    py: Python<'_>,
    mask: PyReadonlyArray2<u8>,
) -> PyResult<Py<coco::PolygonsRS>> {
    let mask = mask.to_owned_array();
    let encoded_mask = coco::PolygonsRS::from(&mask);
    Py::new(py, encoded_mask)
}
