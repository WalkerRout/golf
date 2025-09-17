use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use brotli::CompressorWriter;

use flate2::write::GzEncoder;
use flate2::Compression as GzipCompression;

fn main() -> io::Result<()> {
  let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let static_dir = Path::new(&out_dir).join("static");

  fs::create_dir_all(&static_dir)?;

  compress_directory(&static_dir)?;

  Ok(())
}

fn compress_directory(dir: &Path) -> io::Result<()> {
  if !dir.is_dir() {
    return Ok(());
  }

  for entry in fs::read_dir(dir)? {
    let path = entry?.path();

    if path.is_dir() {
      compress_directory(&path)?;
    } else if path.is_file() {
      compress_file(&path)?;
    }
  }

  Ok(())
}

fn compress_file(input_path: &Path) -> io::Result<()> {
  // Skip already compressed files
  if input_path
    .extension()
    .is_some_and(|ext| ext == "gz" || ext == "br")
  {
    return Ok(());
  }

  // get original file extension
  let ext = input_path
    .extension()
    .map(|e| e.to_string_lossy().into_owned())
    .unwrap_or_default();

  // gzip compression
  let gzip_path = input_path.with_extension(format!("{}.gz", ext));
  let gzip_file = File::create(&gzip_path)?;
  let mut gzip_encoder = GzEncoder::new(gzip_file, GzipCompression::default());
  let input_data = fs::read(input_path)?;
  gzip_encoder.write_all(&input_data)?;
  gzip_encoder.finish()?;

  // brotli compression
  let brotli_path = input_path.with_extension(format!("{}.br", ext));
  let brotli_file = File::create(&brotli_path)?;
  let mut brotli_compressor = CompressorWriter::new(
    brotli_file,
    4096, // input buffer size
    11,   // quality (0-11, higher better compression but slower)
    22,   // lgwin (window size, 22 is max)
  );
  brotli_compressor.write_all(&input_data)?;
  brotli_compressor.flush()?;

  Ok(())
}
