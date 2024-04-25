use symbol::{expr::Expression, simplify::simplify};

fn main() {
    let input = "(2+x)*3-2*(1+x)";
    // let input = "3*(2+x)";
    let expr = input.parse::<Expression>().unwrap();
    
    // dbg!(&expr);
    println!("expressions:");
    println!("- `{}` (original)", input);
    println!("- `{}` (parsed)", expr);
    println!();
    let simplified = simplify(&expr);
    println!("- `{}` (simplified)", simplified)
}
