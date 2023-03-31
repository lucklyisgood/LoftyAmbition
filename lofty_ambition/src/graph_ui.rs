use la_node_graph::{ Graph, NodeId};
use eframe::{egui, epaint};

pub struct GraphUI<CustomDataType> {
    pub graph: Graph<CustomDataType>,
    pub node_ui_data_list: slotmap::SecondaryMap<NodeId, egui::Pos2>,
    pub node_order_list: Vec<NodeId>,
}

impl<CustomDataType> GraphUI<CustomDataType> {
    pub fn new(graph: Graph<CustomDataType>) -> Self {
        Self {
            graph,
            node_ui_data_list: slotmap::SecondaryMap::default(),
            node_order_list: Vec::default(),
        }
    }

    fn draw_node(&self, ui: &mut egui::Ui, pos: &egui::Pos2) {
        let base_shape = epaint::RectShape {
            rect: egui::Rect::from_min_size(egui::pos2(pos.x, pos.y), egui::vec2(200.0, 200.0)),
            rounding: epaint::Rounding::same(3.0),
            fill: egui::Color32::from_rgb(60, 60, 60),
            stroke: epaint::Stroke { width: 3.0, color:  egui::Color32::from_rgb(255, 0, 0) },
        };
        ui.painter().add(base_shape);
    }

    #[allow(dead_code)]
    pub fn draw(&mut self, ui: &mut egui::Ui, _pan: &egui::Vec2, _zoom: f32) {
        for id in self.node_order_list.iter().copied() {
            if let Some(item_pos) = self.node_ui_data_list.get(id) {
                self.draw_node(ui, item_pos);
            }
        }
    }

    pub fn add_node(&mut self, custom_data: CustomDataType, pos: egui::Pos2) {
        let id = self.graph.add_node("Node".into(), custom_data, |_graph, _id| {});
        self.node_ui_data_list.insert(id, pos);
        self.node_order_list.push(id);
    }
}