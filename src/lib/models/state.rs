use super::cardinality::Cardinality;

pub type  ParserResult<R, E> = Option<Result<Cardinality<R>, E>>;

pub struct State<R, T, E> {
   pub index: usize,
   pub target: T,
   pub result: ParserResult<R, E>
}

// TODO: Clone or Copy or Ref?
impl<R, T, E> State<R, T, E> {
   pub fn new_err(self, err: E) -> Self {
      Self {
         index : self.index,
         target: self.target,
         result: Some(Err(err))
      }
   }
}