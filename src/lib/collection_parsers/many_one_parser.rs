use crate::models::cardinality::Cardinality;
use crate::models::parser_traits::Parse;
use crate::models::state::State;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

/// # ManyOne:
/// Attempts to parse at least one instance of the specified Parser
/// the parser will run until it encounters an error in the specified target
/// if the parser does not find any values it will return an error
/// 
/// To parse zero or more see [`super::many_parser::Many`] 
/// 
/// ### Returns:
/// A result of type [`Cardinality::Many`]
///
/// ### Examples
///
/// Basic Usage:
///
/// ```
/// use parser_combinator::collection_parsers::many_one_parser::ManyOne;
/// use parser_combinator::parsers::str_parser::Str;
/// use parser_combinator::models::parser_traits::Parse;
///
/// let str_parser = Str::new("Test".to_owned());
/// let many = ManyOne::new(str_parser);
/// let result = many.run("TestTestTest");
/// assert!(result.result.is_some());
/// assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 3);
/// assert_eq!(result.index, 12);
/// ```
#[derive(Debug)]
pub struct ManyOne<I, R1, R2, T>
    where I: Parse<R1, R2, T> {
        parser: I,
        _p1: PhantomData<R1>,
        _p2: PhantomData<R2>,
        _p3: PhantomData<T>,
}

impl<I, R1, R2, T> ManyOne<I, R1, R2, T> 
    where I: Parse<R1, R2, T> {

    pub fn new(parser: I) -> Self {
        Self { parser, _p1: PhantomData, _p2: PhantomData, _p3: PhantomData }
    }
}

impl<I, R1, R2, T> Parse<R1, R2, T> for ManyOne<I, R1, R2, T> 
    where R1: fmt::Debug, R2: fmt::Debug, T: fmt::Debug, 
        I: Parse<R1,R2,T> {    

   fn transform(&self, state: State<R1, T>) -> State<R2, T> {

      let mut results: Vec<R2> = Vec::new();
      let target = Rc::clone(&state.target);
      let mut final_state: State<R1, T> = State {
         index: state.index,
         target: Rc::clone(&state.target),
         result: state.result,
      };

      let mut done = false;
      while !done {
         let state = self.parser.transform(final_state);

         match state.result.unwrap() {
             Ok(Cardinality::One(res)) => results.push(res),
             Ok(Cardinality::Many(mut res)) => results.append(&mut res),
             Err(_) => done = true,
         }

         final_state = State {
            index: state.index,
            target: Rc::clone(&target),
            result: None,
         }
      }

      if results.is_empty() {
         let err_state = final_state
            .new_err("ManyOne: Unable to match any input using parser @ index".to_owned());

         return err_state;
      }

      let res = State {
         index: final_state.index,
         target,
         result: Some(Ok(Cardinality::Many(results))),
      };


      res
   }
}

#[cfg(test)]
mod tests {
    use super::ManyOne;
    use crate::models::parser_traits::Parse;
    use crate::parsers::str_parser::Str;

    #[test]
    fn many_one_parser_full_run() {
        let str_parser = Str::new("Test".to_owned());
        let many_one = ManyOne::new(str_parser);
        let result = many_one.run("TestTestTest");
        
        assert!(result.result.is_some());
        assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 3);
        assert_eq!(result.index, 12);
    }

    #[test]
    fn many_one_parser_partial_parse() {
        let str_parser = Str::new("Test".to_owned());
        let many_one = ManyOne::new(str_parser);
        let result = many_one.run("TestStuffTest");
        assert!(result.result.is_some());
        assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 1);
        assert_eq!(result.index, 4);
    }

    #[test]
    fn many_one_parser_zero_fail() {
        let str_parser = Str::new("Test".to_owned());
        let many_one = ManyOne::new(str_parser);
        let result = many_one.run("StuffTest");
        assert!(result.result.is_some());
        assert!(result.result.unwrap().is_err());
        assert_eq!(result.index, 0);
    }
}
