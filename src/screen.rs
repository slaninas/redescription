use opencv::prelude::*;

pub const ACTUAL_PIXEL_COUNT: (u32, u32) = (426, 240);
pub const CROP_SIZE: (u32, u32) = (42, 27);

pub const ACTUAL_PIXEL_COUNT_WITHOUT_CROP: (u32, u32) = (
    ACTUAL_PIXEL_COUNT.0 - 2 * CROP_SIZE.0,
    ACTUAL_PIXEL_COUNT.1 - 2 * CROP_SIZE.1,
);

#[cfg(windows)]
pub fn get_capturer() -> captrs::Capturer {
    // On Windows capturer returns screenshots only when screen is changing - use long timeout to
    // prevent crashes when there is nothing happening on the screen
    captrs::Capturer::new_with_timeout(0, Duration::from_secs(10 * 1000))
        .expect("Capturer creation failed.")
}

#[cfg(not(windows))]
pub fn get_capturer() -> captrs::Capturer {
    captrs::Capturer::new(0).expect("Capturer creation failed")
}

pub fn get_prepared_screenshot(capturer: &mut captrs::Capturer) -> Mat {
    let (width, height) = ACTUAL_PIXEL_COUNT;
    let size_with_crop = ACTUAL_PIXEL_COUNT_WITHOUT_CROP;
    let screenshot = take_screen(capturer);

    let mut squashed_screen = Mat::default();

    opencv::imgproc::resize(
        &screenshot,
        &mut squashed_screen,
        opencv::core::Size {
            width: width as i32,
            height: height as i32,
        },
        0.0,
        0.0,
        1,
    )
    .expect("Image resizing failed");

    get_cropped(
        &squashed_screen,
        &opencv::core::Rect {
            x: CROP_SIZE.0 as i32,
            y: CROP_SIZE.1 as i32,
            width: size_with_crop.0 as i32,
            height: size_with_crop.1 as i32,
        },
    )
}

fn take_screen(capturer: &mut captrs::Capturer) -> Mat {
    let frame = capturer.capture_frame().expect("Frame capture failed.");
    let screen_size = capturer.geometry();

    let mut image = get_zero_matrix(
        screen_size.0 as i32,
        screen_size.1 as i32,
        opencv::core::CV_8UC4,
    );

    for y in 0..screen_size.1 {
        for x in 0..screen_size.0 {
            let index_1d = x + y * screen_size.0;
            let captrs::Bgr8 { b, g, r, .. } = frame[index_1d as usize];

            unsafe {
                let ptr = image
                    .ptr_2d_mut(y as i32, x as i32)
                    .expect("Error while getting the pointer");
                *ptr = b;
                *(ptr.add(1)) = g;
                *(ptr.add(2)) = r;
                *(ptr.add(3)) = 255u8;
            }
        }
    }

    let mut image_grayscale = get_zero_matrix(
        screen_size.0 as i32,
        screen_size.1 as i32,
        opencv::core::CV_8UC4,
    );

    opencv::imgproc::cvt_color(
        &image,
        &mut image_grayscale,
        opencv::imgproc::ColorConversionCodes::COLOR_BGRA2GRAY as i32,
        0,
    )
    .expect("Color conversion to grayscale failed.");

    image_grayscale
}

fn get_cropped(image: &Mat, area: &opencv::core::Rect) -> Mat {
    let mut cropped_image = get_zero_matrix(area.width, area.height, opencv::core::CV_8U);

    for y in 0..area.height {
        for x in 0..area.width {
            let orig_pixel = image
                .ptr_2d(area.y + y, area.x + x)
                .expect("Error while getting pointer.");
            unsafe {
                *cropped_image
                    .ptr_2d_mut(y, x)
                    .expect("Error while getting pointer.") = *orig_pixel;
            }
        }
    }

    cropped_image
}

fn get_zero_matrix(width: i32, height: i32, mat_type: i32) -> opencv::prelude::Mat {
    Mat::zeros(height, width, mat_type)
        .expect("Zero array creation failed.")
        .to_mat()
        .expect("Conversion from array to matrix failed.")
}
