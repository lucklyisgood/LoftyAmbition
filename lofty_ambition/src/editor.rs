use eframe::{egui, epaint};

use crate::graph_ui::GraphUI;
use crate::menu_ui::ContextMenuUI;

pub struct Editor {
    pub graph_ui: GraphUI<u32>,
    pub context_menu_ui: ContextMenuUI,
    pub box_select_start_pos: Option<egui::Pos2>,
    pub pan: egui::Vec2,
    pub zoom: f32,
}

impl Editor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            graph_ui: GraphUI::new(la_node_graph::Graph::new()),
            context_menu_ui: ContextMenuUI::default(),
            box_select_start_pos: None,

            pan: egui::Vec2::ZERO,
            zoom: 1.0,
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
        
        let view_rect = ui.max_rect();
        // ignore hover event in rect;
        ui.allocate_rect(view_rect, egui::Sense::hover());

        let pointer_pos = ui.input(|i|i.pointer.hover_pos().unwrap_or(egui::Pos2::ZERO));
        let pointer_in_view = view_rect.contains(pointer_pos);
        let mut drag_start_in_bg = false;
        let mut drag_end_in_bg = false;
        let mut click_in_bg = false;

        // Draw nodes
        self.graph_ui.draw(ui, &self.pan, self.zoom);

        let r = ui.allocate_rect(ui.min_rect(), egui::Sense::click().union(egui::Sense::drag()));
        if r.clicked() {
            click_in_bg = true;
        } else if r.drag_started() {
            drag_start_in_bg = true;
        } else if r.drag_released() {
            drag_end_in_bg = true;
        }


        // handle box select
        if let Some(select_start_pos) = self.box_select_start_pos {
            let select_rect = egui::Rect::from_two_pos(select_start_pos, pointer_pos);
            let fill_color = egui::Color32::from_rgba_premultiplied(200, 200, 200, 20);
            let stroke_color = egui::Color32::from_rgba_unmultiplied(200, 200, 200, 180);
            ui.painter().rect(select_rect, 2.0, fill_color, epaint::Stroke::new(3.0, stroke_color));

            // self.selected_nodes = node_rects.into_iter().filter_map(|(node_id, rect)| { if selection_rect.intersects(rect) { Some(node_id) } else { None } }).collect();
        }

        let is_pirmary_down = ui.input(|i| i.pointer.primary_down());
        let is_pirmary_release = ui.input(|i| i.pointer.primary_released());

        if drag_start_in_bg && is_pirmary_down {
            self.box_select_start_pos = Some(pointer_pos);
        }
        if drag_end_in_bg || is_pirmary_release {
            self.box_select_start_pos = None;
        }

        // show context menu
        self.show_context_menu(ui, r, &pointer_pos, &view_rect);
        
    }

    fn show_context_menu(&mut self, ui: &mut egui::Ui, r: egui::Response, pointer_pos: &egui::Pos2, view_rect: &egui::Rect) -> egui::Response {
        r.context_menu(|ui| {
            ui.menu_button("readbook", |ui| {
                if ui.button("spine").clicked() {
                    tracing::info!("create node readbook/spine");
                    let pos = egui::pos2(pointer_pos.x, pointer_pos.y) - self.pan - view_rect.min.to_vec2();
                    self.graph_ui.add_node(10086, pos);
                    ui.close_menu();
                }
            });

            ui.menu_button("msic", |ui| {
                if ui.button("spine").clicked() {
                    tracing::info!("create node msic/spine");
                    ui.close_menu();
                }
            });

            if ui.button("test").clicked() {
                tracing::info!("create node test");
                ui.close_menu();
            }
        })
    }
}

#[allow(dead_code)]
fn draw_connection(painter: &egui::Painter, src_pos: egui::Pos2, dst_pos: egui::Pos2, color: egui::Color32) {
    let connection_stroke = egui::Stroke { width: 5.0, color };

    let control_scale = ((dst_pos.x - src_pos.x) / 2.0).max(30.0);
    let src_control = src_pos + egui::Vec2::X * control_scale;
    let dst_control = dst_pos - egui::Vec2::X * control_scale;

    let bezier = epaint::CubicBezierShape::from_points_stroke(
        [src_pos, src_control, dst_control, dst_pos],
        false,
        egui::Color32::TRANSPARENT,
        connection_stroke,
    );

    painter.add(bezier);
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