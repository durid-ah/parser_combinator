use std::rc::Rc;
use crate::models::parser_traits::Parse;
use crate::models::state::State;
use crate::models::cardinality::Cardinality;

// TODO: Test out parser

pub struct ManyOne<R1, R2, T> {
   parser: Box<dyn Parse<R1, R2, T>>
}

impl<R1, R2, T> ManyOne<R1, R2, T> {
   pub fn new(parser: Box<dyn Parse<R1, R2, T>>) -> Self {
      Self { parser }
   }
}

impl<R1, R2, T> Parse<R1, R2, T> for ManyOne<R1, R2, T> {
   fn transform(&mut self, state: State<R1, T>) -> State<R2, T> {

      let mut results: Vec<R2> = Vec::new();
      let target = Rc::clone(&state.target);
      let mut final_state: State<R1, T> = State {
         index: state.index,
         target: Rc::clone(&state.target),
         result: state.result
      };

      let done = false;
      while !done {
         let state = self.parser.transform(final_state);
         
         match state.result.unwrap() {
            Ok(Cardinality::One(res)) => results.push(res),
            Ok(Cardinality::Many(mut res)) => results.append(&mut res),
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

      if results.len() == 0 {
         return final_state
            .new_err("manyOne: Unable to match any input using parser @ index".to_owned())
      }

      return State { index: state.index, target, result: Some(Ok(Cardinality::Many(results))) }
   }
}