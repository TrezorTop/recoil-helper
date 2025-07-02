use opencv::core::{min_max_loc, AlgorithmHint, Mat, Point};
use opencv::prelude::MatTraitConst;
use opencv::{imgcodecs, imgproc};

/// The threshold value used to determine if the screen contains the target image.
/// A value greater than this threshold indicates the target image was found on the screen.
const MAX_IMAGE_THRESHOLD: f64 = 0.95;

/// Reduces the number of gray shades in an image (color quantization)
///
/// # Arguments
/// * `src` - The source image (grayscale)
/// * `dst` - The destination image
/// * `num_levels` - The number of gray levels to reduce to
///
/// # Returns
/// * `Result<(), opencv::Error>` - Ok if successful, Err otherwise
fn quantize_image(src: &Mat, dst: &mut Mat, num_levels: i32) -> Result<(), opencv::Error> {
    // Calculate the step size between levels
    let max_value = 255.0;
    let step = max_value / (num_levels as f64 - 1.0);

    // Create a temporary matrix for intermediate results
    let mut temp = Mat::default();

    // Clone the source image to the destination
    src.copy_to(dst)?;

    // Apply thresholding for each level
    for i in 1..num_levels {
        let threshold_value = i as f64 * step;

        // Apply threshold
        imgproc::threshold(
            dst,
            &mut temp,
            threshold_value,
            threshold_value,
            imgproc::THRESH_TRUNC,
        )?;

        // Copy the result back to dst
        temp.copy_to(dst)?;
    }

    Ok(())
}

/// Matches a template image against a screen matrix
///
/// # Arguments
/// * `screen_mat` - The screen matrix to search in
/// * `image_path` - Path to the template image
///
/// # Returns
/// * `bool` - True if the template was found on the screen, false otherwise
pub fn match_template(screen_mat: &Mat, image_path: String) -> bool {
    // Read the template image
    let template_mat = match imgcodecs::imread(image_path.as_ref(), imgcodecs::IMREAD_UNCHANGED) {
        Ok(mat) => mat,
        Err(_) => return false, // Skip this image if it can't be read
    };

    // Convert to grayscale
    let mut gray_screen = Mat::default();
    let mut gray_template = Mat::default();

    if imgproc::cvt_color(
        screen_mat,
        &mut gray_screen,
        imgproc::COLOR_BGR2GRAY,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    )
    .is_err()
    {
        return false; // Skip if grayscale conversion fails
    }

    if imgproc::cvt_color(
        &template_mat,
        &mut gray_template,
        imgproc::COLOR_BGR2GRAY,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    )
    .is_err()
    {
        return false; // Skip if grayscale conversion fails
    }

    // Reduce the number of gray shades (color quantization)
    let mut quantized_screen = Mat::default();
    let mut quantized_template = Mat::default();

    // Define the number of gray levels we want (fewer levels = less color sensitivity)
    let num_levels = 3;

    // Apply quantization to screen image
    if quantize_image(&gray_screen, &mut quantized_screen, num_levels).is_err() {
        return false; // Skip if quantization fails
    }

    // Apply quantization to template image
    if quantize_image(&gray_template, &mut quantized_template, num_levels).is_err() {
        return false; // Skip if quantization fails
    }

    // Use the quantized images for template matching instead of the grayscale ones
    gray_screen = quantized_screen;
    gray_template = quantized_template;

    let mut result = Mat::default();

    // Perform template matching on grayscale images
    if imgproc::match_template(
        &gray_screen,
        &gray_template,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &Mat::default(),
    )
    .is_err()
    {
        return false; // Skip this image if template matching fails
    }

    // Find the minimum and maximum values in the result
    let mut min_val = 0.0;
    let mut max_val = 0.0;
    let mut min_loc = Point::default();
    let mut max_loc = Point::default();

    if min_max_loc(
        &result,
        Some(&mut min_val),
        Some(&mut max_val),
        Some(&mut min_loc),
        Some(&mut max_loc),
        &Mat::default(),
    )
    .is_err()
    {
        return false; // Skip this image if min_max_loc fails
    }

    println!("{:?} {:?} {:?}", min_val, max_val, image_path);

    // If the image is found on the screen, return true
    max_val > MAX_IMAGE_THRESHOLD
}