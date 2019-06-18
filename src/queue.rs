extern crate redis;

pub struct Queue {
    name: String,
    client: redis::Connection,
}

impl Queue {
    pub fn new(name: String, client: redis::Connection) -> Queue {
        Queue {
            name: name,
            client: client,
        }
    }

    pub fn queue(&self) -> &str {
        &self.name
    }
}
