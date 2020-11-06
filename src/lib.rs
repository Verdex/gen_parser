

#[derive(Debug)]
pub enum Parser {
    Def { name: String, def: Box<Parser> },
    Invoke(String),
    Expect(String),
    ParseSymbol,
    ParseNumber,
    ParseString,
    Maybe(Box<Parser>),
    ZeroOrMore(Box<Parser>),
    OneOrMore(Box<Parser>),
    List(Box<Parser>),
    Choice(Box<Parser>),
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
