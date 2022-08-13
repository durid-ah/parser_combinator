use crate::models::parser_traits::Parse;
use crate::models::state::{ State, ParserResult };
use crate::parser_helpers::map_result;
use crate::models::cardinality::Cardinality;
use crate::collection_parsers::sequence_of_parser::SequenceOf;

pub struct Between<'a,R1,R2: Copy,T> {
   parser: Box<dyn Parse<R1,R2,T> + 'a>
}

impl<'a, R1,R2: Copy,T> Between<'a,R1,R2,T> 
   where R2: Copy + 'a, R1: 'a, T: 'a {
   pub fn new(
      left: Box<dyn Parse<R1,R2,T>>,
      right: Box<dyn Parse<R1,R2,T>>,
      value: Box<dyn Parse<R1,R2,T>>) -> Self {

         let parser =  SequenceOf::new(vec![left, value, right]);
         let parser = map_result::<'a>(parser, |opt: ParserResult<R2>| {
            opt.map(move |res| {
               res.map(move|card| { 
                  let res_vec = card.unwrap_many();
                  let slice = &res_vec[1..res_vec.len() - 1];
                  let mut result = Vec::new();
                  result.copy_from_slice(slice);
                  Cardinality::Many(result)
               })
            })
         });


         Self { parser }
   }
}

impl<'a,R1,R2: Copy,T> Parse<R1,R2,T> for Between<'a,R1,R2,T>  {
   fn transform(&mut self, state: State<R1, T>) -> State<R2, T> {
      return self.parser.transform(state);
   }

}