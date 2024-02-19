use std::path::Path;

use crate::performance::gpu::tdp::{TDPDevice, TDPResult, TDPError};

use zbus::{Connection, Result};

use rog_dbus::RogDbusClientBlocking;
use rog_dbus::DbusProxies;
use rog_platform::error::PlatformError;
use rog_platform::platform::{GpuMode, Properties, ThrottlePolicy};
use rog_profiles::error::ProfileError;


/// Implementation of asusd with a fallback to asus-wmi sysfs
/// See https://www.kernel.org/doc/html/v6.8-rc4/admin-guide/abi-testing.html#abi-sys-devices-platform-platform-ppt-apu-sppt
pub struct ASUS {
    
}

impl ASUS {

    /// test if we are in an asus system with asus-wmi loaded
    pub fn new() -> Option<Self> {
        

        match RogDbusClientBlocking::new() {
            Ok((dbus, _)) => {
                let supported_properties = dbus.proxies().platform().supported_properties().unwrap();
                let supported_interfaces = dbus.proxies().platform().supported_interfaces().unwrap();

                Some(Self {
                        
                })
            },
            Err(err) => {
                let asus_wmi_path = Path::new("");

                if asus_wmi_path.exists() {
                    Some(Self {
                        
                    })
                } else {
                    None
                }
            }
        }
    }

}

impl TDPDevice for ASUS {
    fn tdp(&self) -> TDPResult<f64> {
        match RogDbusClientBlocking::new() {
            Ok((dbus, _)) => {
                let supported_properties = dbus.proxies().platform().supported_properties().unwrap();
                let supported_interfaces = dbus.proxies().platform().supported_interfaces().unwrap();

                dbus.proxies().platform().ppt_apu_sppt();
            },
            Err(err) => {

            }
        }

        
        todo!()
    }

    fn set_tdp(&mut self, value: f64) -> TDPResult<()> {
        todo!()
    }

    fn boost(&self) -> TDPResult<f64> {
        todo!()
    }

    fn set_boost(&mut self, value: f64) -> TDPResult<()> {
        todo!()
    }

    fn thermal_throttle_limit_c(&self) -> TDPResult<f64> {
        todo!()
    }

    fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> TDPResult<()> {
        todo!()
    }

    fn power_profile(&self) -> TDPResult<String> {
        todo!()
    }

    fn set_power_profile(&mut self, profile: String) -> TDPResult<()> {
        todo!()
    }

}