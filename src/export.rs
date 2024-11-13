use anyhow::Result;
use std::fs::File;
use std::io::BufWriter;

pub fn export_png(
  path: &str,
  data: &[u8],
  width: u32,
  height: u32,
) -> Result<()> {
  let file = File::create(path)?;
  let ref mut w = BufWriter::new(file);

  let mut png_encoder = png::Encoder::new(w, width, height);
  png_encoder.set_color(png::ColorType::Rgba);

  let mut writer = png_encoder.write_header()?;
  writer.write_image_data(&data)?;

  Ok(())
}
