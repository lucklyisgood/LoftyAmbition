use la_node_graph::Graph;

pub struct GraphUI<CustomDataType> {
    pub graph: Graph<CustomDataType>,
}

impl<CustomDataType> GraphUI<CustomDataType> {
    pub fn new(graph: Graph<CustomDataType>) -> Self {
        Self { graph }
    }

    #[allow(dead_code)]
    pub fn draw(&mut self, _ui: &mut eframe::egui::Ui) {

    }
}