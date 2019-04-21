#[derive(Error, Debug)]
pub enum AppError {
    /// Unable to open dump file
    CannotOpenDumpFile,
    /// Unable to write to outfile
    CannotWriteToOutfile,
}

pub type HandlerResult = Result<(), AppError>;
