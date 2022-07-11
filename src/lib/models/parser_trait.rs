
use super::{state::{State, ParserResult}, cardinality::Cardinality, parse_test::Parser};

type StringState = State<String, String, String>;

pub struct  Str {
   pub to_match: String
}

impl Str {
   pub fn new(to_match: String) -> Self {
      Self { to_match }
   }
}

impl Transform<String,String,String,String,String> for Str {

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


fn map_ok<'a, R1,R2,T,E1,E2,S,F>(parser: Box<dyn Transform<R1,R2,T,E1,E2>>, map_fn: F)
   where F: FnMut(ParserResult<R2, E2>) -> Parser<'a,R1,S,T,E1,E2> {

   todo!("Finished polishing the Parser struct")
}


pub trait Transform<R1,R2,T,E1,E2> {
   fn transform(&mut self, state: State<R1, T, E1>) -> State<R2, T, E2>;
   fn run(&mut self, target: T) -> State<R2, T, E2>;
}


// pub trait Chain {
//    type R2;
//    type T;
//    type E1;
//    type E2;
//    type P : Parser;

//    fn chain<S, F>(&self, chain_fn: F) -> Self::P
//       where F: FnMut(ParserResult<Self::R2, Self::E2>) -> Box<dyn Parser<R1=Self::R2, R2=S, T=Self::T, E1 = Self::E1, E2 = Self::E2>>;
// }


#[cfg(test)]
mod tests {
}