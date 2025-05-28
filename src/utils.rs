use std::io;

pub type Error = Box<dyn std::error::Error>;

pub async fn read_line() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input.trim().to_string()
}

#[derive(Debug, Clone, Copy)]
pub struct Term {
    pub coefficient: i64,
    pub index: i64,
}

impl Term {
    pub fn new(coefficient: i64, index: i64) -> Self {
        Self { coefficient, index }
    }

    // pub fn as_string(&self) -> String {
    //     format!("{}x^{}", self.coefficient, self.index)
    // }

    pub fn index_mult_coeff(&self) -> i64 {
        self.index * self.coefficient
    }
}

#[derive(Default, Debug)]
pub struct Expression {
    pub terms: Vec<Term>,
}

impl Expression {
    pub fn exp_from_str(expr: String) -> Result<Expression, Error> {
        let filtered: Vec<String> = expr
            .split_whitespace()
            .filter(|&token| token != "+" && token != "-" && token != "*" && token != "/")
            .map(|x| x.to_string())
            .collect();

        let mut degree = 0;
        let mut terms = Vec::new();

        for element in filtered {
            if element.contains("x") {
                let (_, index_str) = match element.split_once('^') {
                    Some(a) => a,
                    None => ("", "1"),
                };
                if index_str.contains(".") {
                    eprintln!("ERROR: FLOATING POINT AS INDEX ISN'T SUPPORTED ");
                    return Err(Box::new(InvalidExpression(format!(
                        "FLOATING POINT IN INDEX ISNT SUPPORTED"
                    ))));
                }
                let index = index_str.parse().expect("Error parsing expression");

                if index > degree {
                    degree = index
                }

                let (coefficient_str, _) = match element.split_once('x') {
                    Some(a) => a,
                    None => ("0", ""),
                };

                let coefficient = match coefficient_str.parse() {
                    Ok(a) => a,
                    Err(_) => 1,
                };

                terms.push(Term::new(coefficient, index));
            }
        }

        Ok(Expression { terms })
    }
}

#[derive(Debug)]
struct InvalidExpression(String);
impl std::error::Error for InvalidExpression {}
impl std::fmt::Display for InvalidExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Expression: {}", self.0)
    }
}
