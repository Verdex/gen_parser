
/*
    expect
    parse_symbol
    parse_number
    parse_string
    maybe
    zero_or_more
    one_or_more
    list
    choice
*/

#[derive(Debug)]
pub enum Parser {
    Expect(String),
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
