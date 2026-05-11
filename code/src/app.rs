pub struct App{
    label: String,
}

impl Default for App{
    fn default() -> Self{
        Self {
            label: "This is BCS: Brewery Control System".to_owned()
        }
    }
}

impl App{
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self{
        Default::default()
    }
    
}

impl eframe::App for App{
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame){

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::CentralPanel::default().show_inside(ui, |ui|{
            ui.heading("BCS");
            ui.horizontal(|ui|{
                ui.label("write something: ");
                ui.text_edit_singleline(&mut self.label);
            });
        });
    }
}
