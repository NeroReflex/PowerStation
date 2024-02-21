use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::performance::gpu::tdp::{TDPDevice, TDPError, TDPResult};

/// Implementation of TDP control for Intel GPUs
pub struct TDP {
    pub path: String,
}

impl TDP {
    pub fn new(path: String) -> TDP {
        TDP { path }
    }
}

impl TDPDevice for TDP {

    fn tdp(&self) -> impl std::future::Future<Output = TDPResult<f64>> {
        async move {
            let path = "/sys/class/powercap/intel-rapl/intel-rapl:0/constraint_0_power_limit_uw";
            let result = fs::read_to_string(path);
            let content = result.map_err(|err| TDPError::IOError(err.to_string()))?;

            // Parse the output to get the long TDP
            let long_tdp = match content.parse::<f64>() {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e);
                    return Err(TDPError::FailedOperation(e.to_string()));
                }
            };

            Ok(long_tdp / 1000000.0)
        }
    }

    fn set_tdp(&mut self, value: f64) -> impl std::future::Future<Output = TDPResult<()>> {
        async move {
            if value < 1.0 {
                let err = "Cowardly refusing to set TDP less than 1";
                log::warn!("{}", err);
                return Err(TDPError::InvalidArgument(String::from(err)));
            }

            // Get the current boost value so the peak tdp can be set *boost*
            // distance away.
            let boost = self.boost().await?;

            // Open the sysfs file to write to
            let path = "/sys/class/powercap/intel-rapl/intel-rapl:0/constraint_0_power_limit_uw";
            let file = OpenOptions::new().write(true).open(path);

            // Convert the value to a writable string
            let value = format!("{}", value * 1000000.0);

            // Write the value
            file
                .map_err(|err| TDPError::FailedOperation(err.to_string()))?
                .write_all(value.as_bytes())
                .map_err(|err| TDPError::IOError(err.to_string()))?;

            // Update the boost value
            Ok(self.set_boost(boost).await?)
        }
    }

    fn boost(&self) -> impl std::future::Future<Output = TDPResult<f64>> {
        async move {
            let path = "/sys/class/powercap/intel-rapl/intel-rapl:0/constraint_2_power_limit_uw";
            let result = fs::read_to_string(path);
            let content = result
                .map_err(|err| TDPError::IOError(err.to_string()))?;

            // Parse the output to get the peak TDP
            let peak_tdp = match content.parse::<f64>() {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e);
                    return Err(TDPError::FailedOperation(e.to_string()));
                }
            };

            let tdp = self.tdp()?;
            Ok((peak_tdp / 1000000.0) - tdp)
        }
    }

    fn set_boost(&mut self, value: f64) -> impl std::future::Future<Output = TDPResult<()>> {
        async move {
            if value < 0.0 {
                let err = "Cowardly refusing to set TDP Boost less than 0";
                log::warn!("{}", err);
                return Err(TDPError::InvalidArgument(String::from(err)));
            }

            let tdp = self.tdp()?;
            let boost = value.clone();
            let short_tdp = if boost > 0.0 {
                ((boost / 2.0) + tdp) * 1000000.0
            } else {
                tdp * 1000000.0
            };

            // Write the short tdp
            let path = "/sys/class/powercap/intel-rapl/intel-rapl:0/constraint_1_power_limit_uw";
            let file = OpenOptions::new().write(true).open(path);
            let value = format!("{}", short_tdp);
            file
                .map_err(|err| TDPError::FailedOperation(err.to_string()))?
                .write_all(value.as_bytes())
                .map_err(|err| TDPError::IOError(err.to_string()))?;

            // Write the peak tdp
            let path = "/sys/class/powercap/intel-rapl/intel-rapl:0/constraint_2_power_limit_uw";
            let file = OpenOptions::new().write(true).open(path);
            let value = format!("{}", (boost + tdp) * 1000000.0);
            file
                .map_err(|err| TDPError::FailedOperation(err.to_string()))?
                .write_all(value.as_bytes())
                .map_err(|err| TDPError::IOError(err.to_string()))
        }
    }

    fn thermal_throttle_limit_c(&self) -> impl std::future::Future<Output = TDPResult<f64>> {
        async {
            log::error!("Thermal throttling not supported on intel gpu");
            Err(TDPError::FeatureUnsupported)
        }
    }

    fn set_thermal_throttle_limit_c(&mut self, _limit: f64) -> impl std::future::Future<Output = TDPResult<()>> {
        async move {
            log::error!("Thermal throttling not supported on intel gpu");
            Err(TDPError::FeatureUnsupported)
        }
    }

    fn power_profile(&self) -> impl std::future::Future<Output = TDPResult<String>> {
        async move {
            log::error!("Power profiles not supported on intel gpu");
            Err(TDPError::FeatureUnsupported)
        }
    }

    fn set_power_profile(&mut self, _profile: String) -> impl std::future::Future<Output = TDPResult<()>> {
        async move {
            log::error!("Power profiles not supported on intel gpu");
            Err(TDPError::FeatureUnsupported)
        }
    }
}
