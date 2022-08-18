use std::{cell::RefCell, rc::Rc, vec};
use parser_combinator::{
   parsers::{
      digits_parser::Digits, 
      str_parser::Str, 
      between_parser::Between
   }, 
   collection_parsers::{
      choice_parser::Choice, 
      sequence_of_parser::SequenceOf, 
      many_one_parser::ManyOne
   }, 
   models::{
      parser::Parser, 
      parser_traits::Parse, 
      state::{State, ParserResult}, cardinality::Cardinality
   }, parser_helpers::map_result
};

// type StrParser<'a> = Parser<'a, String, String, &'a str>;
// type StrSeq<'a> = SequenceOf<String,String,&'a str>;
// type StrChoice<'a> = Choice<String,String,&'a str>;
type Ptr<T> = Rc<RefCell<T>>;

type TokenParser<'a> = Parser<'a, String, Token, &'a str>;
type TokenParseTrait<'a> = Box<dyn Parse<String, Token, &'a str>>;
type TokenSeq<'a> = SequenceOf<String,Token,&'a str>;
type TokenChoice<'a> = Choice<String,Token,&'a str>;
type TokenRes = Result<Cardinality<Token>, String>;

#[derive(Debug)]
enum Operation {
   Add,
   Subtract,
   Divide,
   Multiply
}

#[derive(Debug)]
enum Token {
   Op(Operation),
   Number(f64),
   String(String),
   // Statement(Vec<Token>)
}

fn map_string_parser<'a>(parser: Str) -> Box<dyn Parse<String, Token, &'a str> + 'a> {
   
   return map_result(parser, |result: ParserResult<String>| {
      let res: TokenRes = match result {
         Some(Ok(Cardinality::One(res))) => 
            Ok(Cardinality::One(Token::String(res))),
         _ => Err("Failed to parse string".to_owned())
      };
   
      Some(res)
   });
}

pub fn main() {
   let digits: Box<dyn Parse<String, Token, &str>> = map_result(Digits::new(), 
      |result: ParserResult<String>| {
         let res = match result {
            Some(Ok(Cardinality::One(res))) => 
               Ok(Cardinality::One(
                  Token::Number(res.parse::<f64>().unwrap()))),
            _ => Err("Failed to parse digits".to_owned())
         };

         Some(res)
      });

   let space: TokenParseTrait = map_string_parser(Str::new(" ".to_owned()));

   let add = Str::new("+".to_owned());
   let subtract = Str::new("-".to_owned());
   let multiply = Str::new("*".to_owned());
   let divide = Str::new("/".to_owned());

   let expr: TokenChoice = Choice::new(vec![digits]);

   let expr: Ptr<TokenChoice> = Rc::new(RefCell::new(expr));

   let expr_2: Ptr<TokenChoice> = Rc::clone(&expr);
   let expr_parser: TokenParser = Parser::new(
      Box::new(
         move |state: State<String, &str>| expr_2.borrow().transform(state)));

   let space_prefix_expr: Box<TokenSeq> = Box::new(
      SequenceOf::new(vec![space, Box::new(expr_parser)]));

   let operator = Choice::new(vec![
      Box::new(add),
      Box::new(subtract),
      Box::new(multiply),
      Box::new(divide)
   ]);

   let operator: TokenParseTrait = map_result(operator, 
      |result: ParserResult<String>| {
         use Cardinality::One;
         use Token::Op;
         use Operation::{Add, Subtract, Multiply, Divide};

         let res: TokenRes = if let Some(Ok(One(res))) = result {
            match res.as_str() {
               "+" => Ok(One(Op(Add))),
               "-" => Ok(One(Op(Subtract))),
               "*" => Ok(One(Op(Multiply))),
               "/" => Ok(One(Op(Divide))),
               _ => panic!("Invalid Operation")
            }
         } else {
            panic!("Unkown Operation");
         };

         Some(res)
      });

   let operation_sequence: TokenSeq =
      SequenceOf::new(vec![
         operator,
         Box::new(ManyOne::new(space_prefix_expr)) // TODO: change to: at_least_two
      ]);

   let left_bracket: TokenParseTrait = map_string_parser(Str::new("(".to_owned()));
   let right_bracket: TokenParseTrait = map_string_parser(Str::new("(".to_owned()));

   let operation = Between::new(
      left_bracket,
      right_bracket,
      Box::new(operation_sequence));

   expr.borrow_mut().push_parser(Box::new(operation));
   let _ = expr.borrow().run("(+ 1 2)");

   println!("Stuff")
}