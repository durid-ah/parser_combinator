use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
   pub static ref LOCAL_LOG: RwLock<LocalLog> = RwLock::new(LocalLog { indent: 0 }); 
}
// LocalLog{ indent: AtomicUsize::new(0) };

#[derive(Default)]
pub struct LocalLog {
   indent: usize
}

pub fn start_scope() {
   LOCAL_LOG.write().expect("Failed Start Scope").start_scope();
}

pub fn end_scope() {
   LOCAL_LOG.write().expect("Failed End Scope").close_scope();
}

pub fn log(message: String) {
   let i = LOCAL_LOG.read().expect("Failed Log").indent;
   println!("{:indent$}{}","", message, indent=i*3);
}

impl LocalLog {
   pub fn start_scope(&mut self) {
      self.indent += 1;
   }

   pub fn close_scope(&mut self) {
      self.indent -= 1;
   }
}
