use std::collections::HashMap;
use core::iter::Peekable;
use std::str::Chars;


#[derive(Debug)]
#[derive(PartialEq)]
struct Terminal (String);

#[derive(Debug)]
#[derive(PartialEq)]
struct NonTerm (String);

#[derive(Debug)]
#[derive(PartialEq)]
enum TerminalOrNonTerminal {
    Term (Terminal),
    Non (NonTerm),
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Production {
    terms : Vec<TerminalOrNonTerminal>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct NonTerminal {
    name: String,
    productions : Vec<Production>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Interpreter {
    nonterminals: HashMap<String, NonTerminal>,
}


// MARK: Data Structures for parsing new Langauge

#[derive(Debug)]
enum LanguageProductionOrTerm {
    Term(Terminal),
    Prod(LanguageProduction),
}
#[derive(Debug)]
pub struct LanguageProduction {
    name : String,
    index: usize,
    productions : Vec<LanguageProductionOrTerm>
}

impl Interpreter {

    pub fn new() -> Interpreter {
        return Interpreter {
            nonterminals : HashMap::new(),
        };
    }

    fn parse_whitespace(&self, lines: &mut Peekable<Chars>) -> bool {

        fn is_whitespace(c: Option<&char>) -> bool {
            match c {
                Some(c) => *c == ' ' || *c == '\n' || *c == '\t',
                None => false
            }
        }

        while is_whitespace(lines.peek()) {
            lines.next();
        }

        return lines.peek() == None;
    }

    fn parse_identifier(&self, lines : &mut Peekable<Chars>) -> Result<String, String> {

        // return identifier
        let mut r = String::new();

        // Parse first character
        if let Some(c) = lines.peek() { 
            if !c.is_alphabetic() {
                return Err(String::from("Tried to parse identifier but found '"));
            }
            r.push(*c);
            lines.next();
        } else {
            return Err(String::from("Nothing to parse here"));
        }


        // Parse rest
        
        while let Some(c) = lines.peek() {
            if c.is_alphanumeric() {
                r.push(*c);
                lines.next();
            } else {
                break;
            }
        }

        return Ok(r);
    }

    fn parse_specific_id(&self, lines : &mut Peekable<Chars>, id : &String) 
        -> bool {
        
        let itr1 = lines;
        let mut itr2 = id.chars().peekable();

        loop {
            match (itr1.peek(), itr2.peek()) {
                (Some(c1), Some(c2)) => 
                    if c1 != c2 { return false; } else { itr1.next(); itr2.next(); },
                (_, None) => return true,
                _ => return false,
            }
        }

    }

    fn parse_char_in_string(&self, lines : &mut Peekable<Chars>) -> Result<char, String> {

        if let Some(c) = lines.next() {
            if c == '\\' {
                match lines.next() {
                    Some('\\') => return Ok('\\'),
                    Some('n') => return Ok('\n'),
                    Some('t') => return Ok('\t'),
                    Some(_) => return Err("Unknown character after \\ in string.".to_string()),
                    None => return Err("EOF reached parsing character after \\ in string.".to_string()),
                }
            } else {
                return Ok(c);
            }
        }
        else {
            return Err("EOF reached parsing char in string".to_string());
        }
    }


    fn parse_string(&self, lines : &mut Peekable<Chars>) -> Result<String, String> {

        // return identifier
        let mut r = String::new();

        if let Some(c) = lines.peek() {
            if *c != '\'' {
                return Err(String::from("First character was not '"));
            }
            lines.next();
        } else {
            return Err(String::from("Their was no characters."));
        }

        let mut result = self.parse_char_in_string(lines);
        while let Ok(c) = result {
            if c == '\'' {
                return Ok(r);
            } 
            r.push(c);
            result = self.parse_char_in_string(lines);
        }

        return match result {
            Ok(_) => Ok(r), // Should never happen
            Err(e) => Err(e),
        };
    }


    fn parse_term_nonterm(&self, lines : &mut Peekable<Chars>) -> 
        Result<TerminalOrNonTerminal, String> {

        match lines.peek() {
            Some(c) if *c == '\'' => {
                let result = self.parse_string(lines);
                if let Ok(s) = result {
                    Ok(TerminalOrNonTerminal::Term(Terminal(s)))
                } else if let Err(e) = result {
                    Err(e)
                } else {
                    Err("Should not happen".to_string())
                }
            },
            Some(c) if c.is_alphabetic() => {
                let result = self.parse_identifier(lines);
                if let Ok(c2) = result {
                    return Ok(TerminalOrNonTerminal::Non(NonTerm(c2)));
                } if let Err(e) = result {
                    return Err(e);
                } else {
                    return Err("Should not happen.".to_string());
                }
            },
            Some(_) => Err("Unknown character when parsing term or nonterminal.".to_string()),
            None => Err("Found EOF when parsing term or nonterminal.".to_string()),
        }
    }
    
    fn parse_production(&self, lines : &mut Peekable<Chars>) -> Result<Production, String> {

        let mut terms = Vec::new();

        loop {
            self.parse_whitespace(lines);
            match lines.peek() {
                Some(c) if *c == ';' || *c == '|' =>  {
                    return Ok(Production {terms: terms});
                },
                Some(_) => {
                    let result = self.parse_term_nonterm(lines);
                    if let Ok(term) = result {
                        terms.push(term);
                    } else if let Err(e) = result {
                        return Err(e);
                    }
                },
                None => {
                    return Err("Forgot a semicolon while parsing a production".to_string());
                }
            }
        }
    }

    fn parse_line(&self, lines : &mut Peekable<Chars>) -> Result<NonTerminal, String> {

        // Parse Id
        let result = self.parse_identifier(lines);
        let mut identifier = String::new();
        if let Ok(id) = result {
            identifier = id;
        } else if let Err(e) = result {
            return Err(e);
        }


        // Parse Whitespace If needed
        self.parse_whitespace(lines);


        // Parse ->
        if !self.parse_specific_id(lines, &"->".to_string()) {
            return Err("Expected -> while parsing line.".to_string());   
        }


        // Parse Productions
        let mut productions = Vec::new();

        loop {

            // Parse Whitespace If needed
            self.parse_whitespace(lines);

            match lines.peek() {
                Some(_) => {
                    let result = self.parse_production(lines);
                    if let Ok(production) = result {
                        productions.push(production);
                    } else if let Err(e) = result {
                        return Err(e);
                    }

                    if let Some(c) = lines.peek() {
                        if *c == '|' {
                            lines.next();
                        } else if *c == ';' {
                            lines.next();
                            return Ok(NonTerminal{ name: identifier, productions: productions});
                        }
                    }
                },
                None => return Err("Forgot a semicolon while parsing line.".to_string()),
            }
        }
    }
    


    pub fn add_interpreter(&mut self, lines : &String) -> Option<String> {

        let mut itr = lines.chars().peekable();
        self.parse_whitespace(&mut itr);
        
        while itr.peek() != None {

            let result = self.parse_line(&mut itr);

            match result {
                Ok(non_term) => { self.nonterminals.insert(non_term.name.clone(), non_term); },
                Err(e) => { return Some(e); },
            }

            self.parse_whitespace(&mut itr);
        }

        return None;
    }


    // MARK: Parsing new Language

    fn can_parse_using_production(&self, production : &Production, next_c : Option<char>)
        -> bool {

        if production.terms.is_empty() {
            return false;
        }

        let p = &production.terms[0];
        if let TerminalOrNonTerminal::Term(Terminal(s)) = p {
            match (s.chars().next(), next_c) {
                (Some(c1), Some(c2)) => return c1 == c2,
                (None, _) => return true,
                (_, _) => return false,
            }
        } else if let TerminalOrNonTerminal::Non(NonTerm(s)) = p {
            if let Some(non_term) = self.nonterminals.get(s) {
                return self.can_parse_using_nonterminal(&non_term, next_c);
            }
        }

        return true;
    }

    fn can_parse_using_nonterminal(&self, nonterminal : &NonTerminal, next_c : Option<char>)
        -> bool {

        for p in &nonterminal.productions {
            if self.can_parse_using_production(&p, next_c) {
                return true;
            }
        }

        return false;
    }

    fn parse_using_production(&self, production : &Production, lines : &mut Peekable<Chars>)
        -> Result<Vec<LanguageProductionOrTerm>, String> {

        let mut productions = Vec::new();
        
        for term in &production.terms {
            if let TerminalOrNonTerminal::Term(Terminal(t)) = term {
                let mut titr = t.chars().peekable();
                loop {
                    match (titr.peek(), lines.peek()) {
                        (Some(c1), Some(c2)) if *c1 == *c2 => {
                            titr.next(); lines.next();
                        },
                        (Some(_), _)  => {
                            let mut error_msg = 
                                String::from("Error trying to parse production. Trying to parse '");

                            error_msg.push_str(t.as_str());
                            error_msg.push_str("'.");

                            if let Some(c3) = lines.peek() {
                                error_msg.push_str(" Recieved charater '");
                                error_msg.push(*c3);
                                error_msg.push_str("'");
                            } else {
                                error_msg.push_str(" Reached EOF.");
                            }
                        
                            return Err(error_msg);
                        },
                        (None,  _) => break,
                    }
                }
                productions.push(LanguageProductionOrTerm::Term(Terminal(t.clone())));
            } else if let TerminalOrNonTerminal::Non(NonTerm(name)) = term {
                if let Some(nonterm) = self.nonterminals.get(name) {
                    let result = self.parse_using_nonterminal(nonterm, lines);
                    if let Ok(lp) = result {
                        productions.push(LanguageProductionOrTerm::Prod(lp));
                    } else if let Err(e) = result {
                        return Err(e);
                    }
                }
            }
        }

        return Ok(productions);
    }

    fn parse_using_nonterminal(&self, nonterminal : &NonTerminal, lines : &mut Peekable<Chars>)
        -> Result<LanguageProduction, String> {


        let c = if let Some(c1) = lines.peek() {
            Some(*c1)
        } else {
            None
        };

        for (index, p) in nonterminal.productions.iter().enumerate() {
            if self.can_parse_using_production(&p, c) {

                let result = self.parse_using_production(&p, lines);

                if let Ok(productions) = result {
                    return Ok(LanguageProduction { 
                        name: nonterminal.name.clone(), 
                        index: index,
                        productions: productions,
                    });
                } else if let Err(e) = result {
                    let mut error = e.clone();

                    error.push_str(" In nonterminal ");
                    error.push_str(nonterminal.name.as_str());
                    error.push_str(" parsing the ");
                    error.push_str(index.to_string().as_str());
                    error.push_str(" production.");

                    return Err(error);
                }
            }
        }

        println!("parsing {:#?}", nonterminal);
        //println!("{:#?}", lines);
        println!("{:#?}", lines.peek());

        let mut error_msg = String::from("Could not parse the production ");
        error_msg.push_str(nonterminal.name.as_str());
        return Err(error_msg);
    }

    pub fn parse(&self, starting_production : &String, lines : &String) -> Result<LanguageProduction, String> {

        let mut itr = lines.chars().peekable();

        if let Some(current_nonterm) = self.nonterminals.get(starting_production) {
            return self.parse_using_nonterminal(&current_nonterm, &mut itr);
        }
        
        return Err("Could not find production given in language.".to_string());
    }
}

