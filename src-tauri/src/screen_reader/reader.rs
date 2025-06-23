use opencv::core::{min_max_loc, Mat, Point};
use opencv::{imgcodecs, imgproc};

/// The threshold value used to determine if the screen contains the target image.
/// A value greater than this threshold indicates the target image was found on the screen.
const IMAGE_THRESHOLD: f64 = 0.9;

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

    let mut result = Mat::default();

    // Perform template matching
    if imgproc::match_template(
        screen_mat,
        &template_mat,
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

    println!("{:?}", max_val);

    // If the image is found on the screen, return true
    max_val > IMAGE_THRESHOLD
}
