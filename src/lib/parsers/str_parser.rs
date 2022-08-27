use std::rc::Rc;

use crate::models::parser_traits::Parse;
use crate::models::state::State;
use crate::models::cardinality::Cardinality;
use crate::utility::local_log;

pub type StringState<'state> = State<String, &'state str>;

/// # Str:
/// Parse a specific string in the target
#[derive(Clone, Debug)]
pub struct  Str {
   pub to_match: String
}

impl Str {
   pub fn new(to_match: String) -> Self {
      Self { to_match }
   }
}


impl Parse<String,String,&str> for Str {

   fn transform<'s>(&self, state: StringState<'s>) -> StringState<'s> {
      local_log::log(format!("{:?}", self));
      local_log::start_scope();

      let contains_error = state.is_error();
         
      if contains_error {
         local_log::log(format!("{:?}", state));
         local_log::end_scope();
         return state;
      }

      let start_index = state.index;
      let sliced_target = &state.target[start_index..];
      if sliced_target.is_empty() {
         let err_state = state.new_err(String::from("Str: Unexpected end of input"));
         
         local_log::log(format!("{:?}", err_state));
         local_log::end_scope();

         return err_state;
      }

      if sliced_target.starts_with(self.to_match.as_str()) {
         let res = State {
            target: state.target,
            index: start_index + self.to_match.len(),
            result: Some(Ok(Cardinality::One(self.to_match.clone())))
         };

         local_log::log(format!("{:?}", res));
         local_log::end_scope();

         return res;
      }

      let res = State { 
         index: start_index, 
         target: Rc::clone(&state.target),
         result: Some(Err(format!("Str: Tried to match {}, but got {}", self.to_match, state.target)))
      };

      local_log::log(format!("{:?}", res));
      local_log::end_scope();

      return res;
   }
}

#[cfg(test)]
mod tests {
   use crate::models::parser_traits::Parse;
   use super::Str;

   #[test]
   fn str_success_exact_parse() {
      let parser = Str::new("Test".to_owned());
      let res = parser.run("Test");
      assert!(res.result.unwrap().is_ok());
      assert_eq!(res.index, 4);
   }

   #[test]
   fn str_success_partial_parse() {
      let parser = Str::new("Test".to_owned());
      let res = parser.run("Tester");
      assert!(res.result.unwrap().is_ok());
      assert_eq!(res.index, 4);
   }

   #[test]
   fn str_fail_no_match_parse() {
      let parser = Str::new("Test".to_owned());
      let res = parser.run("Abcde");
      assert!(res.result.unwrap().is_err());
      assert_eq!(res.index, 0);
   }

   #[test]
   fn str_fail_short_target_parse() {
      let parser = Str::new("Test".to_owned());
      let res = parser.run("T");
      assert!(res.result.unwrap().is_err());
   }
}
