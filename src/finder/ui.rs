use eframe::egui;

pub struct AnagramUi {
    search_term: String,
    anagram_solver: super::anagram::Solver,
    anagrams: String,
}

impl AnagramUi {
    pub fn new(cc: &eframe::CreationContext<'_>,solver: super::anagram::Solver) -> Self {
        Self {
            search_term: "".to_owned(),
            anagram_solver: solver,
            anagrams: "".to_owned(),
        }
    }
}

impl eframe::App for AnagramUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|hui| {
               if hui.text_edit_singleline(&mut self.search_term).changed() {
                   self.anagrams = self.anagram_solver.get_anagrams(self.search_term.clone());
               }
            });
            for ana in self.anagrams.split(", ") {
               ui.label(ana);
            }
            // ui.label(format!("Anagrams: {}", self.anagrams));
        }); 
    }
}
