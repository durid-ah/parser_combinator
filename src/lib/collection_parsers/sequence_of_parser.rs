use std::fmt::{Debug, self};
use std::rc::Rc;
use crate::models::{parser_traits::Parse, state::State};
use crate::models::cardinality::Cardinality::{One, Many};


/// # SequenceOf:
/// Parse a sequence of parsers, if the parser fails it will return an error
/// instead of the parsed values
/// 
/// ### Returns:
/// A result of type [`Many`]
///
/// ### Examples
///
/// Basic Usage:
///
/// ```
/// use parser_combinator::collection_parsers::sequence_of_parser::SequenceOf;
/// use parser_combinator::parsers::str_parser::Str;
/// use parser_combinator::models::parser_traits::Parse;
///
/// let comma = Str::new(",".to_owned());
/// let test_string = Str::new("Test".to_owned());
/// let mut seq_parser = SequenceOf::new(vec![Box::new(test_string)]);
/// let result = seq_parser.run("Test,Test,Test");
/// 
/// assert!(result.result.is_some());
/// ```
#[derive(Debug) ]
pub struct SequenceOf<R1,R2,T> {
   parsers: Vec<Box<dyn Parse<R1,R2,T>>>
}

impl<R1,R2,T> SequenceOf<R1,R2,T> 
   where R1: fmt::Debug, R2: fmt::Debug, T: fmt::Debug {

   pub fn new(parsers: Vec<Box<dyn Parse<R1,R2,T>>>) -> Self {
      if parsers.is_empty() {
         panic!("SequenceOf: parsers must not be empty")
      }

      Self { parsers }
   }

   pub fn push_parser(&mut self, parser: Box<dyn Parse<R1,R2,T>>) {
      self.parsers.push(parser);
   }
}

impl<R1,R2,T> Parse<R1,R2,T> for SequenceOf<R1,R2,T> 
   where R1: Debug, R2: Debug, T:Debug {
      
   fn transform(&self, state: State<R1, T>) -> State<R2, T> {
      let contains_error = state.is_error();

      if contains_error {         
         return State::from_err_state(state)
      }

      let mut results: Vec<R2> = Vec::with_capacity(self.parsers.len());

      let mut final_state: State<R1, T> = State {
         index: state.index,
         target: Rc::clone(&state.target),
         result: None
      };

      let target = Rc::clone(&state.target);

      for parser in &self.parsers {
         let state = parser.transform(final_state);

         match state.result.unwrap() {
            Ok(One(res)) => results.push(res),
            Ok(Many(mut res)) => results.append(&mut res),
            Err(err) => {
               let err_state = State {
                  index: state.index,
                  target: state.target,
                  result: Some(Err(err))
               };

               return err_state;
            }
         }

         final_state = State {
            index: state.index,
            target: Rc::clone(&target),
            result: None
         }
      }


      let res = State { 
         index: final_state.index, 
         target, 
         result: Some(Ok(Many(results))) 
      };

      res
   }
}

#[cfg(test)]
mod tests {
   use crate::parsers::str_parser::Str;
   use super::*;

   #[test]
   fn test_success() {
      let s1 = Box::new(Str::new("Test1".to_owned()));
      let s2 = Box::new(Str::new("Test2".to_owned()));
      let seq = SequenceOf::new(vec![s1,s2]);
      let result = seq.run("Test1Test2");

      assert!(result.result.is_some());
      assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 2);
      assert_eq!(result.index, 10);
   }

   #[test]
   fn test_fail() {
      let s1 = Box::new(Str::new("Test1".to_owned()));
      let s2 = Box::new(Str::new("Test2".to_owned()));
      let seq = SequenceOf::new(vec![s1,s2]);
      let result = seq.run("Test1Test3");

      assert!(result.result.is_some());
      assert!(result.result.unwrap().is_err());
      assert_eq!(result.index, 5);
   }
}
