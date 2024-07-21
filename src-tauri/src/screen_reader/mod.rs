use std::error::Error;

use opencv::core::{min_max_loc, Mat, Mat_AUTO_STEP, Point, CV_8UC4};
use opencv::{imgcodecs, imgproc};
use screenshots::image::RgbaImage;
use screenshots::Screen;

/// The threshold value used to determine if the screen contains the target image.
/// A value greater than this threshold indicates the target image was found on the screen.
const IMAGE_THRESHOLD: f64 = 0.9;

/// Checks if the screen contains the specified image.
///
/// This function captures the primary screen, converts it to an OpenCV Mat, and then performs template matching
/// to check if the specified image file is present on the screen. The function returns `true` if the image is
/// found on the screen, and `false` otherwise.
///
/// # Arguments
/// * `file_path` - The path to the image file to search for on the screen.
///
/// # Returns
/// * `Result<bool, Box<dyn Error>>` - A `Result` indicating whether the image was found on the screen (`true`)
///   or not (`false`), or an `Error` if there was a problem capturing the screen or performing the template
///   matching.
pub fn screen_contains_image(file_path: &str) -> Result<bool, Box<dyn Error>> {
    // Capture the primary screen
    let screen = Screen::all()?;
    let primary_screen = screen.first().ok_or("No primary screen found")?;
    let image_screen = primary_screen.capture()?;

    // Convert the captured screen to an OpenCV Mat
    let screen_mat = rgba_image_to_mat(&image_screen)?;

    // Read the template image
    let template_mat = imgcodecs::imread(file_path, imgcodecs::IMREAD_UNCHANGED)?;
    let mut result = Mat::default();

    // Perform template matching
    imgproc::match_template(
        &screen_mat,
        &template_mat,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &Mat::default(),
    )?;

    // Find the minimum and maximum values in the result
    let mut min_val = 0.0;
    let mut max_val = 0.0;
    let mut min_loc = Point::default();
    let mut max_loc = Point::default();

    min_max_loc(
        &result,
        Some(&mut min_val),
        Some(&mut max_val),
        Some(&mut min_loc),
        Some(&mut max_loc),
        &Mat::default(),
    )?;

    Ok(max_val > IMAGE_THRESHOLD)
}

/// Converts an RGBA image to an OpenCV Mat.
///
/// # Arguments
/// * `rgba_image` - The RGBA image to convert.
///
/// # Returns
/// A `Result` containing the converted OpenCV Mat, or an `Error` if there was a problem converting the image.
fn rgba_image_to_mat(rgba_image: &RgbaImage) -> Result<Mat, Box<dyn Error>> {
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
        )?
    };

    // Convert the color channels from RGBA to BGRA
    let mut mat_bgra = Mat::default();
    imgproc::cvt_color(&mat, &mut mat_bgra, imgproc::COLOR_RGBA2BGRA, 0)?;

    Ok(mat_bgra)
}
