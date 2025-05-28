mod diff;
mod utils;

use diff::Differentiatable;
use utils::{Error, Expression, read_line};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Enter your expression to differentiate:");
    let expr = Expression::exp_from_str(read_line().await);
    // 4x^3 + 3x^2 + 5x + 2 -> Test input
    let differentiated = expr?.differentiate().await.unwrap();
    println!("{}", differentiated);
    Ok(())
}
