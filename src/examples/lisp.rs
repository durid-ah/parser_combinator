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
      state::State
   }
};

type StrParser<'a> = Parser<'a, String, String, &'a str>;
type StrSeq<'a> = SequenceOf<String,String,&'a str>;
type StrChoice<'a> = Choice<String,String,&'a str>;
type Ptr<T> = Rc<RefCell<T>>;

enum Operation {
   Add,
   Subtract,
   Divide,
   Multiply
}

enum Token {
   Op(Operation),
   Number(f64),
   Statement(Vec<Token>)
}

pub fn main() {
   let digits = Box::new(Digits::new());
   let add = Str::new("+".to_owned());
   let subtract = Str::new("-".to_owned());
   let multiply = Str::new("*".to_owned());
   let divide = Str::new("/".to_owned());

   let expr: StrChoice = Choice::new(vec![
      digits,
   ]);

   let expr: Ptr<StrChoice> = Rc::new(RefCell::new(expr));

   let expr_2: Ptr<StrChoice> = Rc::clone(&expr);
   let expr_parser: StrParser = Parser::new(
      Box::new(
         move |state: State<String, &str>| expr_2.borrow().transform(state))
   );

   let space_prefix_expr: Box<StrSeq> = Box::new(
      SequenceOf::new(vec![
         Box::new(Str::new(" ".to_owned())),
         Box::new(expr_parser)
      ]));

   let operator = Choice::new(vec![
      Box::new(add),
      Box::new(subtract),
      Box::new(multiply),
      Box::new(divide)
   ]);

   let operation_sequence: StrSeq =
      SequenceOf::new(vec![
         Box::new(operator),
         Box::new(ManyOne::new(space_prefix_expr)) // TODO: change to: at_least_two
      ]);

   let operation = Between::new(
      Box::new(Str::new("(".to_owned())),
      Box::new(Str::new(")".to_owned())),
      Box::new(operation_sequence)
   );

   expr.borrow_mut().push_parser(Box::new(operation));
   let res = expr.borrow().run("(+ 1 2)");

   println!("Stuff")
}