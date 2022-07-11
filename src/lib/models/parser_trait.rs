
use super::{state::{State}, cardinality::Cardinality};

type StringState = State<String, String, String>;

pub struct  Str {
   pub to_match: String
}

impl Str {
   pub fn new(to_match: String) -> Self {
      Self { to_match }
   }
}

impl Parse for Str {
   type R1 = String;
   type R2 = String;
   type T = String;
   type E1 = String;
   type E2 = String;

   fn transform(&self, state: StringState) -> StringState {
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

   fn run(&self, target: String) -> StringState {
      let initial_state = State{target, index: 0, result: None };
      return self.transform(initial_state);
   }
}

pub trait Parse {
   type R1;
   type R2;
   type T;
   type E1;
   type E2;

   fn transform(&self, state: State<Self::R1, Self::T, Self::E1>) -> State<Self::R2, Self::T, Self::E2>;
   fn run(&self, target: Self::T) -> State<Self::R2, Self::T, Self::E2>;
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

pub trait Map {
   type R1;
   type R2;
   type T;
   type E1;
   type E2;

   fn map_ok<S>(&self) -> Box<dyn Parse<R1=Self::R2, R2=S, T=Self::T, E1 = Self::E1, E2 = Self::E2>>;
   fn map_err<E>(&self) -> Box<dyn Parse<R1=Self::R1, R2=Self::R2, T=Self::T, E1 = Self::E1, E2 = E>>;
}

#[cfg(test)]
mod tests {
   //  use super::Parser;


   // #[test]
   // fn test_init() {
   //    let p: Parser<String, String, String, String, String> = 
   //       Parser::new(Box::new(|x| x ));
   // }
}