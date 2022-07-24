use std::rc::Rc;
use super::cardinality::Cardinality;

pub type  ParserResult<R, E> = Option<Result<Cardinality<R>, E>>;

#[derive(Clone)]
pub struct State<R, T, E> {
   pub index: usize,
   pub target: Rc<T>,
   pub result: ParserResult<R, E>
}

impl<R, T, E> State<R, T, E> {

   pub fn new_err(self, err: E) -> Self {
      Self {
         index : self.index,
         target: self.target,
         result: Some(Err(err))
      }
   }

   pub fn from_err_state<R2>(state: State<R2,T,E>) -> Self {
      if state.result.is_none() {
         panic!("from_err_state: result can't be none")
      }

      if state.result.as_ref().unwrap().is_ok() {
         panic!("from_err_state: result can't be ok")
      }

      let err_res: Result<Cardinality<R>, E> = match  state.result.unwrap() {
         Err(err) => Err(err),
         _ => panic!("from_err_state: result must be err")
      };

      Self { index: state.index, target: Rc::clone(&state.target), result: Some(err_res) }
   }
}