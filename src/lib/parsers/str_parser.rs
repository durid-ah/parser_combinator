use crate::models::{parser_trait::Parse, state::State, cardinality::Cardinality};

type StringState = State<String, String, String>;

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
         target: state.target, 
         result: Some(Err(String::from(""))) // TODO: Add error message
      }
   }

   fn run(&mut self, target: String) -> StringState {
      let initial_state = State{target, index: 0, result: None };
      return self.transform(initial_state);
   }
}
