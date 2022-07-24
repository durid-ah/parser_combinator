use std::rc::Rc;

use crate::models::{parser_traits::Parse, state::State};
use crate::models::cardinality::Cardinality::{One, Many};

pub struct SequenceOf<R1,R2,T,E> {
   parsers: Vec<Box<dyn Parse<R1,R2,T,E,E>>>
}

impl<R1,R2,T,E> SequenceOf<R1,R2,T,E> {
   pub fn new(parsers: Vec<Box<dyn Parse<R1,R2,T,E,E>>>) -> Self {
      if parsers.is_empty() {
         panic!("SequenceOf: parsers must not be empty")
      }

      Self { parsers }
   }
}

impl<R1,R2,T,E> Parse<R1,R2,T,E,E> for SequenceOf<R1,R2,T,E> {
   fn transform(&mut self, state: State<R1, T, E>) -> State<R2, T, E> {
      let contains_error = state.result
         .as_ref()
         .and_then(|r| Some(r.is_err()))
         .unwrap_or(false);

      if contains_error {
         return State::from_err_state(state)
      }

      let mut results: Vec<R2> = Vec::with_capacity(self.parsers.len());

      let mut final_state: State<R1, T, E> = State {
         index: state.index,
         target: Rc::clone(&state.target),
         result: None
      };

      let target = Rc::clone(&state.target);

      for parser in &mut self.parsers {
         let state = parser.transform(final_state);

         match state.result.unwrap() {
            Ok(One(res)) => results.push(res),
            Ok(Many(mut res)) => results.append(&mut res),
            Err(err) => {
               return State {
                  index: state.index,
                  target: state.target,
                  result: Some(Err(err))
               }
            }
         }

         final_state = State {
            index: state.index,
            target: Rc::clone(&target),
            result: None
         }
      }

      return State { index: state.index, target, result: Some(Ok(Many(results))) }

   }
}

