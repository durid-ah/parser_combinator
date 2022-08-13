use crate::models::parser_traits::Parse;



pub struct Between<R1,R2,T> {
   left: Box<dyn Parse<R1,R2,T>>,
   right: Box<dyn Parse<R1,R2,T>>,
   value: Box<dyn Parse<R1,R2,T>>
}

impl<R1,R2,T> Between<R1,R2,T>  {
   pub fn new(
      left: Box<dyn Parse<R1,R2,T>>,
      right: Box<dyn Parse<R1,R2,T>>,
      value: Box<dyn Parse<R1,R2,T>>) -> Self {
         Self { left, right, value }
   }
}