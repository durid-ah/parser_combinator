use std::rc::Rc;

use crate::models::{parser_traits::Parse, state::State};



pub struct Choice<R1,R2,T> {
   parsers: Vec<Box<dyn Parse<R1,R2,T>>>
}

impl<R1,R2,T> Choice<R1,R2,T> {
   pub fn new(parsers: Vec<Box<dyn Parse<R1,R2,T>>>) -> Self {
      Self { parsers }
   } 
}

impl<R1,R2,T> Parse<R1,R2,T> for Choice<R1,R2,T> {
   fn transform(&self, state: State<R1, T>) -> State<R2, T> {
     State {
         index: 0,
         target: Rc::clone(&state.target),
         result: None
     }
   }
}

