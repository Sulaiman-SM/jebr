#[derive(Debug)]
pub struct Expression<'a> {
    pub definition : Definition<'a>,
    pub statement : Statement<'a>
}

#[derive(Debug)]
pub struct Statement<'a> {
    statement_string : &'a str,
    value : &'a str
}

#[derive(Debug)]
pub struct Definition<'a> {
    var_name : &'a str,
    var_type : &'a str,
    var_value : &'a str
}

impl Expression<'_> {
    pub fn new(def: Definition<'static>, stmt: Statement<'static>) -> Expression<'static> {
        Expression { definition: def, statement: stmt }
    }
}

impl Definition<'_> {
    pub fn new(name: &'static str, var_type: &'static str, var_value: &'static str) -> Definition<'static> {
        Definition { var_name: name, var_type: var_type, var_value: var_value }
    }
}

impl Statement<'_> {
    pub fn new(name: &'static str, var_type: &'static str) -> Statement<'static> {
        Statement { statement_string: name, value: var_type }
    }
}