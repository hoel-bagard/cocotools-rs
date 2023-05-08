use anyhow::{Context, Result};
use cocotools::errors::MaskError;
use numpy::ndarray::Array;
use numpy::ndarray::ShapeBuilder;
use numpy::IntoPyArray;
use numpy::PyArray2;
use numpy::PyReadonlyArray2;
use pyo3::prelude::*;
use pyo3::pyfunction;

use cocotools::coco::object_detection;
use cocotools::mask;
use cocotools::mask::conversions;

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
    m.add_function(wrap_pyfunction!(decode_coco_rle, m)?)?;
    m.add_function(wrap_pyfunction!(decode_poly_rs, m)?)?;
    m.add_function(wrap_pyfunction!(decode_poly, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_rle, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_coco_rle, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_polygons, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_polygons_rs, m)?)?;
    Ok(())
}

#[pyfunction]
fn decode_rle(py: Python<'_>, encoded_mask: object_detection::Rle) -> PyResult<&PyArray2<u8>> {
    Ok(decode(
        py,
        &object_detection::Segmentation::Rle(encoded_mask),
    )?)
}

#[pyfunction]
fn decode_coco_rle(
    py: Python<'_>,
    encoded_mask: object_detection::CocoRle,
) -> PyResult<&PyArray2<u8>> {
    Ok(decode(
        py,
        &object_detection::Segmentation::CocoRle(encoded_mask),
    )?)
}

#[pyfunction]
fn decode_poly_rs(
    py: Python<'_>,
    encoded_mask: object_detection::PolygonsRS,
) -> PyResult<&PyArray2<u8>> {
    Ok(decode(
        py,
        &object_detection::Segmentation::PolygonsRS(encoded_mask),
    )?)
}

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
fn decode_poly(
    py: Python<'_>,
    poly: object_detection::Polygons,
    width: u32,
    height: u32,
) -> Result<&PyArray2<u8>, PyMaskError> {
    let mask = conversions::mask_from_poly(&poly, width, height).map_err(PyMaskError::from)?;
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
#[allow(clippy::needless_pass_by_value)]
fn encode_to_rle(
    py: Python<'_>,
    mask: PyReadonlyArray2<u8>,
) -> PyResult<Py<object_detection::Rle>> {
    let mask = mask.to_owned_array();
    let encoded_mask = object_detection::Rle::from(&mask);
    Py::new(py, encoded_mask)
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
fn encode_to_coco_rle(
    py: Python<'_>,
    mask: PyReadonlyArray2<u8>,
) -> PyResult<Py<object_detection::CocoRle>> {
    let mask = mask.to_owned_array();
    let encoded_mask = object_detection::CocoRle::try_from(&mask).map_err(PyMaskError::from)?;
    Py::new(py, encoded_mask)
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
fn encode_to_polygons(
    py: Python<'_>,
    uncompressed_mask: PyReadonlyArray2<u8>,
) -> PyResult<Py<PyPolygons>> {
    let uncompressed_mask = uncompressed_mask.to_owned_array();
    let encoded_mask = PyPolygons(conversions::poly_from_mask(&uncompressed_mask));
    Py::new(py, encoded_mask)
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
fn encode_to_polygons_rs(
    py: Python<'_>,
    mask: PyReadonlyArray2<u8>,
) -> PyResult<Py<object_detection::PolygonsRS>> {
    let mask = mask.to_owned_array();
    let encoded_mask = object_detection::PolygonsRS::from(&mask);
    Py::new(py, encoded_mask)
}
