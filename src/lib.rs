#![deny(clippy::all)]
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use napi::bindgen_prelude::*;
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
fn generate(images: Vec<Buffer>, size: u32,border_margin:u32,margin:u32,bg_file:Option<String>,bg_color:[ u8; 4 ]) -> DynamicImage {
  // let _max_count = 9;// 最大数量
  let avatar_count = images.len() as u32; // 头像数量
  let avatar_per_col = (avatar_count as f32).sqrt().ceil() as u32; // 头像的列数
  let avatar_per_row = (avatar_count as f32 / avatar_per_col as f32).ceil() as u32; // 头像的行数
  let residue = avatar_per_col - (avatar_count % avatar_per_col);
  let avatar_size = (size - border_margin*2 - margin * ( avatar_per_col - 1 )) / avatar_per_col; // 计算头像尺寸
  let top_margin = (avatar_size + margin) * ( avatar_per_col - avatar_per_row) / 2; // 顶部边距（整体垂直居中用） = 一个头像和边距除2的距离
  

  let mut bg: ImageBuffer<Rgba<u8>, Vec<u8>>;
  // 判断 bg_file 是否为空，如果为空则使用默认背景
  if bg_file.is_none() {
    bg = ImageBuffer::from_fn(size, size, |_x, _y| Rgba(bg_color)); // Start with transparent background
  } else {
    bg = image::open(bg_file.unwrap().as_str()).expect("Failed to open image").resize(size, size,image::imageops::FilterType::Lanczos3).into_rgba8();
  }

  for (index, image_buffer) in images.iter().enumerate() {
      let img = resize_image(&image_buffer, avatar_size).thumbnail(avatar_size, avatar_size);
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


#[napi(object)]
pub struct Config{
  pub images: Vec<Buffer>,
  pub size: Option<u32>,
  pub border_margin: Option<u32>,
  pub margin: Option<u32>,
  pub save_file: Option<bool>,
  pub save_path: Option<String>,
  pub bg_file: Option<String>,
  pub bg_color: Option<Vec<u8>>,
}
impl Config {
  // 添加一个默认配置的方法
  pub fn new_default() -> Self {
      Self {
          images: Vec::new(),
          size: Some(600),
          border_margin: Some(20),
          margin: Some(20),
          save_file: Some(false),
          save_path: Some("group_avatar.png".to_string()),
          bg_file: None,
          bg_color: None

      }
  }
}
#[napi]
async fn generate_group_avatar(cfg: Config) -> Result<Option<Buffer>> {
  let config = Config::new_default();
  let bg_color: [u8; 4] = if let Some(ref colors) = cfg.bg_color {
      if colors.len() == 3 {
        // 将Vec<u8>转换为[u8; 4]
        [colors[0], colors[1], colors[2], 255]
      } else if colors.len() == 4 {
        [colors[0], colors[1], colors[2], colors[3]]
      } else {
        return Err(Error::new(
          Status::GenericFailure,
          format!("bg_color must be rgba 3 or 4 length U8int, but got length{}",colors.len()),
        ));
      }
  } else {
      // 如果cfg_bg_color是None，则使用默认值
      [0, 0, 0, 0]
  };
  // 判断可选配置是否有值，并将值覆盖
  let config = Config {
      images: if cfg.images.is_empty() { config.images } else { cfg.images },
      size: if cfg.size.is_some() { cfg.size } else { config.size },
      border_margin: if cfg.border_margin.is_some() { cfg.border_margin } else { config.border_margin },
      margin: if cfg.margin.is_some() { cfg.margin } else { config.margin},
      save_file: if cfg.save_file.is_some() { cfg.save_file } else { config.save_file },
      save_path: if cfg.save_path.is_some() { cfg.save_path } else { config.save_path },
      bg_file: config.bg_file,
      bg_color:Some(bg_color.to_vec())
  };

  napi::tokio::task::spawn(async move { 
    let group_avatar: DynamicImage = generate(
      config.images,
      config.size.unwrap(),
      config.border_margin.unwrap(),
      config.margin.unwrap(),
      config.bg_file,
      bg_color
    );
    if config.save_file.unwrap() {
      let res = match group_avatar.save(config.save_path.unwrap()) {
        Ok(_) => Ok(Option::None),
        Err(e) => {
          return Err(Error::new(
            Status::GenericFailure,
            format!("failed to save file, {}", e),
          ));
        }
      };
      return res;
    } else {
      // 准备一个空的 Vec<u8> 作为缓冲区
      let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
      // 将 DynamicImage 写入到 buf 中，格式为 PNG
      match group_avatar.write_to(&mut buf, ImageFormat::Png) {
        Ok(_) => {
          let bf = buf.into_inner();
          let aa = Buffer::from(bf);
          return Ok(Some(aa));
        },
        Err(e) => {
          return Err(Error::new(
            Status::GenericFailure,
            format!("failed to write_to file, {}", e),
          ));
        }
      }
    }
  })
  .await
  .unwrap()
}
