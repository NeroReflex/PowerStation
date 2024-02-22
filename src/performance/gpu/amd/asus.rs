use std::sync::Arc;
use udev::{Enumerator, Device};

use crate::performance::gpu::tdp::{TDPDevice, TDPResult, TDPError};
use crate::performance::gpu::dbus::devices::TDPDevices;

use zbus::{Connection, Result};

use rog_dbus::{ RogDbusClientBlocking, RogDbusClient };
use rog_dbus::DbusProxies;
use rog_platform::{platform::RogPlatform, error::PlatformError};
use rog_platform::platform::{GpuMode, Properties, ThrottlePolicy};
use rog_profiles::error::ProfileError;

use std::sync::Mutex;

/// Implementation of asusd with a fallback to asus-wmi sysfs
/// See https://www.kernel.org/doc/html/v6.8-rc4/admin-guide/abi-testing.html#abi-sys-devices-platform-platform-ppt-apu-sppt
pub struct ASUS {
    platform: Arc<Mutex<RogPlatform>>,
}

impl ASUS {

    /// test if we are in an asus system with asus-wmi loaded
    pub fn new() -> Option<Self> {
        match RogPlatform::new() {
            Ok(platform) => {
                log::info!("Module asus-wmi WAS found");
                Some(Self {
                    platform: Arc::new(Mutex::new(platform))
                })
            },
            Err(err) => {
                log::info!("Module asus-wmi not found: {}", err);
                None
            }
        }
    }

}

impl TDPDevice for ASUS {
    async fn tdp(&self) -> TDPResult<f64> {
        match RogDbusClient::new().await {
            Ok((dbus, _)) => {
                let supported_properties = dbus.proxies().rog_bios().supported_properties().await.unwrap();
                let supported_interfaces = dbus.proxies().rog_bios().supported_interfaces().await.unwrap();

                match dbus.proxies().rog_bios().ppt_apu_sppt().await {
                    Ok(result) => {
                        log::info!("Initial ppt_apu_sppt: {}", result);
                        Ok(result as f64)
                    },
                    Err(err) => {
                        log::warn!("Error fetching ppt_apu_sppt: {}", err);
                        Err(TDPError::FailedOperation(format!("")))
                    }
                }
            },
            Err(err) => {
                log::warn!("Unable to use asusd to read tdp: {} -- asus-wmi interface will be used", err);
                Err(TDPError::FailedOperation(format!("")))
            }
        }
    }

    async fn set_tdp(&mut self, value: f64) -> TDPResult<()> {
        log::error!("Set fake TDP of {}", value);
        
        Ok(())
    }

    async fn boost(&self) -> TDPResult<f64> {
        Ok(120.0)
    }

    async fn set_boost(&mut self, value: f64) -> TDPResult<()> {
        println!("Set fake boost of {}", value);
        
        Ok(())
    }

    async fn thermal_throttle_limit_c(&self) -> TDPResult<f64> {
        Ok(256.0)
    }

    async fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> TDPResult<()> {
        println!("Set fake thermal throttle limit of {}", limit);

        Ok(())
    }

    async fn power_profile(&self) -> TDPResult<String> {
        Ok(String::from("max-performance"))
    }

    async fn set_power_profile(&mut self, profile: String) -> TDPResult<()> {
        println!("Set fake power profile of {}", profile);
        
        Ok(())
    }
}
