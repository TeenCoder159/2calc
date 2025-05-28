#![allow(unused)]

use std::io;

fn read_line() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input.trim().to_string()
}

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Clone, Copy)]
struct Term {
    coefficient: i64,
    index: i64,
}

impl Term {
    pub fn new(coefficient: i64, index: i64) -> Self {
        Self { coefficient, index }
    }

    pub fn as_string(&self) -> String {
        format!("{}x^{}", self.coefficient, self.index)
    }

    pub fn index_mult_coeff(&self) -> i64 {
        self.index * self.coefficient
    }
}

#[derive(Default, Debug)]
struct Expression {
    terms: Vec<Term>,
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

    pub fn differentiate(&self) -> Option<String> {
        let mut answer: Vec<String> = Vec::new();

        for term in self.terms.clone() {
            answer.push(format!(
                "{}x^{}",
                term.index_mult_coeff(),
                term.coefficient - 1
            ));
        }

        Some(answer.join(" "))
    }
}

fn main() {
    // loop {
    //     println!("Enter your expression: ");

    //     let input = read_line();
    //     if input == "quit" {
    //         return;
    //     }
    // }
    let expr = Expression::exp_from_str(String::from("x^4 + 234x + 1"));
    println!("{:#?}", expr.unwrap().differentiate().unwrap())
}
