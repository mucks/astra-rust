#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstraStatus {
    Success = sys::astra_status_t_ASTRA_STATUS_SUCCESS,
    InvalidParameter = sys::astra_status_t_ASTRA_STATUS_INVALID_PARAMETER,
    DeviceError = sys::astra_status_t_ASTRA_STATUS_DEVICE_ERROR,
    Timeout = sys::astra_status_t_ASTRA_STATUS_TIMEOUT,
    InvaildParameterToken = sys::astra_status_t_ASTRA_STATUS_INVALID_PARAMETER_TOKEN,
    InvalidOperation = sys::astra_status_t_ASTRA_STATUS_INVALID_OPERATION,
    InternalError = sys::astra_status_t_ASTRA_STATUS_INTERNAL_ERROR,
    Unintialized = sys::astra_status_t_ASTRA_STATUS_UNINITIALIZED,
}

impl From<sys::astra_status_t> for AstraStatus {
    fn from(v: sys::astra_status_t) -> Self {
        unsafe { std::mem::transmute(v as u32) }
    }
}
