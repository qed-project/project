use symbol::{expr::Expression, simplify::simplify};

fn main() {
    let input = "(2+x)*3";
    let expr = input.parse::<Expression>().unwrap();
    let simplified = simplify(&expr);
    
    dbg!(&expr);
    println!("expressions:");
    println!("- `{}` (original)", input);
    println!("- `{}` (parsed)", expr);
    println!("- `{}` (simplified)", simplified)
}
