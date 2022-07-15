use regex::Regex;

use crate::models::parser_traits::Parse;


pub struct Letters {
   
}

impl Letters {
   pub fn something() {
      let matcher = Regex::new(r"^[A-Za-z]+").unwrap();
      // matcher.find_at(text, start) //find(text)
   }
}

impl Parse<String,String,String,String,String> for Letters {
    fn transform(&mut self, state: crate::models::state::State<String, String, String>) -> crate::models::state::State<String, String, String> {
        todo!()
    }

    fn run(&mut self, target: String) -> crate::models::state::State<String, String, String> {
        todo!()
    }
}
