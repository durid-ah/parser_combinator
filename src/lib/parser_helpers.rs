use crate::models::{state::{ParserResult, State}, parser::Parser, parser_traits::Parse};


/// Maps the result of a parser that implements the [`Parse`] trait and creates a
/// generic [`Parser`] that will run the previous parser and return a state with 
/// the mapped result
pub fn map_result<'a, R1, R2, T, E1, E2, S, F, I>(mut parser: I, mut map_fn: F) -> Box<dyn Parse< R1, S, T, E1, E2> + 'a>
   where F: FnMut(ParserResult<R2, E2>) -> ParserResult<S,E2> + 'a,
         I: Parse<R1,R2,T,E1,E2> + 'a,
         R1: 'a, R2: 'a, T: 'a, E1: 'a, E2: 'a, S: 'a {

   let transformer = move |state: State<R1,T, E1>| {
      let next = parser.transform(state);

      let result = map_fn(next.result);
      State{ index: next.index, target: next.target, result }
   };

   Box::new(Parser::new(Box::new(transformer)))
}


/// Chains to parsers to each other through a closure that takes in the result 
/// of the first parser and returns a struct that implements the [`Parse`] trait.
/// Returns a new generic [`Parser`] that executes the parsers after each other 
pub fn chain_parser<'a,I,F,R1,R2,T,E1,E2,S>(mut parser: I, mut chain_fn: F) -> Box<dyn Parse<R1, S, T, E1, E2> + 'a>
   where F: FnMut(& ParserResult<R2, E2>) -> Box<dyn Parse<R2, S, T, E2, E2> + 'a> + 'a,
         I: Parse<R1,R2,T,E1,E2> + 'a,
         R1: 'a, R2: 'a, T: 'a, E1: 'a, E2: 'a, S: 'a {

   let transformer = move |state: State<R1, T, E1>| {
      let next = parser.transform(state);
      let mut next_parser = chain_fn(&next.result);

      next_parser.transform(next)
   };

   Box::new(Parser::new(Box::new(transformer)))
} 

// TODO: Test