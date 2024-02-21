use std::sync::Arc;
use zbus::fdo::Error;
use zbus::fdo;
use zbus_macros::dbus_interface;

use tokio::sync::Mutex;

use crate::performance::gpu::tdp::TDPError;
use crate::performance::gpu::tdp::{TDPDevice, TDPResult};

pub struct GPUTDPDBusIface {
    dev: Arc<Mutex<dyn TDPDevice>>
}

impl Into<fdo::Error> for TDPError {
    fn into(self) -> zbus::fdo::Error {
        match &self {
            Self::FailedOperation(err) => fdo::Error::Failed(err.to_string()),
            Self::FeatureUnsupported => fdo::Error::Failed(String::from("Unsupported feature")),
            Self::InvalidArgument(err) => fdo::Error::Failed(err.to_string()),
            Self::IOError(err) => fdo::Error::IOError(err.to_string())
        }
    }
}

impl GPUTDPDBusIface {
    pub fn new(dev: Arc<Mutex<dyn TDPDevice>>) -> GPUTDPDBusIface {
        GPUTDPDBusIface {
            dev
        }
    }
}

#[dbus_interface(name = "org.shadowblip.GPU.Card.TDP")]
impl GPUTDPDBusIface {

    /// Get the currently set TDP value
    #[dbus_interface(property, name = "TDP")]
    async fn tdp(&self) -> fdo::Result<f64> {
        match self.dev.lock().await.tdp().await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

    /// Sets the given TDP value
    #[dbus_interface(property, name = "TDP")]
    async fn set_tdp(&mut self, value: f64) -> fdo::Result<()> {
        match self.dev.lock().await.set_tdp(value).await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

    /// The TDP boost for AMD is the total difference between the Fast PPT Limit
    /// and the STAPM limit.
    #[dbus_interface(property)]
    async fn boost(&self) -> fdo::Result<f64> {
        match self.dev.lock().await.boost().await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

    #[dbus_interface(property)]
    async fn set_boost(&mut self, value: f64) -> fdo::Result<()> {
        match self.dev.lock().await.set_boost(value).await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

    #[dbus_interface(property)]
    async fn thermal_throttle_limit_c(&self) -> fdo::Result<f64> {
        match self.dev.lock().await.thermal_throttle_limit_c().await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

    #[dbus_interface(property)]
    async fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> fdo::Result<()> {
        match self.dev.lock().await.set_thermal_throttle_limit_c(limit).await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

    #[dbus_interface(property)]
    async fn power_profile(&self) -> fdo::Result<String> {
        match self.dev.lock().await.power_profile().await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

    #[dbus_interface(property)]
    async fn set_power_profile(&mut self, profile: String) -> fdo::Result<()> {
        match self.dev.lock().await.set_power_profile(profile).await {
            Ok(exec_res) => match exec_res {
                TDPResult::Ok(result) => Ok(result),
                TDPResult::Err(err) => Err(err.into())
            },
            Err(err) => Err(fdo::Error::Failed(
                format!("Error getting TDP: {:?}", err),
            )),
        }
    }

}