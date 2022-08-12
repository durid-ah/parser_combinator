

/// # Cardinality
/// `Cardinality` represents either ([`Cardinality::One`]) or ([`Cardinality::Many`])
/// values of the type `R`
#[derive(Debug, Clone)]
pub enum Cardinality<R> {
   // Represents a single value of type `R`
   One(R),
   // Represents multiple values of type `R`
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