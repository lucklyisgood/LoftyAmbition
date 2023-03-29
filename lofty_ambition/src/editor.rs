use eframe::egui::{self, Response};

use crate::graph_ui::GraphUI;
use crate::menu_ui::ContextMenuUI;

pub struct Editor {
    pub graph_ui: GraphUI,
    pub context_menu_ui: ContextMenuUI,

    // allowed_to_close: bool,
    // show_confirmation_dialog: bool,
}

impl Editor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // allowed_to_close: false,
            // show_confirmation_dialog: false,
            graph_ui: GraphUI::default(),
            context_menu_ui: ContextMenuUI::default(),
        }
    }

    fn draw_bg(&mut self, ui: &mut eframe::egui::Ui) -> Response {
        egui::plot::Plot::new("node_grpah_yd")
            .show_x(false)
            .show_y(false)
            .allow_boxed_zoom(false)
            .data_aspect(1.0)
            .show(ui, |_plot_ui| {
                // plot_ui.
            }).response
    }

    fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        self.draw_bg(ui).context_menu(|ui| {
            self.context_menu_ui.draw(ui);
        });

        egui::Window::new("Seq")
            .auto_sized()
            // .min_width(200.0)
            // .min_height(200.0)
            .show(ui.ctx(), |ui| {
                egui::Resize::default().show(ui, |ui|{
                    ui.label("Floating text!");
                });
                // egui::Area::new("my_area")
                //     .movable(true)
                //     // .fixed_pos(egui::pos2(32.0, 32.0))
                //     .show(ui.ctx(), |ui| {
                //         ui.label("Floating text!");
                //     });
            });

        
    }

    // fn show_exit_confirm_dialog(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    //     egui::Window::new("Do you want to quit?")
    //         .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0_f32, 0_f32))
    //         .collapsible(false)
    //         .resizable(false)
    //         .show(ctx, |ui| {
    //             ui.horizontal(|ui| {
    //                 if ui.button("Yes!").clicked() {
    //                     self.allowed_to_close = true;
    //                     frame.close();
    //                 }
                    
    //                 if ui.button("Cancel").clicked() {
    //                     self.show_confirmation_dialog = false;
    //                 }
    //             });
    //         });
    // }
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