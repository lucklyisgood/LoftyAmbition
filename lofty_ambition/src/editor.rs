use eframe::epaint::Stroke;
use eframe::{egui, epaint};

use crate::graph_ui::GraphUI;
use crate::menu_ui::ContextMenuUI;

pub struct Editor {
    pub graph_ui: GraphUI<u32>,
    pub context_menu_ui: ContextMenuUI,
}

impl Editor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            graph_ui: GraphUI::new(la_node_graph::Graph::new()),
            context_menu_ui: ContextMenuUI::default(),
        }
    }

    fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        // egui::ScrollArea::new([true, true]).id_source("node graph")
        // .show(ui, |ui| {
        //     egui::Grid::new("node_grid").striped(true)
        //         .show(ui, |_ui| {
                    
        //         }).response.context_menu(|ui| {
        //             self.context_menu_ui.draw(ui);
        //         });
        // });
        let plot_size = egui::Vec2::new(800.0, 600.0);
        let plot_background = epaint::RectShape {
            rect: egui::Rect::from_min_size(egui::Pos2::ZERO, plot_size),
            rounding: epaint::Rounding::same(0.0),
            fill: egui::Color32::from_rgba_premultiplied(255, 255, 0, 200).into(),
            stroke: Stroke::NONE,
        };
        ui.painter().add(plot_background);
    }
}


impl eframe::App for Editor {
    
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // egui::widgets::global_dark_light_mode_switch(ui);
                egui::menu::menu_button(ui, "File", |ui|{
                    if ui.button("Open").clicked() {
                        tracing::info!("Open File...");
                        ui.close_menu();
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw(ui);
        });
    }
}