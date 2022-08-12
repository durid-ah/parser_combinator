use std::rc::Rc;

use crate::models::cardinality::Cardinality::{One, Many};
use crate::models::parser_traits::Parse;
use crate::models::state::State;

/// # SepByOne:
/// Parse at least one value separated by a `separator` value
/// 
/// ### Returns:
/// A result of type [`Many`]
///
/// ### Examples
///
/// Basic Usage:
///
/// ```
/// use parser_combinator::collection_parsers::sep_by_one_parser::SepByOne;
/// use parser_combinator::parsers::str_parser::Str;
/// use parser_combinator::models::parser_traits::Parse;
///
/// let comma = Str::new(",".to_owned());
/// let test_string = Str::new("Test".to_owned());
/// let mut sep_parser = SepByOne::new(Box::new(comma), Box::new(test_string));
/// let result = sep_parser.run("Test,Test,Test");
/// 
/// assert!(result.result.is_some());
/// assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 3);
/// assert_eq!(result.index, 14);
/// ```
pub struct SepByOne<R1,R2,T>{
   separator: Box<dyn Parse<R1, R2, T>>,
   separated: Box<dyn Parse<R1,R2,T>>
}

impl<R1,R2,T> SepByOne<R1,R2,T> {
   pub fn new(separator: Box<dyn Parse<R1, R2, T>>, separated: Box<dyn Parse<R1,R2,T>>) -> Self {
      Self { separator, separated }
   } 
}

impl<R1,R2,T> Parse<R1,R2,T> for SepByOne<R1,R2,T> {
   fn transform(&mut self, state: State<R1, T>) -> State<R2, T> {
      let contains_error = state.is_error();
      if contains_error {
         return State::from_err_state(state);
      }

      let target = Rc::clone(&state.target);
      let mut results: Vec<R2> = Vec::new();
      let mut final_state: State<R1, T> = State {
         index: state.index,
         target: Rc::clone(&state.target),
         result: None
      };

      loop {
         let thing_we_want_state = self.separated.transform(final_state);
         final_state = State {
            index: thing_we_want_state.index,
            target: Rc::clone(&target),
            result: None,
         };

         match thing_we_want_state.result.unwrap() {
            Ok(One(res)) => results.push(res),
            Ok(Many(mut res)) => results.append(&mut res),
            Err(_) => break
         }

         let separator_state = self.separator.transform(final_state);
         final_state = State {
            index: separator_state.index,
            target: Rc::clone(&target),
            result: None,
         };

         if separator_state.result.unwrap().is_err() {
            break;
         }
      }

      if results.len() == 0 {
         return final_state
            .new_err("manyOne: Unable to match any input using parser @ index".to_owned());
      }

      return State {
         index: final_state.index,
         target,
         result: Some(Ok(Many(results))),
     };
   }
}

#[cfg(test)]
mod tests {
   use crate::parsers::str_parser::Str;
   use super::*;

   #[test]
   fn success() {
      let comma = Str::new(",".to_owned());
      let test_string = Str::new("Test".to_owned());
      let mut sep_parser = SepByOne::new(Box::new(comma), Box::new(test_string));
      let result = sep_parser.run("Test,Test,Test");

      assert!(result.result.is_some());
      assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 3);
      assert_eq!(result.index, 14);
   }

   #[test]
   fn ends_with_separator_success() {
      let comma = Str::new(",".to_owned());
      let test_string = Str::new("Test".to_owned());
      let mut sep_parser = SepByOne::new(Box::new(comma), Box::new(test_string));
      let result = sep_parser.run("Test,Test,");

      assert!(result.result.is_some());
      assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 2);
      assert_eq!(result.index, 10);
   }

   #[test]
   fn empty_fail() {
      let comma = Str::new(",".to_owned());
      let test_string = Str::new("Test".to_owned());
      let mut sep_parser = SepByOne::new(Box::new(comma), Box::new(test_string));
      let result = sep_parser.run("");

      assert!(result.result.is_some());
      assert!(result.result.unwrap().is_err());
      assert_eq!(result.index, 0);
   }
}