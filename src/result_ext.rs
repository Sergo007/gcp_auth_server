use tracing::*;

pub trait ResultExt<T, E> {
    fn log(self, context: &str) -> Result<T, E>;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for Result<T, E> {
    #[track_caller]
    fn log(self, context: &str) -> Result<T, E> {
        if self.is_err() {
            let caller_location = std::panic::Location::caller();
            let caller_line_file = caller_location.file();
            let caller_line_number = caller_location.line();
            error!(target: "normal", "{caller_line_file}:{caller_line_number} {context}. Err {err}", err=self.as_ref().err().unwrap());
        }
        self
    }
}
