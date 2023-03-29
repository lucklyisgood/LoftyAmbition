
use serde::{Deserialize, Serialize};

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutpuId;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node<CustomDataType> {
    pub id: NodeId,
    pub title: String,
    pub inputs: Vec<(String, InputId)>,
    pub outputs: Vec<(String, OutpuId)>,
    pub custom_data: CustomDataType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph<CustomNodeDataType> {
    pub nodes: slotmap::SlotMap<NodeId, Node<CustomNodeDataType>>,
}