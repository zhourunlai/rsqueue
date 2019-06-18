use redis::{ErrorKind, RedisResult, Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: String,
}

impl Job {
    pub fn new() -> Job {
        Job {
            id: Uuid::new_v4().to_string(),
        }
    }
}

pub trait JobEncodable {
    fn encode(&self) -> Vec<u8>;
}

pub trait JobDecodable
where
    Self: Sized,
{
    fn decode(value: &Value) -> RedisResult<Self>;
}

impl<T: Serialize> JobEncodable for T {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

impl<T: DeserializeOwned> JobDecodable for T {
    fn decode(value: &Value) -> RedisResult<T> {
        match *value {
            Value::Data(ref v) => {
                serde_json::from_slice(v).map_err(|_| From::from((ErrorKind::TypeError, "Decode failed")))
            }

            _ => return Err(From::from((ErrorKind::TypeError, "Decode error"))),
        }
    }
}
