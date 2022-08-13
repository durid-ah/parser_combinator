use super::{state::State, parser_traits::Parse};

/// # Parser: 
/// a generic parser container that implements the parser trait
pub struct Parser<'a, R1, R2, T> {
   pub transformer_fn: Box<dyn FnMut(State<R1,T>) -> State<R2,T> + 'a>,
}

impl<'a, R1, R2, T> Parser<'a, R1, R2, T> {
   
   /// Instantiates a `Parser` struct
   /// 
   /// # Examples
   /// 
   /// A parser that tries to match the word "cat" in the target and updates the index to the character after
   /// ```
   /// use parser_combinator::models::parser::Parser;
   /// use parser_combinator::models::state::State;
   /// use parser_combinator::models::cardinality::Cardinality;
   /// 
   /// Parser::new(Box::new(|state: State<String,String>| {
   ///    if *state.target == "cat" {
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
   pub fn new(transformer_fn: Box<dyn FnMut(State<R1,T>) -> State<R2,T> + 'a>) -> Self {
      Self { transformer_fn }
   }
}

impl<'a, R1, R2, T> Parse<R1, R2, T> for Parser<'a, R1, R2, T> {
   
   /// Run the parsing the logic on a `State` instance
   fn transform(&mut self, state: State<R1, T>) -> State<R2, T> {
      (self.transformer_fn)(state)
   }
}



#[cfg(test)]
mod tests {
    use crate::models::{state::State, cardinality::Cardinality};

    use super::Parser;

   #[test]
   fn some_test() {
      let _: Parser<String, String, String> = 
         Parser::new(Box::new(|state| {
            if *state.target == "cat" {
               return State{ 
                  index: 3, 
                  target: state.target, 
                  result: Some(Ok(Cardinality::One(String::from("cat"))))}
            }

            State{ 
               index: state.index, 
               target: state.target, 
               result: Some(Err(String::from("Unable to match string 'cat'")))}
         }));
   }

   #[test]
   fn test() {
      use crate::models::{parser::Parser, state::State, cardinality::Cardinality};
                  
      Parser::new(Box::new(|state: State<String,String>| {
         if *state.target == "cat" {
            return State{ 
               index: 3, 
               target: state.target, 
               result: Some(Ok(Cardinality::One(String::from("cat"))))}
         }
       
         State{ 
            index: state.index, 
            target: state.target, 
            result: Some(Err(String::from("Unable to match string 'cat'")))}
      }));
   }
}