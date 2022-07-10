use super::state::{State, ParserResult};

type StringState = State<String, String, String>; 


pub struct Str 
{
   pub transformer_fn: Box<dyn FnMut(StringState) -> StringState>,
}

impl Str 
{
   pub fn new() -> Self {

      let transformer_fn = |state : StringState| -> StringState {
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
            
         }

         state
      };

      let transformer_fn = Box::new(transformer_fn);

      Self { transformer_fn }
   }
}


pub trait Parser {
   type R1;
   type R2;
   type T;
   type E1;
   type E2;
   type P: Parser;
   type TransformerFn: FnMut(State<Self::R1, Self::T, Self::E1>) -> State<Self::R2,Self::T,Self::E2>;

   fn transform(transform_fn: Self::TransformerFn) -> State<Self::R2, Self::T, Self::E2>;
   fn run(target: Self::T) -> State<Self::R2, Self::T, Self::E2>;
}

// pub trait Chain {
//    type R2;
//    type T;
//    type E1;
//    type E2;
//    type P : Parser;

//    fn chain<S, F>(chain_fn: F) -> Self::P
//       where F: FnMut(ParserResult<Self::R2, Self::E2>) -> Self::P;
// }

