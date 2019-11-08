use model::{AstraStatus, Error};

pub fn astra_status_to_err(status: AstraStatus) -> Error {
    Error::AstraStatusError(status)
}
pub fn astra_status_to_result<T>(status: AstraStatus, t: T) -> Result<T, Error> {
    if status == AstraStatus::Success {
        Ok(t)
    } else {
        Err(Error::AstraStatusError(status))
    }
}
