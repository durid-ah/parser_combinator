use std::{rc::Rc, cell::RefCell};

use crate::models::{parser_traits::Parse, state::State};




pub struct RefParser<R1,R2,T,E1,E2> {
   reference: Rc<RefCell<dyn Parse<R1,R2,T,E1,E2>>>
}

impl<R1,R2,T,E1,E2> RefParser<R1,R2,T,E1,E2> {
   pub fn new(reference: Rc<RefCell<dyn Parse<R1,R2,T,E1,E2>>>) -> Self {
      RefParser { reference }
   } 
}

impl<R1,R2,T,E1,E2> Parse<R1,R2,T,E1,E2> for RefParser<R1,R2,T,E1,E2> {
   fn transform(&mut self, state: State<R1, T, E1>) -> State<R2, T, E2> {
      self.reference.borrow_mut().transform(state)
   }

   fn run(&mut self, target: T) -> State<R2, T, E2> {
      self.reference.borrow_mut().run(target)
   }
}