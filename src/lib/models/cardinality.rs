
pub enum Cardinality<R> {
   One(R),
   Many(Vec<R>)
}