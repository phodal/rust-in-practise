use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();

    cc::Build::new()
        .cpp(true)
        // .flag("-xc++")
        .include(&dir)
        .file(dir.join("scanner.c"))
        .file(dir.join("parser.c"))
        .compile("tree-sitter-javascript");
}