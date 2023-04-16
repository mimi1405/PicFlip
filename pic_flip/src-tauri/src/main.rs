// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use image::{ GenericImage, GenericImageView, ImageBuffer, RgbImage };
use image_convert::{ ImageResource, PNGConfig, BMPConfig, GIFConfig, JPGConfig, TIFFConfig, WEBPConfig, ICOConfig, GrayRawConfig, PGMConfig, to_png, to_bmp, to_gif, to_jpg, to_tiff, to_webp, to_ico, to_gray_raw, to_pgm };
//TODO! Overview methods --> currently test with jpg to png is located in jpg method instead of png method
#[tauri::command]
fn convert_image(path: &str, choice: &str) {
    match choice {
        "png" => create_png(path),
        "jpg" => create_jpg(path),
        "jpeg" => create_jpg(path),
        "bmp" => create_bmp(path),
        "gif" => create_gif(path),
        "tiff" => create_tiff(path),
        "webp" => create_webp(path),
        "ico" => create_ico(path),
        "raw" => create_gray_raw(path),
        "pgm" => create_pgm(path),
        _ => println!("There is no accepted format!"),
    };
}

fn create_pgm(path: &str) {
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.pgm");
    let mut config = PGMConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_pgm(&mut output, &input, &config).unwrap();
}

fn create_gray_raw(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.raw");
    let mut config = GrayRawConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_gray_raw(&mut output, &input, &config).unwrap();
}

fn create_jpg(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.jpg");
    let mut config = JPGConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_jpg(&mut output, &input, &config).unwrap();
}

fn create_ico(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.ico");
    let mut config = ICOConfig::new();
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_ico(&mut output, &input, &config).unwrap();
}

fn create_webp(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.webp");
    let mut config = WEBPConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_webp(&mut output, &input, &config).unwrap();
}

fn create_png(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.png");
    let mut config = PNGConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_png(&mut output, &input, &config).unwrap();
}

fn create_gif(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.gif");
    let mut config = GIFConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_gif(&mut output, &input, &config).unwrap();
}

fn create_bmp(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.bmp");
    let mut config = BMPConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_bmp(&mut output, &input, &config).unwrap();
}

fn create_tiff(path: &str){
    let source_image_path = Path::new(path);
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.tiff");
    let mut config = TIFFConfig::new();
    config.width = 800;
    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path);
    to_tiff(&mut output, &input, &config).unwrap();
}



fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![convert_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}