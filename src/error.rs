use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("创建请求失败: {0}")]
    RequestError(String),
}
