use std::{rc::Rc, error::Error};
use super::cardinality::Cardinality;

pub type  ParserResult<R, E> = Option<Result<Cardinality<R>, E>>;

#[derive(Clone)]
pub struct State<R, T> {
   pub index: usize,
   pub target: Rc<T>,
   pub result: ParserResult<R, dyn Error>
}

impl<R, T> State<R, T> {

   pub fn new_err<E: Error>(self, err: E) -> Self {
      Self {
         index : self.index,
         target: self.target,
         result: Some(Err(err))
      }
   }

   pub fn from_err_state<R2>(state: State<R2,T>) -> Self {
      if state.result.is_none() {
         panic!("from_err_state: result can't be none")
      }

      if state.result.as_ref().unwrap().is_ok() {
         panic!("from_err_state: result can't be ok")
      }

      let err_res: Result<Cardinality<R>, dyn Error> = match  state.result.unwrap() {
         Err(err) => Err(err),
         _ => panic!("from_err_state: result must be err")
      };

      Self { index: state.index, target: Rc::clone(&state.target), result: Some(err_res) }
   }

   /// Check if the result is an error type. Returns `false` if 
   /// the result is `Ok()` or `None`
   pub fn is_error(&self) -> bool {
      self.result
         .as_ref()
         .and_then(|r| Some(r.is_err()))
         .unwrap_or(false)
   }
}