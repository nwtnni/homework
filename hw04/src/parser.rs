use std::io::{self, Write};
use std::str::FromStr;

use rpn::{self, Stack};

/// Start a read-eval-print loop, which runs until an error or `quit`.
pub fn read_eval_print_loop() -> rpn::Result<()> {
    // Create a stack to work on.
    loop {
        // Print a user input prompt.
        print!("> ");
        try!(io::stdout().flush().map_err(rpn::Error::IO));

        // TODO: Read from stdin into a String, and evaluate_line the result.
        // * An io::Error should be converted into a rpn::Error::IO
        let mut stack = Stack::new();
        let mut buffer = String::new();
        try!(io::stdin().read_line(&mut buffer).map_err(rpn::Error::IO));
        let result = evaluate_line(&mut stack, &buffer);

        match try!(result) {
            rpn::Elt::Int(i) => println!("{}", i),
            rpn::Elt::Bool(b) => println!("{}", b),
        }

        try!(io::stdout().flush().map_err(rpn::Error::IO));
    }
}

fn evaluate_line(stack: &mut Stack, buf: &str) -> rpn::Result<rpn::Elt> {
    // Create an iterator over the tokens.
    let tokens = buf.trim().split_whitespace();
    use rpn::{Elt, Op};

    for token in tokens {
        try!(match token {
            "true" => stack.push(Elt::Bool(true)),
            "false" => stack.push(rpn::Elt::Bool(false)),
            "+" => stack.eval(Op::Add),
            "~" => stack.eval(Op::Neg),
            "<->" => stack.eval(Op::Swap),
            "=" => stack.eval(Op::Eq),
            "#" => stack.eval(Op::Rand),
            "quit" => stack.eval(Op::Quit),
            s => {
                match i32::from_str(s) {
                    Ok(i) => {
                        stack.push(Elt::Int(i))
                    }
                    _ => Err(rpn::Error::Syntax),
                }
            },
        });
    }
    stack.result() 
}

#[cfg(test)]
mod tests {
    use rpn::{Stack, Error, Elt};
    use parser::evaluate_line;

    #[test]
    fn test_evaluate_line_bool() {
        let mut stack = Stack::new();
        let s = "true".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_int() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_plus() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "13".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "+".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(25));
    }

    #[test]
    fn test_evaluate_line_neg() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "~".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_evaluate_line_swap() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "<->".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
        assert_eq!(stack.pop().unwrap(), Elt::Int(15));
    }

    #[test]
    fn test_evaluate_line_eq() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "=".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_rand() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "#".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let res = stack.pop();
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res >= Elt::Int(0));
        assert!(res < Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_quit() {
        let mut stack = Stack::new();
        let s = "quit".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Quit) = res {
        } else { assert!(false); }
    }

    #[test]
    fn test_evaluate_line_bad_parse() {
        let mut stack = Stack::new();
        let s = "~false".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Syntax) = res {
        } else { assert!(false); }
    }
}
