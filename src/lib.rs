use opencv::{
	core::{CV_16UC1, Mat},
	imgcodecs,
	prelude::*,
};

use std::{
	ffi::CStr,
	os::raw::{c_char, c_int},
};

#[unsafe(no_mangle)]
pub extern "C" fn save_jpeg2000(
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

		let width_usize = width as usize;
		let height_usize = height as usize;
		let stride_usize = stride_pixels as usize;

		let mut image = Mat::new_rows_cols_with_default(height, width, CV_16UC1, 0.into())?;

		let dst = image.data_typed_mut::<u16>()?;

		for y in 0..height_usize {
			let src_row =
				unsafe { std::slice::from_raw_parts(pixels.add(y * stride_usize), width_usize) };

			let dst_row = &mut dst[y * width_usize..(y + 1) * width_usize];

			dst_row.copy_from_slice(src_row);
		}

		let mut params = opencv::core::Vector::<i32>::new();

		params.push(imgcodecs::IMWRITE_JPEG2000_COMPRESSION_X1000);
		params.push(compression_level.clamp(1, 1000));

		let ok = imgcodecs::imwrite(filename, &image, &params)?;

		if !ok {
			return Err(opencv::Error::new(0, "Failed to save JPEG2000"));
		}

		Ok(())
	})();

	match result {
		Ok(_) => 0,
		Err(_) => -1,
	}
}
