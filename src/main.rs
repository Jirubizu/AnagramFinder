mod finder;
use finder::anagram::Solver;

#[tokio::main]
async fn main() {
    let solver:Solver = Solver::new().await;
    solver.display_anagram("car".to_string());
    solver.display_anagram("dog".to_string());
    //solver.display();
    println!("Hello, world!");
}
