mod finder;
use eframe::epaint::Vec2;
use finder::anagram::Solver;
use finder::ui::AnagramUi;

#[tokio::main]
async fn main() {
    let solver:Solver = Solver::new().await;

    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some(Vec2::new(200.0, 300.0));
    eframe::run_native("Anagram Finder", options, Box::new(|cc| Box::new(AnagramUi::new(cc,solver))));
}
