use std::error::Error;

use opencv::core::{AlgorithmHint, Mat, Mat_AUTO_STEP, CV_8UC4};
use opencv::imgproc;
use screenshots::image::RgbaImage;

use crate::screen_reader::error::ScreenReaderError;

/// Converts an RGBA image to an OpenCV Mat.
///
/// # Arguments
/// * `rgba_image` - The RGBA image to convert.
///
/// # Returns
/// A `Result` containing the converted OpenCV Mat, or an `Error` if there was a problem converting the image.
pub fn rgba_image_to_mat(rgba_image: &RgbaImage) -> Result<Mat, Box<dyn Error>> {
    // Get the dimensions of the image
    let (width, height) = (rgba_image.width() as i32, rgba_image.height() as i32);

    // Create a Mat from the raw RGBA data
    let mat = unsafe {
        Mat::new_rows_cols_with_data_unsafe(
            height,
            width,
            CV_8UC4,
            rgba_image.as_raw().as_ptr() as *mut std::ffi::c_void,
            Mat_AUTO_STEP,
        ).map_err(|_| ScreenReaderError::ImageConversionFailed)?
    };

    // Convert the color channels from RGBA to BGRA
    let mut mat_bgra = Mat::default();
    imgproc::cvt_color(
        &mat,
        &mut mat_bgra,
        imgproc::COLOR_RGBA2BGRA,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    ).map_err(|_| ScreenReaderError::ImageConversionFailed)?;

    Ok(mat_bgra)
}