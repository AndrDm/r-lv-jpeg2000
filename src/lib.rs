use opencv::{
	core::{CV_16UC1, Mat, Vector},
	imgcodecs,
	prelude::*,
};
use std::{
	ffi::CStr,
	os::raw::{c_char, c_int},
	slice::*,
};

/// Saves a 16-bit grayscale image as JPEG2000.
///
/// # Safety
///
/// -   `pixels` must point to a valid 16-bit image buffer containing at least
///     `height * stride_pixels` elements of type `u16`.
/// -   `pixels` must remain valid for the duration of this call.
/// -   `path` must point to a valid NUL-terminated string.
///     `path` must remain valid for the duration of this call.
/// -   `width` and `height` must be positive.
/// -   `stride_pixels` is the line width and must be greater than or equal to `width`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_jpeg2000(
	pixels: *const u16,
	stride_pixels: c_int,
	width: c_int,
	height: c_int,
	compression_level: c_int,
	path: *const c_char,
) -> c_int {
	if pixels.is_null() || path.is_null() || width <= 0 || height <= 0 || stride_pixels < width {
		return -1;
	}

	let result = (|| -> opencv::Result<()> {
		let filename = unsafe {
			CStr::from_ptr(path).to_str().map_err(|_| opencv::Error::new(0, "Invalid path"))?
		};

		let mut image = Mat::new_rows_cols_with_default(height, width, CV_16UC1, 0.into())?;
		let dst = image.data_typed_mut::<u16>()?;

		let w = width as usize;
		for y in 0..height {
			let src_row = unsafe { from_raw_parts(pixels.add((y * stride_pixels) as usize), w) };
			let offset = y as usize * w;
			dst[offset..offset + w].copy_from_slice(src_row);
		}

		let mut params = Vector::<i32>::new();
		params.push(imgcodecs::IMWRITE_JPEG2000_COMPRESSION_X1000);
		params.push(compression_level.clamp(1, 1000));

		if !imgcodecs::imwrite(filename, &image, &params)? {
			return Err(opencv::Error::new(0, "Failed to save JPEG2000"));
		}

		Ok(())
	})();

	if result.is_ok() { 0 } else { -2 }
}
