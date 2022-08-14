use regex::Regex;

use crate::models::{parser_traits::Parse, state::State};
use crate::models::cardinality::Cardinality::One;

use super::str_parser::StringState;

#[derive(Clone)]
pub struct Digits {
   regex_matcher: Regex
}

impl Digits {
   pub fn new() -> Self {
      Digits { regex_matcher: Regex::new(r"^[0-9]+").unwrap() }
   }
}

impl Default for Digits {
   fn default() -> Self { Self::new() }
}

impl Parse<String,String,&str> for Digits {
   fn transform<'s>(&self, state: StringState<'s>) -> StringState<'s> {
      let contains_error = state.is_error();

      if contains_error {
         return state;
      }

      if state.index >= state.target.len() {
         return State {
            index: state.index,
            target: state.target,
            result: Some(Err("Digits: Unexpected end of input".to_owned()))
         }
      }

      let match_result = self.regex_matcher.find_at(&state.target, state.index);

      if match_result.is_none() {
         return State {
            index: state.index,
            target: state.target,
            result: Some(Err(format!("Digits: No digits were matched at index: {}", state.index)))
         }
      }

      let match_val = match_result.unwrap();
      return State {
         index: state.index + match_val.end(), 
         target: state.target.clone(),
         result: Some(Ok(One(match_val.as_str().to_owned()))) 
      }
   }
}

#[cfg(test)]
mod tests {
    use crate::models::parser_traits::Parse;
    use super::Digits;

   #[test]
   fn digit_success_run() {
      let p = Digits::new();
      let res = p.run("123s");
      assert!(res.result.unwrap().unwrap().unwrap_one() == "123");
      assert!(res.index == 3);
   }

   #[test]
   fn digit_fail_run() {
      let p = Digits::new();
      let res = p.run("s123s");
      assert!(res.result.unwrap().is_err());
      assert!(res.index == 0);
   }
}