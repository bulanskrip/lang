use pest_derive::Parser as PestParser;
#[derive(PestParser)]
#[grammar = "grammar.pest"] // relative to src
struct Parser;
pub mod error;
pub mod runtime;
