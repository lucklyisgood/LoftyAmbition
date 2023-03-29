
pub struct ContextMenuUI {}

impl Default for ContextMenuUI {
    fn default() -> Self {
        ContextMenuUI { }
    }
}

impl ContextMenuUI {
    pub fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        ui.menu_button("test node", |ui| {
            if ui.button("Spine").clicked() {
                tracing::info!("Click spine context menu");
                ui.close_menu();
            }
        });
    }
}