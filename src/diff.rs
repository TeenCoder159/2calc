use crate::utils::Expression;

pub trait Differentiatable {
    async fn differentiate(&self) -> Option<String>;
}

impl Differentiatable for Expression {
    async fn differentiate(&self) -> Option<String> {
        let mut answer: Vec<String> = Vec::new();

        for term in self.terms.clone() {
            let mut index: &str = &format!("x^{}", term.index);

            if term.index_mult_coeff() != 0 {
                index = match term.index - 1 {
                    0 => "",
                    1 => "x",
                    _ => index,
                };
            }

            let mut new_coeff = format!("{}", term.index_mult_coeff());

            if term.index_mult_coeff() == 0 {
                new_coeff = "".to_string();
            }

            answer.push(format!("{}{}", new_coeff, index));
        }

        Some(answer.join(" + "))
    }
}
