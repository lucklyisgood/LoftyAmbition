use eframe::egui::{self, Response};

use crate::graph_ui::GraphUI;
use crate::menu_ui::ContextMenuUI;

pub struct Editor {
    pub graph_ui: GraphUI<u32>,
    pub context_menu_ui: ContextMenuUI,

    // allowed_to_close: bool,
    // show_confirmation_dialog: bool,
}

impl Editor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // allowed_to_close: false,
            // show_confirmation_dialog: false,
            graph_ui: GraphUI::new(la_node_graph::Graph::new()),
            context_menu_ui: ContextMenuUI::default(),
        }
    }

    fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        egui::ScrollArea::new([true, true]).id_source("node graph")
        .show(ui, |ui| {
            egui::Grid::new("node_grid").striped(true)
                .show(ui, |ui| {
                    
                }).response.context_menu(|ui| {
                    self.context_menu_ui.draw(ui);
                });
        });
    }
}


impl eframe::App for Editor {

    // fn on_close_event(&mut self) -> bool {
    //     tracing::info!("on close evnet");
    //     self.show_confirmation_dialog = true;
    //     self.allowed_to_close
    // }
    
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw(ui);
        });
        // if self.show_confirmation_dialog {
        //     self.show_exit_confirm_dialog(ctx, frame);
        // }
    }
}