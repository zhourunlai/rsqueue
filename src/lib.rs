/*
A task queue using redis as backend.
*/

extern crate serde;
extern crate serde_json;
extern crate uuid;

mod job;
mod queue;

pub use job::Job;
pub use queue::Queue;
