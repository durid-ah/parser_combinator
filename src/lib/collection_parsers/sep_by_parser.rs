use std::rc::Rc;

use crate::models::cardinality::Cardinality::{One, Many};
use crate::models::parser_traits::Parse;
use crate::models::state::State;

// TODO: Test

pub struct SepBy<R1,R2,T>{
   separator: Box<dyn Parse<R1, R2, T>>,
   separated: Box<dyn Parse<R1,R2,T>>
}

impl<R1,R2,T> SepBy<R1,R2,T> {
   pub fn new(separator: Box<dyn Parse<R1, R2, T>>, separated: Box<dyn Parse<R1,R2,T>>) -> Self {
      Self { separator, separated }
   } 
}

impl<R1,R2,T> Parse<R1,R2,T> for SepBy<R1,R2,T> {
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

         match thing_we_want_state.result.unwrap() {
            Ok(One(res)) => results.push(res),
            Ok(Many(mut res)) => results.append(&mut res),
            Err(_) => {
               final_state = State {
                  index: state.index,
                  target: Rc::clone(&target),
                  result: None,
               };
               break;
            }
         }

         final_state = State {
            index: thing_we_want_state.index,
            target: Rc::clone(&target),
            result: None,
         };

         let separator_state = self.separator.transform(final_state);
         if separator_state.result.unwrap().is_err() {
            final_state = State {
               index: thing_we_want_state.index,
               target: Rc::clone(&target),
               result: None,
            };
            break;
         }

         final_state = State {
            index: thing_we_want_state.index,
            target: Rc::clone(&target),
            result: None,
         };
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
   use super::*;

   #[test]
   fn test_name() {
       
   }
}