use super::token::Token;
use std::iter::Peekable;
use std::str::Chars;

// peekable lets us peek at the next char
#[derive(Debug)]
pub struct Tokenizer<'a> {
    // expr is a field of chars that have optional refrence to next ele
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                // stringify next_char(int) into the variable number(string)
                let mut number = next_char?.to_string();

                // while self.expr.peek() == Some(char) pass that char into
                while let Some(extracted_next_char) = self.expr.peek() {
                    // check if extracted char is a numeric or a decimal
                    if extracted_next_char.is_numeric() || extracted_next_char == &'.' {
                        // push the extracted char to number
                        number.push(self.expr.next()?);
                    // if & or ( return a None
                    } else if extracted_next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }

                // returns our completed number variable parsed into a f64
                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("42");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(42.0))
    }

    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("42.42");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(42.42))
    }

    #[test]
    #[ignore]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(27.0));
    }
}
