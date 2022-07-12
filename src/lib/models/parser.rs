use super::{state::State, parser_trait::Parse};

/// Parser: a generic parser container that implements the parser trait
pub struct Parser<'a, R1, R2, T, E1, E2> {
   pub transformer_fn: Box<dyn FnMut(State<R1,T,E1>) -> State<R2,T,E2> + 'a>,
}

impl<'a, R1, R2, T, E1, E2> Parser<'a, R1, R2, T, E1, E2> {
   
   /// Instantiates a `Parser` struct
   /// 
   /// # Examples
   /// 
   /// A parser that tries to match the word "cat" in the target and updates the index to the character after
   /// ```
   /// Parser::new(Box::new(|state| {
   ///    if state.target == "cat" {
   ///       return State{ 
   ///          index: 3, 
   ///          target: state.target, 
   ///          result: Some(Ok(Cardinality::One(String::from("cat"))))}
   ///    }
   /// 
   ///    return State{ 
   ///       index: state.index, 
   ///       target: state.target, 
   ///       result: Some(Err(String::from("Unable to match string 'cat'")))};
   /// }));
   /// ```
   pub fn new(transformer_fn: Box<dyn FnMut(State<R1,T,E1>) -> State<R2,T,E2> + 'a>) -> Self {
      Self { transformer_fn }
   }
}

impl<'a, R1, R2, T, E1, E2> Parse<R1, R2, T, E1, E2> for Parser<'a, R1, R2, T, E1, E2> {
   
   /// Run the parsing the logic on a `State` instance
   fn transform(&mut self, state: State<R1, T, E1>) -> State<R2, T, E2> {
      (self.transformer_fn)(state)
   }

   /// Run the parser with an initial `None` result
   fn run(&mut self, target: T) -> State<R2, T, E2> {
      let initial_state = State{target, index: 0, result: None };
      self.transform(initial_state)
   }
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

#[cfg(test)]
mod tests {
    use crate::models::{state::State, cardinality::Cardinality};

    use super::Parser;

   #[test]
   fn some_test() {
      let p: Parser<String, String, String, String, String> = 
         Parser::new(Box::new(|state| {
            if state.target == "cat" {
               return State{ 
                  index: 3, 
                  target: state.target, 
                  result: Some(Ok(Cardinality::One(String::from("cat"))))}
            }

            return State{ 
               index: state.index, 
               target: state.target, 
               result: Some(Err(String::from("Unable to match string 'cat'")))};
         }));
   }
}