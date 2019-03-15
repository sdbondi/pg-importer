#[derive(Debug)]
pub enum AppError {
    DumpFileNotFound,
}

pub type HandlerResult = Result<(), AppError>;
