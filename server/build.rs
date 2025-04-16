use include_dir::{include_dir, Dir};
use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("assets.rs");
  let static_dir: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

  // codegen
  let mut file = File::create(&dest_path).unwrap();
  generate_assets_code(&mut file, &static_dir);

  println!("cargo:rerun-if-changed=static/*");
}

fn generate_assets_code(file: &mut File, static_dir: &Dir) {
  writeln!(file, "// Auto-generated asset manifest").unwrap();
  writeln!(file, "pub struct HashedAsset {{").unwrap();
  writeln!(file, "    pub original: &'static str,").unwrap();
  writeln!(file, "    pub hashed: &'static str,").unwrap();
  writeln!(file, "    pub contents: &'static [u8],").unwrap();
  writeln!(file, "}}").unwrap();

  writeln!(file, "pub static ASSETS: &[HashedAsset] = &[").unwrap();

  // Iterate through all files in the directory
  for file_entry in static_dir.files() {
    // Get the path as a string
    let full_path = file_entry.path().to_string_lossy().to_string();

    // Generate hash
    let contents = file_entry.contents();
    let mut hasher = Sha256::new();
    hasher.update(contents);
    let hash = format!("{:x}", hasher.finalize());
    let short_hash = &hash[..6];

    // Create hashed filename
    let (name, ext) = full_path
      .rsplit_once('.')
      .map(|(n, e)| (n, e))
      .unwrap_or((&full_path, ""));

    let hashed_filename = if ext.is_empty() {
      format!("{}.{}", name, short_hash)
    } else {
      format!("{}.{}.{}", name, short_hash, ext)
    };

    // Escape paths for Rust string literals
    let escaped_original = full_path.replace('\\', "\\\\").replace('"', "\\\"");
    let escaped_hashed = hashed_filename.replace('\\', "\\\\").replace('"', "\\\"");

    // Write asset entry
    writeln!(file, "    HashedAsset {{").unwrap();
    writeln!(file, "        original: \"{}\",", escaped_original).unwrap();
    writeln!(file, "        hashed: \"{}\",", escaped_hashed).unwrap();
    writeln!(
      file,
      "        contents: include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/static/{}\")),",
      escaped_original
    )
    .unwrap();
    writeln!(file, "    },").unwrap();
  }

  writeln!(file, "];").unwrap();
}
