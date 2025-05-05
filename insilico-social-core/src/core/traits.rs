pub trait Agent {
    fn get_id(&self) -> String;
    fn step(&mut self) {
        // Default implementation does nothing
    }
    fn serialize(&self) -> JsValue;
    fn create_agent(&self, agent_type: &str) -> JsValue;
}

pub trait Model {
    fn init(&mut self);
    fn step(&mut self);
    fn serialize(&self) -> JsValue;
}