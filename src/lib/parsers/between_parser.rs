use std::fmt::{Debug, self};

use crate::models::parser::Parser;
use crate::models::parser_traits::Parse;
use crate::models::state::{ State, ParserResult };
use crate::parser_helpers::map_result;
use crate::models::cardinality::Cardinality;
use crate::collection_parsers::sequence_of_parser::SequenceOf;

/// # Between
/// Parse the value(s) between two other parsers
/// 
/// ### Returns:
/// The result of the middle parser (value) or the error returned by any of the
/// parsers (left, value, right)
#[derive(Debug)]
pub struct Between<'a,R1,R2,T> {
   parser: Parser<'a,R1,R2,T>
}

impl<'a, R1,R2,T> Between<'a,R1,R2,T> 
   where R2: 'a + Debug, R1: 'a + Debug, T: 'a + Debug {
   pub fn new(
      left: Box<dyn Parse<R1,R2,T>>,
      right: Box<dyn Parse<R1,R2,T>>,
      value: Box<dyn Parse<R1,R2,T>>) -> Self {

         let parser =  SequenceOf::new(vec![left, value, right]);
         let parser = map_result::<'a>(parser, |opt: ParserResult<R2>| {
            opt.map(move |res| {
               res.map(move|card| {
                  let mut res_vec = card.unwrap_many();
                  let result: Vec<R2> = res_vec
                     .drain(1..res_vec.len() - 1)
                     .collect();

                  Cardinality::Many(result)
               })
            })
         });

         Self { parser }
   }
}


impl<'a,R1,R2,T> Parse<R1,R2,T> for Between<'a,R1,R2,T>  
   where R1: fmt::Debug, R2: fmt::Debug, T: fmt::Debug {

   fn transform(&self, state: State<R1, T>) -> State<R2, T> {      
      let res = self.parser.transform(state);
      
      res
   }
}

#[cfg(test)]
mod tests {
   use crate::parsers::str_parser::Str;
   use super::*;

   #[test]
   fn test_success() {
      let left = Box::new(Str::new("(".to_owned()));
      let value = Box::new(Str::new("test".to_owned()));
      let right = Box::new(Str::new(")".to_owned()));

      let bet = Between::new(left, right, value);
      let result = bet.run("(test)");

      assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 1);
      assert_eq!(result.index, 6);       
   }

   #[test]
   fn test_fail() {
      let left = Box::new(Str::new("(".to_owned()));
      let value = Box::new(Str::new("test".to_owned()));
      let right = Box::new(Str::new(")".to_owned()));

      let bet = Between::new(left, right, value);
      let result = bet.run("(Test)");

      assert!(result.result.unwrap().is_err());
      assert_eq!(result.index, 1);       
   }
}