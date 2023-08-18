pub struct Parser {
    input: Vec<u8>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            input: input.into_bytes(),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        Ok(())
    }
}
