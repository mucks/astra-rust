#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum AstraStatus {
    Success = sys::astra_status_t_ASTRA_STATUS_SUCCESS as isize,
    InvalidParameter = sys::astra_status_t_ASTRA_STATUS_INVALID_PARAMETER as isize,
    DeviceError = sys::astra_status_t_ASTRA_STATUS_DEVICE_ERROR as isize,
    Timeout = sys::astra_status_t_ASTRA_STATUS_TIMEOUT as isize,
    InvaildParameterToken = sys::astra_status_t_ASTRA_STATUS_INVALID_PARAMETER_TOKEN as isize,
    InvalidOperation = sys::astra_status_t_ASTRA_STATUS_INVALID_OPERATION as isize,
    InternalError = sys::astra_status_t_ASTRA_STATUS_INTERNAL_ERROR as isize,
    Unintialized = sys::astra_status_t_ASTRA_STATUS_UNINITIALIZED as isize,
}

impl From<sys::astra_status_t> for AstraStatus {
    fn from(astra_status: sys::astra_status_t) -> Self {
        num::FromPrimitive::from_u32(astra_status)
            .unwrap_or_else(|| panic!("could not parse astra status: {}", astra_status))
    }
}
