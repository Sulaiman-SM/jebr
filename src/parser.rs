use regex::Regex;

extern crate evalexpr;
use evalexpr::eval;

struct ParserRule {
    name: String,
    regex: Regex,
}

pub struct Parser {
    rules: Vec<ParserRule>
}

impl Parser {
    pub fn new() -> Parser {
        let mut p_rules = vec![];

        let rules = vec![
        ("DEFENITION", r"^(عرف) ([\u0621-\u064A]+)\s*$"), 
        ("STATEMENT", r"(عرف) ([\u0621-\u064A]+)(\s*=\s*)\d")
        ];

        for rule in rules {
            let reg = Regex::new(rule.1).unwrap();
            let parse_rule = ParserRule {name: rule.0.to_string(), regex: reg};
            p_rules.push(parse_rule)
        }

        return Parser{rules: p_rules}
    }

    pub fn parse<'a>(&'a self, s: String) {
        for line in s.lines() {
            // let l = 
            self.parse_statement(line.to_string());
        }
    }

    pub fn parse_statement<'a>(&'a self, line: String) {
        for rule in self.rules.iter() {
            if rule.regex.is_match(&line) {
                if rule.name == "STATEMENT" {
                    // Regex::new(r"^([-+/*]\d+(\.\d+)?)*");
                    let exp = line.split("=");
                    let vec: Vec<&str> = exp.collect();
                    let evaluated = std::string::ToString::to_string(vec[1]);
                    println!("line {} is {} with value {}", line, rule.name, eval(&evaluated).unwrap() )
                }
                
            }
        } 
    }
}