use pest_derive::*;

#[derive(Parser)]
#[grammar = "pest/grammar.pest"]
pub struct MnsParser;