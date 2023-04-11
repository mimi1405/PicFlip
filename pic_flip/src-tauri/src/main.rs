// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use image::{ GenericImage, GenericImageView, ImageBuffer, RgbImage };
use image_convert::{ ImageResource, PNGConfig, to_png };
//TODO! Overview methods --> currently test with jpg to png is located in jpg method instead of png method
#[tauri::command]
fn convert_image(path: &str, choice: &str) {
    let p = Path::new(path);
    let extension = p.extension().unwrap().to_str().unwrap();
    match extension {
        "svg" => to_svg(choice, path),
        "png" => make_png(choice, path),
        "jpg" => to_jpg(choice, path),
        "jpeg" => to_jpg(choice, path),
        "bmp" => to_bmp(choice, path),
        "gif" => to_gif(choice, path),
        _ => println!("There is no accepted format!"),
    }
}

fn to_svg(extension: &str, path: &str) {
    match extension {
        "png" => png_to_svg(path),
        "jpg" => jpeg_to_svg(path),
        "bmp" => bmp_to_svg(path),
        "gif" => gif_to_svg(path),
        _ => println!("There is no accepted format!"),
    }
}

fn to_bmp(extension: &str, path: &str) {
    match extension {
        "png" => png_to_bmp(path),
        "jpg" => jpg_to_bmp(path),
        "svg" => svg_to_bmp(path),
        "gif" => gif_to_bmp(path),
        _ => println!("There is no accepted format!"),
    }
}

fn to_gif(extension: &str, path: &str) {
    match extension {
        "png" => png_to_gif(path),
        "jpg" => jpeg_to_gif(path),
        "bmp" => bmp_to_gif(path),
        "svg" => svg_to_gif(path),
        _ => println!("There is no accepted format!"),
    }
}

fn make_png(extension: &str, path: &str) {
    match extension {
        "gif" => gif_to_png(path),
        "jpg" => jpeg_to_png(path),
        "bmp" => bmp_to_png(path),
        "svg" => svg_to_png(path),
        _ => println!("There is no accepted format!"),
    }
}

fn to_jpg(extension: &str, path: &str) {
    match extension {
        "gif" => gif_to_jpeg(path),
        "png" => png_to_jpeg(path),
        "bmp" => bmp_to_jpeg(path),
        "svg" => svg_to_jpeg(path),
        _ => println!("There is no accepted format!"),
    }
}

// SVG Conversions
fn png_to_svg(path: &str) {}

fn jpeg_to_svg(path: &str) {}
fn bmp_to_svg(path: &str) {}
fn gif_to_svg(path: &str) {}

// Bitmap Converspath: &strions
fn png_to_bmp(path: &str) {}
fn jpg_to_bmp(path: &str) {}
fn svg_to_bmp(path: &str) {}
fn gif_to_bmp(path: &str) {}

// GIF Converspath: &strions
fn png_to_gif(path: &str) {}
fn jpeg_to_gif(path: &str) {}
fn svg_to_gif(path: &str) {}
fn bmp_to_gif(path: &str) {}

// PNG Converspath: &strions
fn gif_to_png(path: &str) {}
fn jpeg_to_png(path: &str) {
    
}
fn svg_to_png(path: &str) {}
fn bmp_to_png(path: &str) {}

// jpeg Converspath: &strions
fn gif_to_jpeg(path: &str) {}
fn png_to_jpeg(path: &str) {
    let source_image_path = Path::new(path);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "new.png");

    let mut config = PNGConfig::new();

    config.width = 800;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_png(&mut output, &input, &config).unwrap();
}
fn svg_to_jpeg(path: &str) {}
fn bmp_to_jpeg(path: &str) {}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![convert_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}