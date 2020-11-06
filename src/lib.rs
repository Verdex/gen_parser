
extern crate ir_data;
extern crate parse_input;

use std::collections::HashMap;

use ir_data::Data;
use parse_input::{Input, ParseError};

#[derive(Debug)]
pub enum Rule {
    Lookup(String),
    Sequence(Vec<Rule>),
    Expect(String),
    ParseSymbol,
    ParseNumber,
    ParseString,
    Maybe(Box<Rule>),
    ZeroOrMore(Box<Rule>),
    OneOrMore(Box<Rule>),
    List(Box<Rule>),
    Choice(Box<Rule>),
}

#[derive(Debug)]
pub enum GenParseError {
    SErr(ParseError),
    LookupError(String),
}

pub fn data_to_parser( data : Data ) -> Result<HashMap<String, Rule>, String> {
    Err("blah".to_string())
}

fn parse( rule : &Rule, parser : &HashMap<String, Rule>, input : Input ) -> Result<Data, GenParseError> {
    match rule {
        Rule::Lookup(name) => lookup(&name, parser, input),

        _ => panic!("TODO"),
    }
}

fn lookup( name : &str, parser : &HashMap<String, Rule>, input : Input ) -> Result<Data, GenParseError> {
    let rule = parser.get(name);

    if matches!(rule, None) {
        return Err(GenParseError::LookupError(format!("Failed to locate rule {}", name)));
    }

    Ok(Data::Cons { name: name.to_string(), params: vec! [parse( rule.unwrap(), parser, input )?] })
}

pub fn run_parser( parser : HashMap<String, Rule>, input : &str ) -> Result<Data, GenParseError> {
    let i = input.char_indices().collect::<Vec<_>>();
    let ii = Input::new( &i );
    
    lookup("main", &parser, ii)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
