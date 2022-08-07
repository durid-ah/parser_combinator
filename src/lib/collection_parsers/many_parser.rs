use crate::models::cardinality::Cardinality;
use crate::models::parser_traits::Parse;
use crate::models::state::State;
use std::rc::Rc;

pub struct Many<R1, R2, T> {
    parser: Box<dyn Parse<R1, R2, T>>,
}

impl<R1, R2, T> Many<R1, R2, T> {
    pub fn new(parser: Box<dyn Parse<R1, R2, T>>) -> Self {
        Self { parser }
    }
}

impl<R1, R2, T> Parse<R1, R2, T> for Many<R1, R2, T> {
    fn transform(&mut self, state: State<R1, T>) -> State<R2, T> {
        let mut results: Vec<R2> = Vec::new();
        let target = Rc::clone(&state.target);
        let mut final_state: State<R1, T> = State {
            index: state.index,
            target: Rc::clone(&state.target),
            result: state.result,
        };

        let mut done = false;
        while !done {
            let state = self.parser.transform(final_state);

            match state.result.unwrap() {
                Ok(Cardinality::One(res)) => results.push(res),
                Ok(Cardinality::Many(mut res)) => results.append(&mut res),
                Err(_) => done = true
            }

            final_state = State {
                index: state.index,
                target: Rc::clone(&target),
                result: None,
            }
        }

        return State {
            index: final_state.index,
            target,
            result: Some(Ok(Cardinality::Many(results))),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Many;
    use crate::models::parser_traits::Parse;
    use crate::parsers::str_parser::Str;

    #[test]
    fn many_parser_success() {
        let str_parser = Str::new("Test".to_owned());
        let mut many = Many::new(Box::new(str_parser));
        let result = many.run("TestTestTest");
        assert!(result.result.is_some());
        assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 3);
        assert_eq!(result.index, 12);
    }
}
