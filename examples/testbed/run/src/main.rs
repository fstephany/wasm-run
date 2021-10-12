use md5::Digest;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Asset {
    pub filename: String,
    pub path: PathBuf,
    pub content_hash: String,
}

fn main() {
    println!("Run");

    // Params:
    // - assets path
    // - output path
    // - config?

    // - go through all the assets
    // - get their hash
    // - copy them over the build directory with content hash
    // - OPTION: Apply optimizer there (e.g., re-encode jpeg?)
    // - Generate the manifest file

    // Actions:
    // - prepare
    // - clean
    // - info
    // - watch

    println!("Listing assets");
    let stylesheets = list_assets("assets/css");

    for style in stylesheets {
        println!("Style: {:?}", style)
    }
}

fn list_assets<P: AsRef<Path>>(path: P) -> Vec<Asset> {
    // assets/images
    // assets/css
    // scss bundles?
    // wasm bundles?
    // --> What about SCSS compilation?

    let entries = std::fs::read_dir(path).unwrap();

    entries
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = entry.file_name();
            println!("- {:?}", entry);
            let mut f = File::open(&path).unwrap();

            let mut buf: Vec<u8> = Vec::new();
            f.read_to_end(&mut buf).unwrap();

            let digest = md5::Md5::digest(&buf);
            let content_hash = String::from(digest.to_ascii_lowercase());

            Asset {
                filename: filename.to_string_lossy().into_owned(),
                path,
                content_hash,
            }
        })
        .collect()
}
