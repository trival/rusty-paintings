pub struct Form {
    pub vertex_count: u32,
}

impl Form {
    pub fn new(vertex_count: u32) -> Self {
        Self { vertex_count }
    }
}
