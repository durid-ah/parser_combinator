use crate::models::{state::{ParserResult, State}, parser::Parser, parser_traits::Parse};


/// Maps the result of a parser that implements the [`Parse`] trait and creates a
/// generic [`Parser`] that will run the previous parser and return a state with 
/// the mapped result
pub fn map_result<'a, R1, R2, T, S, F, I>(parser: I, map_fn: F) -> Parser<'a, R1, S, T>
   where F: Fn(ParserResult<R2>) -> ParserResult<S> + 'a,
         I: Parse<R1,R2,T> + 'a,
         R1: 'a, R2: 'a, T: 'a, S: 'a {

   let transformer = move |state: State<R1,T>| {
      let next = parser.transform(state);

      let result = map_fn(next.result);
      State{ index: next.index, target: next.target, result }
   };

   Parser::new(Box::new(transformer))
}


/// Chains to parsers to each other through a closure that takes in the result 
/// of the first parser and returns a struct that implements the [`Parse`] trait.
/// Returns a new generic [`Parser`] that executes the parsers after each other 
pub fn chain_parser<'a,I,F,R1,R2,T, S>(parser: I, chain_fn: F) -> Box<dyn Parse<R1, S, T> + 'a>
   where F: Fn(& ParserResult<R2>) -> Box<dyn Parse<R2, S, T> + 'a> + 'a,
         I: Parse<R1,R2,T> + 'a,
         R1: 'a, R2: 'a, T: 'a, S: 'a {

   let transformer = move |state: State<R1, T>| {
      let next = parser.transform(state);
      let next_parser = chain_fn(&next.result);

      next_parser.transform(next)
   };

   Box::new(Parser::new(Box::new(transformer)))
} 

#[cfg(test)]
mod tests {
    use crate::parsers::str_parser::Str;

    use super::chain_parser;

   #[test]
   fn chain_test() {
      let str_1 = Str::new("Stuff".to_owned());
      let str_2 = Str::new("Stuff".to_owned());

      let chained = 
         chain_parser(str_1, move |_| Box::new(str_2.clone()));

      let res = chained.run("StuffStuff");
      assert!(res.result.unwrap().is_ok())
   }
}

