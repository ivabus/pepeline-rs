use std::path::Path;
use image::{ImageBuffer, Luma, Rgb, RgbImage};
use ndarray::{Array2, Array3, ArrayD};
use numpy::PyReadonlyArrayDyn;
use pyo3::{pyfunction};
fn array_gray2image(array:ArrayD<u8>,shape:&[usize])->ImageBuffer<Luma<u8>,Vec<u8>>{
    let array2: Array2<u8> = array.into_dimensionality().unwrap();
    let (w, h) = (shape[1] as u32, shape[0] as u32);
    ImageBuffer::from_fn(
        w,
        h,
        |x, y| {
            let value = array2[[y as usize, x as usize]];
            Luma([value])
        })

}
fn array_rgb2image(array: ArrayD<u8>,shape:&[usize]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (w, h) = (shape[1] as u32, shape[0] as u32);
    let raw = array.into_raw_vec();

    RgbImage::from_raw(w , h, raw)
        .expect("container should have the right size for the image dimensions")


}
pub fn gray_img_open(path:&Path)->Array2<u8>{
    let img = image::open(path).unwrap().into_luma8();
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    Array2::from_shape_vec((height as usize, width as usize), input).unwrap()
}
pub fn rgb_img_open(path:&Path)->Array3<u8>{
    let img = image::open(path).unwrap().into_rgb8();
    let (width, height) = img.dimensions();
    let input= img.into_raw();
    Array3::from_shape_vec((height as usize, width as usize, 3), input).unwrap()

}
#[pyfunction]
pub fn save(
    input: PyReadonlyArrayDyn<u8>,
    out_path:String)
{

    let array = input.as_array().to_owned();
    let shape = array.shape();
    match shape.len() {
        2 => {
            let img = array_gray2image(array.clone(),&shape);
            img.save(Path::new(&out_path)).expect("Not Save");
        }
        3 => {
            let img = array_rgb2image(array.clone(),&shape);
            img.save(Path::new(&out_path)).expect("Not Save");
        }

        _ => {panic!("Массив должен быть двумерным или трехмерным")}
    }

}
