use tokio::task::JoinHandle;

pub enum TDPError {
    FeatureUnsupported,
    FailedOperation(String),
    InvalidArgument(String),
    IOError(String),
}

impl Into<String> for TDPError {
    fn into(self) -> std::string::String {
        todo!()
    }
}

pub type TDPResult<T> = Result<T, TDPError>;

pub trait TDPDevice : Sync + Send {

    fn tdp(&self) -> JoinHandle<TDPResult<f64>>;
    fn set_tdp(&mut self, value: f64) -> JoinHandle<TDPResult<()>>;
    fn boost(&self) -> JoinHandle<TDPResult<f64>>;
    fn set_boost(&mut self, value: f64) -> JoinHandle<TDPResult<()>>;
    fn thermal_throttle_limit_c(&self) -> JoinHandle<TDPResult<f64>>;
    fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> JoinHandle<TDPResult<()>>;
    fn power_profile(&self) -> JoinHandle<TDPResult<String>>;
    fn set_power_profile(&mut self, profile: String) -> JoinHandle<TDPResult<()>>;

}