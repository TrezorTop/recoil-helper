use opencv::core::{min_max_loc, Mat, Mat_AUTO_STEP, Point, CV_8UC4};
use opencv::{imgcodecs, imgproc};
use screenshots::image::RgbaImage;
use screenshots::Screen;

pub fn screen_contains_image() {
    let template_path = "../resources/images/m4.png";
    let test_path = "../resources/images/test.png";

    let screen = Screen::all().unwrap();
    let primary_screen = screen.first().unwrap();
    let image_screen = primary_screen.capture().unwrap();

    let screen_mat = rgba_image_to_mat(&image_screen);
    let test_mat = imgcodecs::imread(test_path, imgcodecs::IMREAD_UNCHANGED).unwrap();

    let params = opencv::core::Vector::new();

    let template_mat = imgcodecs::imread(template_path, imgcodecs::IMREAD_UNCHANGED).unwrap();

    println!("test_mat {:#?}", test_mat);
    imgcodecs::imwrite("test_mat.png", &test_mat, &params).unwrap();
    println!("screen_mat {:#?}", screen_mat);
    imgcodecs::imwrite("screen_mat.png", &screen_mat, &params).unwrap();
    println!("template_mat {:#?}", template_mat);
    imgcodecs::imwrite("template_mat.png", &template_mat, &params).unwrap();

    let mut result = Mat::default();

    imgproc::match_template(
        &screen_mat,
        &template_mat,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &Mat::default(),
    )
    .unwrap();

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
    )
    .unwrap();

    println!("{:?}", max_val);
}

fn rgba_image_to_mat(rgba_image: &RgbaImage) -> Mat {
    // Get the dimensions of the image
    let (width, height) = (rgba_image.width() as i32, rgba_image.height() as i32);

    unsafe {
        // Create a Mat from the raw RGBA data
        let mut mat = Mat::new_rows_cols_with_data_unsafe(
            height,
            width,
            CV_8UC4,
            rgba_image.as_raw().as_ptr() as *mut std::ffi::c_void,
            Mat_AUTO_STEP,
        )
        .unwrap();

        // Convert the color channels from RGBA to BGRA
        let mut mat_bgra = Mat::default();
        imgproc::cvt_color(&mat, &mut mat_bgra, imgproc::COLOR_RGBA2BGRA, 0).unwrap();

        mat_bgra
    }
}
