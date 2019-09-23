use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-python", "src"].iter().collect();

    let mut c_builder = cc::Build::new();
    let mut cpp_builder = cc::Build::new();

    c_builder
        .include(&dir)
        .pic(true)
        .warnings(false)
        .file(dir.join("parser.c"))
        .compile("parser");

    cpp_builder
        .include(&dir)
        .cpp(true)
        .file(dir.join("scanner.cc"))
        .compile("tree-sitter-python");
}

