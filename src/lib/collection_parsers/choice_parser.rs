use std::{rc::Rc, fmt};

use crate::{
   models::{parser_traits::Parse, state::State}, 
   utility::local_log
};

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
      local_log::log(format!("{:?}", state));

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

