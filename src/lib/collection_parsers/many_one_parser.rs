use std::rc::Rc;
use crate::models::parser_traits::Parse;
use crate::models::state::State;
use crate::models::cardinality::Cardinality;

pub struct ManyOne<R1, R2, T, E1, E2> {
   parser: Box<dyn Parse<R1, R2, T, E1, E2>>
}

impl<R1, R2, T, E1, E2> ManyOne<R1, R2, T, E1, E2> {
   pub fn new(parser: Box<dyn Parse<R1, R2, T, E1, E2>>) -> Self {
      Self { parser }
   }
}

impl<R1, R2, T, E1, E2> Parse<R1, R2, T, E1, E2> for ManyOne<R1, R2, T, E1, E2> {
   fn transform(&mut self, state: State<R1, T, E1>) -> State<R2, T, E2> {

      let mut results: Vec<R2> = Vec::new();
      let target = Rc::clone(&state.target);
      let mut final_state: State<R1, T, E1> = State {
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

      return State { index: state.index, target, result: Some(Ok(Cardinality::Many(results))) }
   }
}