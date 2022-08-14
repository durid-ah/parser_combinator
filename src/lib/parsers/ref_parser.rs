use std::{rc::Rc, cell::RefCell};
use crate::models::{parser_traits::Parse, state::State};




pub struct RefParser<R1,R2,T> {
   reference: Rc<RefCell<dyn Parse<R1,R2,T>>>
}

impl<R1,R2,T> RefParser<R1,R2,T> {
   pub fn new(reference: Rc<RefCell<dyn Parse<R1,R2,T>>>) -> Self {
      RefParser { reference }
   } 
}

impl<R1,R2,T> Parse<R1,R2,T> for RefParser<R1,R2,T> {
   fn transform(&self, state: State<R1, T>) -> State<R2, T> {
      self.reference.borrow_mut().transform(state)
   }

   fn run(&self, target: T) -> State<R2, T> {
      self.reference.borrow_mut().run(target)
   }
}