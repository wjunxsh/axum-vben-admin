use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T> {
    code: u32,
    data: T,
}

impl<T> Response<T> {
    pub fn new(code: u32, data: T) -> Self {
        Self {
            code,
            data,
        }
    }
}