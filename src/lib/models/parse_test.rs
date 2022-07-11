use super::{state::State, cardinality::Cardinality, parser_trait::Transform};

pub struct Parser<'a, R1, R2, T, E1, E2> {
   pub transformer_fn: Box<dyn FnMut(State<R1,T,E1>) -> State<R2,T,E2> + 'a>,
}

impl<'a, R1, R2, T, E1, E2> Parser<'a, R1, R2, T, E1, E2> {
   pub fn new(transformer_fn: Box<dyn FnMut(State<R1,T,E1>) -> State<R2,T,E2> + 'a>) -> Self {
      Self { transformer_fn }
   }

   // pub fn map_ok<S: 'a>(&'a mut self, mut map_fn: Box<dyn FnMut(Result<Cardinality<R2>, E2>) -> Result<Cardinality<S>, E2> + 'a>) -> Parser<'a,R1, S, T, E1, E2> {
      
   //    let transformer = move |state: State<R1,T, E1>| {
   //       let next = (self.transformer_fn)(state);

   //       let result = next.result
   //          .and_then(|r| Some(map_fn(r)));
   //       State{ index: next.index, target: next.target, result }
   //    };

   //    let transformer = Box::new(transformer);

   //    return Parser::new(transformer);
   // }
}

impl<'a, R1, R2, T, E1, E2> Transform<R1, R2, T, E1, E2> for Parser<'a, R1, R2, T, E1, E2> {
    
   fn transform(&mut self, state: State<R1, T, E1>) -> State<R2, T, E2> {
      (self.transformer_fn)(state)
   }

   fn run(&mut self, target: T) -> State<R2, T, E2> {
      todo!()
   }
}