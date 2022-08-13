use std::rc::Rc;
use super::state::State;


pub trait Parse<R1,R2,T> {
   fn transform(&mut self, state: State<R1, T>) -> State<R2, T>;
   fn run(&mut self, target: T) -> State<R2, T> {
      let initial_state = State{target: Rc::new(target), index: 0, result: None };
      self.transform(initial_state)
   }
}



// pub trait Chain {
//    type R2;
//    type T;
//    type E1;
//    type E2;
//    type P : Parser;

//    fn chain<S, F>(&self, chain_fn: F) -> Self::P
//       where F: FnMut(ParserResult<Self::R2, Self::E2>) -> Box<dyn Parser<R1=Self::R2, R2=S, T=Self::T, E1 = Self::E1, E2 = Self::E2>>;
// }
