use crate::models::{parser_traits::Parse, state::State, cardinality::Cardinality};

type StringState = State<String, String, String>;

#[derive(Clone)]
pub struct  Str {
   pub to_match: String
}

impl Str {
   pub fn new(to_match: String) -> Self {
      Self { to_match }
   }
}

impl Parse<String,String,String,String,String> for Str {

   fn transform(&mut self, state: StringState) -> StringState {
      let contains_error = state.result
         .as_ref()
         .and_then(|r| Some(r.is_err()))
         .unwrap_or(false);
         
      if contains_error {
         return state;
      }

      let start_index = state.index;
      let sliced_target = &state.target.as_str()[start_index..];
      if sliced_target.len() == 0 {
         return state.new_err(String::from("Str: Unexpected end of input"))
      }

      if sliced_target.starts_with(self.to_match.as_str()) {
         return State {
            target: state.target,
            index: start_index + self.to_match.len(),
            result: Some(Ok(Cardinality::One(self.to_match.clone())))
         }
      }

      return State { 
         index: start_index, 
         target: state.target.clone(), // TODO: Change to a ref to prevent copies
         result: Some(Err(format!("Str: Tried to match {}, but got {}", self.to_match, state.target)))
      }
   }

   fn run(&mut self, target: String) -> StringState {
      let initial_state = State{target, index: 0, result: None };
      return self.transform(initial_state);
   }
}

#[cfg(test)]
mod tests {
   use crate::models::parser_traits::Parse;
   use super::Str;

   #[test]
   fn str_success_exact_parse() {
      let mut parser = Str::new("Test".to_owned());
      let res = parser.run("Test".to_owned());
      assert!(res.result.unwrap().is_ok());
      assert_eq!(res.index, 4);
   }

   #[test]
   fn str_success_partial_parse() {
      let mut parser = Str::new("Test".to_owned());
      let res = parser.run("Tester".to_owned());
      assert!(res.result.unwrap().is_ok());
      assert_eq!(res.index, 4);
   }

   #[test]
   fn str_fail_no_match_parse() {
      let mut parser = Str::new("Test".to_owned());
      let res = parser.run("Abcde".to_owned());
      assert!(res.result.unwrap().is_err());
      assert_eq!(res.index, 0);
   }

   #[test]
   fn str_fail_short_target_parse() {
      let mut parser = Str::new("Test".to_owned());
      let res = parser.run("T".to_owned());
      assert!(res.result.unwrap().is_err());
   }
}
