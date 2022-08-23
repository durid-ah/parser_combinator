use parser_combinator::parsers::str_parser::Str;
use parser_combinator::collection_parsers::many_parser::Many;
use parser_combinator::models::parser_traits::Parse;

fn main() {
    let str_parser = Str::new("Test".to_owned());
    let many = Many::new(str_parser);
    let result = many.run("TestTestTest");
    assert!(result.result.is_some());
    assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 3);
}
