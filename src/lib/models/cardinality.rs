
#[derive(Debug)]
pub enum Cardinality<R> {
   One(R),
   Many(Vec<R>)
}

impl<R> Cardinality<R> {
   pub fn unwrap_one(self) -> R {
      match self {
         Cardinality::One(item) => item,
         Cardinality::Many(_) => panic!("Cardinality::Attempted to unwrap one instead got many")
      }
   }

   pub fn unwrap_many(self) -> Vec<R> {
      match self {
         Cardinality::One(_) => panic!("Cardinality::Attempted to unwrap many instead got one"),
         Cardinality::Many(items) => items
      }
   }
}