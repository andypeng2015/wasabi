use ast::Module;
use binary::WasmBinary;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[test]
fn decoding_valid_files_works() {
    for path in wasm_files("test/input") {
        Module::decode(&mut BufReader::new(File::open(path).unwrap())).unwrap();
    }
}

#[test]
fn decoding_and_encoding_roundtrips() {
    for path in wasm_files("test/input") {
        let mut wasm_binary_input = Vec::new();
        let mut wasm_binary_output = Vec::new();

        File::open(&path).unwrap().read_to_end(&mut wasm_binary_input).unwrap();

        let module = Module::decode(&mut Cursor::new(&wasm_binary_input)).unwrap();
        module.encode(&mut wasm_binary_output).unwrap();

        let mut output_file = File::create(path.to_string_lossy().replace("input", "output/identity")).unwrap();
        output_file.write_all(&mut &wasm_binary_output[..]).unwrap();

        assert!(wasm_binary_input == wasm_binary_output,
                "{}: encoding and decoding did not round-trip", path.display());
    }
}

// TODO add BENCH test for encoding + decoding (seperate maybe?)


/* Convenience functions */

fn wasm_files<P: AsRef<Path>>(dir: P) -> impl Iterator<Item=PathBuf> {
    WalkDir::new(dir.as_ref()).into_iter()
        .map(Result::unwrap)
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.extension() == Some("wasm".as_ref()))
}