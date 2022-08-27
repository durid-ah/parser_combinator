use std::{rc::Rc, fmt};

use crate::{
   models::{parser_traits::Parse, state::State}, 
   utility::local_log
};

/// # Choice
/// Goes the through the provided parsers and completes as soon as
/// one of them parses successfully. The Choice parser will ignore failed
/// parses but will return an error if none of the choices have executed successfully
/// 
/// ### Returns: 
/// The state of the first successful parser passed in
/// 
/// ### Examples
///
/// Basic Usage:
///
/// ```
/// use parser_combinator::collection_parsers::choice_parser::Choice;
/// use parser_combinator::parsers::str_parser::Str;
/// use parser_combinator::models::parser_traits::Parse;
///
/// let s1 = Box::new(Str::new("Test1".to_owned()));  
/// let s2 = Box::new(Str::new("Test2".to_owned()));  
/// let s3 = Box::new(Str::new("Test3".to_owned()));  
/// 
/// let choice = Choice::new(vec![s1,s2,s3]);
/// let res = choice.run("Test1");
/// 
/// assert_eq!(res.result.unwrap().unwrap().unwrap_one(), "Test1");
/// assert_eq!(res.index, 5);
/// ```
#[derive(Debug)]
pub struct Choice<R1,R2,T> {
   parsers: Vec<Box<dyn Parse<R1,R2,T>>>
}

impl<R1,R2,T> Choice<R1,R2,T> {
   pub fn new(parsers: Vec<Box<dyn Parse<R1,R2,T>>>) -> Self {
      Self { parsers }
   }

   pub fn push_parser(&mut self, parser: Box<dyn Parse<R1,R2,T>>) {
      self.parsers.push(parser);
   }
}

impl<R1,R2,T> Parse<R1,R2,T> for Choice<R1,R2,T> 
   where R1: fmt::Debug, R2: fmt::Debug, T: fmt::Debug {

   fn transform(&self, state: State<R1, T>) -> State<R2, T> {
      local_log::log(format!("{}", "Choice"));
      local_log::start_scope();

      let contains_error = state.is_error();

      if contains_error {
         local_log::log(format!("{:?}", state));
         local_log::end_scope();

         return State::from_err_state(state)
      }

      let mut final_state: State<R1, T> = State {
         index: state.index,
         target: Rc::clone(&state.target),
         result: None
      };

      for parser in &self.parsers {
         let next = parser.transform(final_state);

         match next.result.as_ref().unwrap() {
            Ok(_) => {
               local_log::log(format!("{:?}", next));
               local_log::end_scope();

               return next
            },
            _ => {
               final_state = State {
                  index: state.index,
                  target: Rc::clone(&state.target),
                  result: None
               };               
            }
         }

      }

      let res = State {
         index: state.index,
         target: Rc::clone(&state.target),
         result: Some(Err("Choice: Failed to parse any of the provided choices".to_owned()))
      };

      local_log::log(format!("{:?}", res));
      local_log::end_scope();
      
      res
   }
}

#[cfg(test)]
mod tests {
   use crate::parsers::str_parser::Str;
   use super::*;

   #[test]
   fn successful_first() {
      let s1 = Box::new(Str::new("Test1".to_owned()));  
      let s2 = Box::new(Str::new("Test2".to_owned()));  
      let s3 = Box::new(Str::new("Test3".to_owned()));  
      
      let choice = Choice::new(vec![s1,s2,s3]);
      let res = choice.run("Test1");

      assert_eq!(res.result.unwrap().unwrap().unwrap_one(), "Test1");
      assert_eq!(res.index, 5);
   }

   #[test]
   fn successful_middle() {
      let s1 = Box::new(Str::new("Test1".to_owned()));  
      let s2 = Box::new(Str::new("Test2".to_owned()));  
      let s3 = Box::new(Str::new("Test3".to_owned()));  
      
      let choice = Choice::new(vec![s1,s2,s3]);
      let res = choice.run("Test2");

      assert_eq!(res.result.unwrap().unwrap().unwrap_one(), "Test2");
      assert_eq!(res.index, 5);
   }

   #[test]
   fn successful_last() {
      let s1 = Box::new(Str::new("Test1".to_owned()));  
      let s2 = Box::new(Str::new("Test2".to_owned()));  
      let s3 = Box::new(Str::new("Test3".to_owned()));  
      
      let choice = Choice::new(vec![s1,s2,s3]);
      let res = choice.run("Test2");

      assert_eq!(res.result.unwrap().unwrap().unwrap_one(), "Test2");
      assert_eq!(res.index, 5);
   }
}
