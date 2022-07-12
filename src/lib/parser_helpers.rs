use crate::models::{state::{ParserResult, State}, parser::Parser, parser_traits::Parse};


fn map_ok<'a, R1,R2,T,E1,E2,S,F,I>(mut parser: I, mut map_fn: F) -> Parser<'a, R1, S, T, E1, E2>
   where F: FnMut(ParserResult<R2, E2>) -> ParserResult<S,E2>,
         I: Parse<R1,R2,T,E1,E2>,
         F: 'a,
         I: 'a {

   let transformer = move |state: State<R1,T, E1>| {
      let next = parser.transform(state);

      let result = map_fn(next.result);
      State{ index: next.index, target: next.target, result }
   };

   Parser::new(Box::new(transformer))
}


// TODO: Document
// TODO: Test