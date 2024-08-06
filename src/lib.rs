mod utils;

use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use image::{codecs::png::PngEncoder, load_from_memory, ColorType::L8, ImageEncoder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello {name}!!!").as_str());
}

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    // log(encoded_file);
    let base64_to_vec = general_purpose::STANDARD.decode(encoded_file).unwrap();

    // Convert into Grayscale
    let mut img = load_from_memory(&base64_to_vec).unwrap();
    let gray_img = img.to_luma8();
    let mut buffer = vec![];

    let cursor = Cursor::new(&mut buffer);

    let encoder = PngEncoder::new(cursor);
    encoder
        .write_image(&gray_img, gray_img.width(), gray_img.height(), L8.into())
        .unwrap();

    let encoded_img = general_purpose::STANDARD.encode(buffer);

    let data_url = format!("data:image/png;base64,{}", encoded_img);

    log("PixflickerWASM: Grayscaling the base64 encoded image successful 🙂");

    data_url
}
