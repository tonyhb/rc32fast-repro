use std::io::{Cursor, Read};
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("no file specified as argument");
        return;
    }

    let filename = std::env::args().nth(1).unwrap();
    match open_zip(filename) {
        Ok(p) => {
            println!("Done! Parsed {} files", p.zip_data.len());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    web_sys::console::log_1(&"initialized".into());
}

pub struct WasmMain {}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
impl WasmMain {
    pub fn new(data: Vec<u8>) -> () {
        web_sys::console::log_1(&"parsing".into());
        match Project::new(data) {
            Ok(p) => {
                web_sys::console::log_1(&format!("Parsed {} files", p.zip_data.len()).into());
            }
            Err(e) => {
                web_sys::console::log_2(&"Error".into(), &format!("{:?}", e).into());
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_zip(filename: String) -> Result<Project, Box<dyn std::error::Error>> {
    let fname = std::path::Path::new(&filename);
    let data = std::fs::read(&fname)?;
    Project::new(data)
}

pub struct Project {
    zip_data: Vec<File>,
}

pub struct File {
    name: String,
    data: Vec<u8>,
}

impl Project {
    pub fn new(zipdata: Vec<u8>) -> Result<Project, Box<dyn std::error::Error>> {
        let mut archive = zip::ZipArchive::new(Cursor::new(zipdata))?;
        let mut zip_data: Vec<File> = Vec::new();

        for i in 0..archive.len() {
            let mut f = archive.by_index(i).unwrap();
            let mut buf = vec![];
            f.read_to_end(&mut buf).unwrap();

            let name = f.name();
            zip_data.push(File {
                name: name.into(),
                data: buf,
            });
        }

        Ok(Project { zip_data: zip_data })
    }
}
