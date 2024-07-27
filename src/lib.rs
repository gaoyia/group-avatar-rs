#![deny(clippy::all)]
use std::io::{BufWriter, Read};

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, RgbImage, Rgba};
use napi::{bindgen_prelude::Buffer, JsBuffer};

#[macro_use]
extern crate napi_derive;


fn resize_image(image_buffer: &Buffer, target_size: u32) -> DynamicImage {
      // 尝试使用 `image` 库解码图片
      let img = match image::load_from_memory(&image_buffer) {
        Ok(image) => image,
        Err(_) => image::DynamicImage::new_rgb8(1, 1)
    };
    let resized = img.resize_to_fill(target_size, target_size, image::imageops::FilterType::Lanczos3);
    resized
}
fn generate(images: Vec<Buffer>, size: u32) -> DynamicImage {
  // let _max_count = 9;// 最大数量
  let avatar_count = images.len() as u32; // 头像数量
  let border_margin = size / 10; // 边框间距
  let avatar_per_col = (avatar_count as f32).sqrt().ceil() as u32; // 头像的列数
  let avatar_per_row = (avatar_count as f32 / avatar_per_col as f32).ceil() as u32; // 头像的行数
  let residue = avatar_per_col - (avatar_count % avatar_per_col);
  let margin = size / 30; // 外框边距
  let avatar_size = (size - border_margin*2 - margin * ( avatar_per_col - 1 )) / avatar_per_col; // 计算头像尺寸
  let top_margin = (avatar_size + margin) * ( avatar_per_col - avatar_per_row) / 2; // 顶部边距（整体垂直居中用） = 一个头像和边距除2的距离
  
  // Load the background image
  // let bg_color: [ u8; 4 ] = [255, 255, 255, 255]; // 背景颜色
  // let mut result = ImageBuffer::from_fn(size, size, |_x, _y| Rgba(bg_color)); // Start with transparent background
  let mut bg = image::open("bg.jpg").expect("Failed to open image").resize(size, size,image::imageops::FilterType::Lanczos3).into_rgba8();
  for (index, image_buffer) in images.iter().enumerate() {
      let img = resize_image(&image_buffer, avatar_size);
      let img = img.thumbnail(avatar_size, avatar_size);
      let row = index as u32 / avatar_per_col + 1;
      let col = index as u32 % avatar_per_col + 1;

      let mut x_offset = (col - 1) * (avatar_size + margin) + border_margin;
      let y_offset = (row - 1) * (avatar_size + margin) + border_margin + top_margin;

      // 如果是最后一行,且余数不等于列数，增加x偏移居中
      let is_last_row = avatar_per_row == row;

      if is_last_row && residue != avatar_per_col {
          x_offset = x_offset + residue * ( margin + avatar_size ) / 2;
      }

      image::imageops::overlay(&mut bg, &img, x_offset as i64, y_offset as i64);
  }

  DynamicImage::ImageRgba8(bg)
}
#[napi]
pub fn generate_group_avatar(images: Vec<Buffer>, size: u32) -> Buffer  {
  let group_avatar: DynamicImage = generate(images, 600);
  // 准备一个空的 Vec<u8> 作为缓冲区
  let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
  group_avatar.save("group_avatar.png").expect("Failed to save group avatar");
  // 将 DynamicImage 写入到 buf 中，格式为 PNG
  group_avatar.write_to(&mut buf, ImageFormat::Png).expect("Failed to write image");
  let bf = buf.into_inner();
  Buffer::from(bf)
}

