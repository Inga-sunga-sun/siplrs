use std::path::Path;
use siplrs::image_file::ImageFile;
use siplrs::image_buffer::buffer::ImageBuffer;

fn main() {
    // let path_in = Path::new(r"D:\006_Source\Assets\5x5_u16.png");
    let path_in = Path::new(r"C:\Users\USER\Documents\006_Source\Assets\5x5_u16.png");
    let img = ImageFile::open(path_in).unwrap();
    let mut img2 = img.clone();
    let bin_img = img2.threshold_mut(8192, 16384);

    println!("here");

}
