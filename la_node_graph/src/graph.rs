use super::*;

impl<CustomNodeDataType> Graph<CustomNodeDataType> {
    pub fn new() -> Self {
        Self {
            nodes: slotmap::SlotMap::default(),
        }
    }

    pub fn add_node(&mut self, title: String, custom_data: CustomNodeDataType, f: impl FnOnce(&mut Graph<CustomNodeDataType>, NodeId)) -> NodeId {
        let key = self.nodes.insert_with_key(|id| {
            Node {
                id,
                title,
                inputs: Vec::default(),
                outputs: Vec::default(),
                custom_data,
            }
        });

        f(self, key);
        key
    }
}