extern crate regex;

use regex::Regex;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {

    pub fn new(s: &str) -> Lexer {
        Lexer {
            source: s.chars().collect(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        try!(self.accept_int());
        try!(self.accept_end());
        println!("Success!");
        Ok(())
    }

    pub fn accept_int(&mut self) -> Result<(), String> {
        self.accept_negative_sign(); // Optional, doesn't matter if Ok or Err
        if self.is('0').is_ok() {
            Ok(())
        } else {
            while self.accept_digit().is_ok() { }
            Ok(())
        }
    }

    pub fn accept_digit(&mut self) -> Result<(), String> {
        let re = Regex::new(r"[0-9]").unwrap();

        if self.position < self.source.len() {
            if re.is_match(&mut self.source[self.position].to_string()) {
                self.bump();
                return Ok(());
            }
        } else {
            return Err(format!("Expected digit, found EOF"));
        }

        Err(format!("Expected digit, found: {}", self.source[self.position]))
    }

    pub fn accept_negative_sign(&mut self) -> Result<(), String> {
        self.is('-')
    }

    pub fn accept_end(&mut self) -> Result<(), String> {
        self.is(';')
    }

    pub fn is(&mut self, c: char) -> Result<(), String> {

        if self.position < self.source.len() {
            if self.source[self.position] == c {
                self.bump();
                return Ok(());
            }
        } else {
            return Err(format!("Expected {}, found EOF", c));
        }

        Err(format!("Expected {}, found: {}", c, self.source[self.position]))
    }

    fn bump(&mut self) {
        self.position = self.position + 1;
    }
}

#[test]
fn test() {
    assert!(Lexer::new("1;").parse().is_ok());
    assert!(Lexer::new("0;").parse().is_ok());
    assert!(Lexer::new("159;").parse().is_ok());
    assert!(Lexer::new("-124;").parse().is_ok());
    assert!(Lexer::new("-0;").parse().is_ok());
    assert!(Lexer::new("123456789;").parse().is_ok());
    assert!(Lexer::new("01;").parse().is_err());
    assert!(Lexer::new("-01;").parse().is_err());
    assert!(Lexer::new("abc;").parse().is_err());
    assert!(Lexer::new("-zyx;").parse().is_err());
    assert!(Lexer::new("1").parse().is_err());
    assert!(Lexer::new("0").parse().is_err());
    assert!(Lexer::new("-1").parse().is_err());
    assert!(Lexer::new("123456789").parse().is_err());
}
