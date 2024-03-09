use std::sync::Arc;

use crate::performance::gpu::tdp::{TDPDevice, TDPResult, TDPError};

use rog_dbus::RogDbusClient;
use rog_platform::platform::RogPlatform;
/*
use rog_platform::platform::{GpuMode, Properties, ThrottlePolicy};
use rog_profiles::error::ProfileError;
*/

use std::sync::Mutex;

/// Implementation of asusd with a fallback to asus-wmi sysfs
/// See https://www.kernel.org/doc/html/v6.8-rc4/admin-guide/abi-testing.html#abi-sys-devices-platform-platform-ppt-apu-sppt
pub struct ASUS {
    platform: Arc<Mutex<RogPlatform>>,
}

impl ASUS {

    /// test if we are in an asus system with asus-wmi loaded
    pub async fn new() -> Option<Self> {
        // For now complile-time disable the usage of asus specific interface
        return None;

        match RogPlatform::new() {
            Ok(platform) => {
                log::info!("Module asus-wmi has been found -- scanning the dbus interface...");

                match RogDbusClient::new().await {
                    Ok((dbus, _)) => {
                        log::info!("asusd dbus interface available");

                        let asus_platform = dbus.proxies().rog_bios();
        
                        match asus_platform.supported_properties().await {
                            Ok(supported_properties) => {
                                for prop in supported_properties {
                                    dbg!(prop);
                                }
                                
                            },
                            Err(err) => {
                                log::warn!("Unable to query asusd for supported properties: {}", err);
                            }
                        }
        
                        match asus_platform.supported_interfaces().await {
                            Ok(supported_properties) => {
                                for prop in supported_properties {
                                    dbg!(prop);
                                }
                                
                            },
                            Err(err) => {
                                log::warn!("Unable to query asusd for supported properties: {}", err);
                            }
                        }
                    },
                    Err(err) => {
                        log::warn!("Unable to connect to asusd: {} -- asus-wmi may be used instead", err);
                    }
                };

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
                let asus_platform = dbus.proxies().rog_bios();
                match asus_platform.ppt_apu_sppt().await {
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
