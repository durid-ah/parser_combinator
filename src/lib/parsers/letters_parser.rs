use std::fmt;

use regex::Regex;

use crate::models::{parser_traits::Parse, state::State};
use crate::models::cardinality::Cardinality::One;

#[derive(Clone)]
pub struct Letters {
   regex_matcher: Regex
}

impl Letters {
   pub fn new() -> Self {
      Letters { regex_matcher: Regex::new(r"^[A-Za-z]+").unwrap() }
   } 
}

impl Default for Letters {
   fn default() -> Self { Self::new() }
}

impl fmt::Debug for Letters {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Letters").finish()
   }
}

impl Parse<String,String,String> for Letters {
   fn transform(&self, state: State<String, String>) -> State<String, String> {
      println!("{:?}", self);
      println!("\t{:?}", state);
      println!("");
      let contains_error = state.is_error();

      if contains_error {
         return state;
      }

      if state.index >= state.target.len() {
         return State {
            index: state.index,
            target: state.target,
            result: Some(Err("Letters: Unexpected end of input".to_owned()))
         }
      }

      let match_result = self.regex_matcher.find_at(&state.target, state.index);

      if match_result.is_none() {
         return State {
            index: state.index,
            target: state.target,
            result: Some(Err(format!("Letters: No letters were matched at index: {}", state.index)))
         }
      }

      let match_val = match_result.unwrap();
      return State { 
         index: state.index + match_val.end(), 
         target: state.target.clone(), // TODO: Work on clone 
         result: Some(Ok(One(match_val.as_str().to_owned()))) 
      }
   }
}

#[cfg(test)]
mod tests {
    use crate::models::parser_traits::Parse;
    use super::Letters;

   #[test]
   fn digit_success_run() {
      let p = Letters::new();
      let res = p.run("abcd1s".to_owned());
      assert!(res.result.unwrap().unwrap().unwrap_one() == "abcd");
      assert!(res.index == 4);
   }

   #[test]
   fn digit_fail_run() {
      let p = Letters::new();
      let res = p.run("123s".to_owned());
      assert!(res.result.unwrap().is_err());
      assert!(res.index == 0);
   }
}