use tree_sitter::{Language, Parser};

extern "C" { fn tree_sitter_c() -> Language; }
extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();

    let source_code = "fn test() {}";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    assert_eq!(root_node.kind(), "source_file");
}
