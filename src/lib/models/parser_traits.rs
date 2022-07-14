use super::state::State;

pub trait Parse<R1,R2,T,E1,E2> {
   fn transform(&mut self, state: State<R1, T, E1>) -> State<R2, T, E2>;
   fn run(&mut self, target: T) -> State<R2, T, E2>;
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


#[cfg(test)]
mod tests {
}