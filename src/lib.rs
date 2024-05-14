use napi_derive::napi;
use tree_sitter::{InputEdit, Language, Parser, Point};

#[napi]
pub fn parse(s: String) -> String {
  let mut parser = Parser::new();
  parser
    .set_language(&tree_sitter_rust::language())
    .expect("Error loading Rust grammar");
  let tree = parser.parse(s, None).unwrap();
  tree.root_node().to_sexp()
}
