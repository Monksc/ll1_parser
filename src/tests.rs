
#[cfg(test)]
mod tests {
    use core::iter::Peekable;
    use std::str::Chars;
    use crate::*;

    fn parse(s : String, update_itr : impl Fn(&mut Peekable<Chars>)) -> String {
        
        let mut itr = s.chars().peekable();

        update_itr(&mut itr);

        let r : String  = itr.collect();
        return r;
    }
    fn test_parse(astr : String, b : String, 
            update_itr : impl Fn(&mut Peekable<Chars>)) {

        let a = parse(astr, update_itr);
        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }
    fn test_parse_ampstr(a : &str, b : &str, 
            update_itr : impl Fn(&mut Peekable<Chars>)) {

        test_parse(String::from(a), String::from(b), update_itr);
    }


    // Parse White Space

    #[test]
    fn all_test_parse_whitespace() {

        fn f(itr : &mut Peekable<Chars>) {
            let interp = Interpreter::new();
            interp.parse_whitespace(itr);
        }

        test_parse_ampstr(" Hello World", "Hello World", f);
        test_parse_ampstr("Hello World", "Hello World", f);
        test_parse_ampstr("   \t  \n  ", "", f);
    }



    // Parse Identifier


    #[test]
    fn all_test_parse_identifier() {

        fn make_f(result : Result<String, String>) -> impl Fn(&mut Peekable<Chars>) {
            move |itr : &mut Peekable<Chars>| {
                let interp = Interpreter::new();
                let r = interp.parse_identifier(itr);
                
                assert_eq!(result, r, 
                    "Result of parse identifier is not right. Expected {:#?} Recieved {:#?}.", 
                    result, r);
            }
        }


        test_parse_ampstr("helloWorld&here", "&here", 
            make_f(Ok(String::from("helloWorld"))));

        test_parse_ampstr("4here", "4here", 
            make_f(Err(String::from("Tried to parse identifier but found '"))));

        test_parse_ampstr("test4EndOfFile", "", 
            make_f(Ok(String::from("test4EndOfFile"))));

        test_parse_ampstr(" test4StartingSpace", " test4StartingSpace", 
            make_f(Err(String::from("Tried to parse identifier but found '"))));

        test_parse_ampstr("", "", 
            make_f(Err(String::from("Nothing to parse here"))));
    }


    // Parse Specific Id

    #[test]
    fn all_test_parse_id() {

        fn make_f(result : bool, key: &str) -> impl Fn(&mut Peekable<Chars>) {

            let key_str = String::from(key);

            move |itr : &mut Peekable<Chars>| {
                let interp = Interpreter::new();
                let r = interp.parse_specific_id(itr, &key_str);
                
                assert_eq!(result, r, 
                    "Result of parse id is not right. Expected {:#?} Recieved {:#?}.", 
                    result, r);
            }
        }


        test_parse_ampstr("-> HELLO", " HELLO", 
            make_f(true, "->"));

        test_parse_ampstr(" -> HELLO", " -> HELLO", 
            make_f(false, "->"));

        test_parse_ampstr("  aj ", "", 
            make_f(true, "  aj "));

        test_parse_ampstr("-", "", 
            make_f(false, "->"));
    }


    // Parse String

    #[test]
    fn all_test_parse_string() {

        fn make_f(result : Result<String, String>) -> impl Fn(&mut Peekable<Chars>) {
            move |itr : &mut Peekable<Chars>| {
                let interp = Interpreter::new();
                let r = interp.parse_string(itr);
                
                assert_eq!(result, r, 
                    "Result of parse id is not right. Expected {:#?} Recieved {:#?}.", 
                    result, r);
            }
        }


        test_parse_ampstr("'chad'", "", 
            make_f(Ok("chad".to_string())));

        test_parse_ampstr("'\\\\ \\t \\n' after", " after", 
            make_f(Ok("\\ \t \n".to_string())));

        test_parse_ampstr(" 's'", " 's'", 
            make_f(Err("First character was not '".to_string())));

        test_parse_ampstr("", "", 
            make_f(Err("Their was no characters.".to_string())));

        test_parse_ampstr("'EOF", "", 
            make_f(Err("EOF reached parsing char in string".to_string())));

    }


    // Parse Term or Nonterminal

    #[test]
    fn all_test_parse_term_nonterminal() {

        fn make_f(result : Result<TerminalOrNonTerminal, String>) -> 
            impl Fn(&mut Peekable<Chars>) {

            move |itr : &mut Peekable<Chars>| {
                let interp = Interpreter::new();
                let r = interp.parse_term_nonterm(itr);
                
                assert_eq!(result, r, 
                    "Result of parse id is not right. Expected {:#?} Recieved {:#?}.", 
                    result, r);
            }
        }


        test_parse_ampstr("hello4World chad martin", " chad martin", 
            make_f(Ok(TerminalOrNonTerminal::Non(NonTerm("hello4World".to_string())))));

        test_parse_ampstr("'hello world' not here", " not here", 
            make_f(Ok(TerminalOrNonTerminal::Term(Terminal("hello world".to_string())))));

    }


    // Parse Production

    #[test]
    fn all_test_parse_production() {

        fn make_f(result : Result<Production, String>) -> 
            impl Fn(&mut Peekable<Chars>) {

            move |itr : &mut Peekable<Chars>| {
                let interp = Interpreter::new();
                let r = interp.parse_production(itr);
                
                assert_eq!(result, r, 
                    "Result of parse id is not right. Expected {:#?} Recieved {:#?}.", 
                    result, r);
            }
        }


        let p1 = Production {
            terms: vec![
                TerminalOrNonTerminal::Non(NonTerm("hello".to_string())),
                TerminalOrNonTerminal::Term(Terminal("->".to_string())),
                TerminalOrNonTerminal::Non(NonTerm("chad".to_string())),
            ],
        };
        test_parse_ampstr(" \t\nhello \t\n'->' chad |", "|", 
            make_f(Ok(p1)));

        let p2 = Production {
            terms: vec![
                TerminalOrNonTerminal::Non(NonTerm("hello".to_string())),
                TerminalOrNonTerminal::Term(Terminal("->".to_string())),
                TerminalOrNonTerminal::Non(NonTerm("chad".to_string())),
            ],
        };
        test_parse_ampstr(" \t\nhello \t\n'->' chad ;", ";", 
            make_f(Ok(p2)));

        test_parse_ampstr("hello world", "", 
            make_f(Err("Forgot a semicolon while parsing a production".to_string())));

    }

}


