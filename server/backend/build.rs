use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use brotli::CompressorWriter;

use flate2::Compression as GzipCompression;
use flate2::write::GzEncoder;

use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};

fn main() -> io::Result<()> {
  println!(
    "cargo:warning=compiling from `{}`",
    env::current_dir().unwrap().display()
  );

  compile_typescript()?;
  // should be called after compiling the ts
  compress_static_files()?;
  
  Ok(())
}

fn compile_typescript() -> io::Result<()> {
  let frontend_src = "../frontend/src";
  let js_output = "static/js";

  // create output dir
  fs::create_dir_all(js_output)?;

  let output = Command::new("esbuild")
    .args([
      &format!("{}/pagination.ts", frontend_src),
      "--bundle",
      "--minify",
      "--sourcemap",
      "--target=es2020",
      "--format=esm",
      &format!("--outdir={}", js_output),
    ])
    .output()
    .map_err(|e| io::Error::other(format!("Failed to run esbuild: {}", e)))?;  // <-- Fail here

  if !output.status.success() {
    return Err(io::Error::other(format!(
      "TypeScript compilation failed:\nstdout: {}\nstderr: {}",
      String::from_utf8_lossy(&output.stdout),
      String::from_utf8_lossy(&output.stderr),
    )));
  }

  println!("cargo:warning=TypeScript compiled successfully");
  Ok(())
}

fn compress_static_files() -> io::Result<()> {
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
  // skip already compressed files and source maps
  if input_path
    .extension()
    .is_some_and(|ext| ext == "gz" || ext == "br" || ext == "map")
  {
    return Ok(());
  }

  let ext = input_path
    .extension()
    .map(|e| e.to_string_lossy().into_owned())
    .unwrap_or_default();

  // read and process file based on type
  let input_data = fs::read(input_path)?;
  let processed_data = if ext == "css" {
    minify_css(&input_data)?
  } else {
    input_data
  };

  // gzip compression
  let gzip_path = input_path.with_extension(format!("{}.gz", ext));
  let gzip_file = File::create(&gzip_path)?;
  let mut gzip_encoder = GzEncoder::new(gzip_file, GzipCompression::default());
  gzip_encoder.write_all(&processed_data)?;
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
  brotli_compressor.write_all(&processed_data)?;
  brotli_compressor.flush()?;

  Ok(())
}

fn minify_css(input: &[u8]) -> io::Result<Vec<u8>> {
  let css_str = String::from_utf8_lossy(input);

  let stylesheet = StyleSheet::parse(&css_str, ParserOptions::default())
    .map_err(|e| io::Error::other(format!("CSS parse error: {:?}", e)))?;

  let result = stylesheet
    .to_css(PrinterOptions {
      minify: true,
      ..PrinterOptions::default()
    })
    .map_err(|e| io::Error::other(format!("CSS minify error: {:?}", e)))?;

  Ok(result.code.into_bytes())
}
