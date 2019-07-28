extern crate nom;

mod ast;
mod codegen;
mod parser;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let filename = args().skip(1).next().unwrap();
    let path = Path::new(&filename);
    let mut file = File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let (_, ast) = parser::expression(&text).unwrap();
    println!("{:#?}", ast);

    let output_path = format!(
        "{}.s",
        Path::new(&path)
            .file_stem()
            .expect("Can't open output file")
            .to_str()
            .unwrap()
    );
    let mut output_file = File::create(output_path).unwrap();

    codegen::codegen(&ast, &mut output_file).unwrap();
}
