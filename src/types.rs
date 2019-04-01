#[derive(Error, Debug)]
pub enum AppError {
    /// Unable to open dump file
    CannotOpenDumpFile,
}

pub type HandlerResult = Result<(), AppError>;
