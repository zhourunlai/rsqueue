use redis::{Commands, ErrorKind, RedisResult, Value};

use crate::job::{JobDecodable, JobEncodable};

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

    pub fn publish<T: JobEncodable>(&self, job: T) -> RedisResult<()> {
        self.client.rpush(self.queue(), job.encode())
    }

    pub fn subscribe<T: JobDecodable>(&self) -> Option<RedisResult<T>> {
        let v = match self.client.lpop(self.queue()) {
            Ok(v) => match v {
                v @ Value::Data(_) => v,
                _ => {
                    return Some(Err(From::from((ErrorKind::TypeError, ""))));
                }
            },
            Err(_) => {
                return Some(Err(From::from((ErrorKind::TypeError, ""))));
            }
        };

        match T::decode(&v) {
            Ok(job) => Some(Ok(job)),
            Err(e) => Some(Err(e)),
        }
    }
}
