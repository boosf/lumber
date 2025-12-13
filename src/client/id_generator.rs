pub trait IDGenerator {
    fn get_id(&mut self) -> i64;
}

pub struct IDGeneratorClient {
    counter: i64,
}

impl IDGeneratorClient {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl IDGenerator for IDGeneratorClient {
    fn get_id(&mut self) -> i64 {
        let id = self.counter;
        self.counter += 1;
        id
    }
}
