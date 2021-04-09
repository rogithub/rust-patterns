/*
    Basically, for any kind of problems we define:

        * a domain specific language,
        * a grammar for this language,
        * an interpreter that solves the problem instances.


    Our goal is to translate simple mathematical expressions into postfix expressions
    (or Reverse Polish notation) For simplicity, our expressions consist of ten digits
    0, ..., 9 and two operations +, -.
    For example, the expression 2 + 4 is translated into 2 4 +.

    Our task is translate infix expressions into postfix ones. Let's define a context free grammar
    for a set of infix expressions over 0, ..., 9, +, and -, where:

    terminal symbols: 0, ..., 9, +, -
    non-terminal symbols: exp, term
    start symbol is exp
    and the following are production rules

    exp -> exp + term
    exp -> exp - term
    exp -> term
    term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
*/

pub struct Interpreter<'a> {
    it: std::str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{}'", op);
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected symbol '{}'", ch),
            None => panic!("Unexpected end of string"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interpreter_tests() {
        let mut intr = Interpreter::new("2+3");
        let mut postfix = String::new();
        intr.interpret(&mut postfix);
        assert_eq!(postfix, "23+");

        intr = Interpreter::new("1-2+3-4");
        postfix.clear();
        intr.interpret(&mut postfix);
        assert_eq!(postfix, "12-3+4-");
    }
}
