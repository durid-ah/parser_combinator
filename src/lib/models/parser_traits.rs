use std::{rc::Rc, fmt::Debug};
use super::state::State;


/// # Parse
/// Trait used by the parsers to ensure they interoperate with each other
pub trait Parse<R1,R2,T>: Debug {
   fn transform(&self, state: State<R1, T>) -> State<R2, T>;
   fn run(&self, target: T) -> State<R2, T> {
      let initial_state = State{target: Rc::new(target), index: 0, result: None };
      self.transform(initial_state)
   }
}
