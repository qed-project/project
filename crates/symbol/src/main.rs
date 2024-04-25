use symbol::expr::Expression;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).format_timestamp(None).init();


    let input = "(2+x)*3-2*(1+x)";
    // let input = "(2+x)*3";
    let expr = input.parse::<Expression>().unwrap();
    
    // dbg!(&expr);
    println!("expressions:");
    println!("- `{}` (original)", input);
    println!("- `{}` (parsed)", expr);
    println!();
    println!("- `{}` (simplified)", expr.simplify())
}
