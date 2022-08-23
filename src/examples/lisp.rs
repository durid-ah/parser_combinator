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
      state::{State, ParserResult}, cardinality::Cardinality::{self, One}
   }, parser_helpers::map_result
};


type Ptr<T> = Rc<RefCell<T>>;

type TokenParser<'a> = Parser<'a, String, Token, &'a str>;
type TokenSeq<'a> = SequenceOf<String,Token,&'a str>;
type TokenChoice<'a> = Choice<String,Token,&'a str>;
type TokenRes = Result<Cardinality<Token>, String>;

#[derive(Debug, Clone, Copy)]
enum Operation {
   Add,
   Minus,
   Divide,
   Multiply
}

#[derive(Debug, Clone)]
enum Token {
   Op(Operation),
   Number(f64),
   String(String),
   Statement(Vec<Token>)
}

fn map_string_parser<'a>(parser: Str) -> TokenParser<'a> {
   return map_result(parser, |result: ParserResult<String>| {
      let res: TokenRes = match result {
         Some(Ok(One(res))) => Ok(One(Token::String(res))),
         _ => Err("Failed to parse string".to_owned())
      };
   
      Some(res)
   });
}

fn map_digit_parser<'a> (digits: Digits) -> TokenParser<'a> {
   map_result(digits, 
      |result: ParserResult<String>| {
         let res: TokenRes = match result {
            Some(Ok(One(res))) => 
               Ok(One(Token::Number(res.parse::<f64>().unwrap()))),
            Some(Err(err)) => Err(err),
            _ => Err("Failed to parse digits".to_owned())
         };

         Some(res)
      })
}

pub fn main() {
   // Get the digits and convert it to Token type
   let digits: TokenParser = map_digit_parser(Digits::new());

   let space: TokenParser = map_string_parser(Str::new(" ".to_owned()));

   // Create a parser for each mathematical operation
   let add = Str::new("+".to_owned());
   let subtract = Str::new("-".to_owned());
   let multiply = Str::new("*".to_owned());
   let divide = Str::new("/".to_owned());

   let expr: TokenChoice = Choice::new(vec![Box::new(digits)]);
   let expr: Ptr<TokenChoice> = Rc::new(RefCell::new(expr));
   let expr_2: Ptr<TokenChoice> = Rc::clone(&expr);
   let expr_parser: TokenParser = Parser::new(
      Box::new(
         move |state: State<String, &str>| expr_2.borrow().transform(state)));

   let expr_parser = map_result(
      expr_parser, 
      |res: ParserResult<Token>| {
         match res {
            Some(Ok(Cardinality::Many(vals))) => Some(Ok(One(Token::Statement(vals)))),
            val => val
         }
      }
   );

   let space_prefix_expr: TokenParser = map_result(
      SequenceOf::new(vec![Box::new(space), Box::new(expr_parser)]),
      |res: ParserResult<Token>| {
         match res {
            Some(Ok(Cardinality::Many(res))) => Some(Ok(One(res[1].clone()))),
            Some(Err(s)) => Some(Err(s)),
            _ => panic!("Space Prefix Parse Failed") 
         }
      }
   );

   let operator = Choice::new(vec![
      Box::new(add),
      Box::new(subtract),
      Box::new(multiply),
      Box::new(divide)
   ]);

   let operator: TokenParser = map_result(operator, 
      |result: ParserResult<String>| {
         use Token::Op;
         use Operation::{Add, Minus, Multiply, Divide};

         let res: TokenRes = if let Some(Ok(One(res))) = result {
            match res.as_str() {
               "+" => Ok(One(Op(Add))),
               "-" => Ok(One(Op(Minus))),
               "*" => Ok(One(Op(Multiply))),
               "/" => Ok(One(Op(Divide))),
               _ => panic!("Invalid Operation")
            }
         } else {
            panic!("Unkown Operation");
         };

         Some(res)
      });

   let operation_sequence: TokenSeq = SequenceOf::new(vec![
         Box::new(operator),
         Box::new(ManyOne::new(space_prefix_expr)) // TODO: change to: at_least_two
      ]);

   let left_bracket: TokenParser = map_string_parser(Str::new("(".to_owned()));
   let right_bracket: TokenParser = map_string_parser(Str::new(")".to_owned()));

   let operation = Between::new(
      Box::new(left_bracket),
      Box::new(right_bracket),
      Box::new(operation_sequence));

   expr.borrow_mut().push_parser(Box::new(operation));
   let res = expr.borrow().run("(+ 1 2 (/ 5 2) (* 2 5) (- 1 5))");

   if let Some(Ok(Cardinality::Many(vals))) = res.result {

      let result = eval(&Token::Statement(vals));
      println!("RESULT: {}", result);
   }
}

fn eval(root_token: &Token) -> f64 {
   let mut total = 0_f64;

   if let Token::Number(val) = root_token {
      return *val
   } else if let Token::Statement(vals) = root_token {

      let op = &vals[0];
      let rest = &vals[1..];
      if let Token::Op(operation) = op {
         use Operation::*;
   
         match operation {
            Add => {
               total = rest.into_iter()
                  .fold(0_f64, |prev, curr| prev + eval(curr));
            },
            Minus => {
               total = rest
                  .into_iter()
                  .rev()
                  .fold(0_f64, |prev, curr| eval(curr) - prev);
            },
            Multiply => {
               total = rest
                  .into_iter()
                  .fold(1_f64, |prev, curr| eval(curr) * prev);
            },
            Divide => {
               total = rest
                  .into_iter()
                  .rev()
                  .fold(1_f64, |prev, curr| eval(curr) / prev);
            }   
         }
   
      }
   }



   total
}