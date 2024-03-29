use std::{rc::Rc, fmt::Debug};
use super::cardinality::Cardinality;

pub type  ParserResult<R> = Option<Result<Cardinality<R>, String>>;

/// # State
/// Represents the state returned from the parser 
#[derive(Clone)]
pub struct State<R, T> {
   /// The index where the parser will start from
   pub index: usize,
   /// The target data that will be parsed
   pub target: Rc<T>,
   pub result: ParserResult<R>
}

impl<R: Debug, T: Debug> State<R, T> {
   // Create an error from the existing state
   pub fn new_err<R2>(self, err: String) -> State<R2, T> {
      State {
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

      let err_res: Result<Cardinality<R>, String> = match  state.result.unwrap() {
         Err(err) => Err(err),
         _ => panic!("from_err_state: result must be err")
      };

      Self { index: state.index, target: Rc::clone(&state.target), result: Some(err_res) }
   }

   /// Check if the result is an error type. Returns `false` if 
   /// the result is `Ok()` or `None`
   pub fn is_error(&self) -> bool {
      self.result
         .as_ref().map(|r| r.is_err())
         .unwrap_or(false)
   }
}

impl<R: Debug, T: Debug> Debug for State<R, T> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("State")
         .field("index", &self.index)
         .field("target", &self.target)
         .field("result", &self.result).finish()
   }
}