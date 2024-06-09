use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
pub struct Device {
    serial_number: String,
    firmware_version: String,
}

#[wasm_bindgen]
impl Device {
    #[wasm_bindgen(constructor)]
    pub fn new(serial_number: String, firmware_version: String) -> Device {
        Device {
            serial_number,
            firmware_version,
        }
    }

    pub fn update_firmware(&mut self, new_version: String) {
        self.firmware_version = new_version;
    }

    pub fn get_firmware_version(&self) -> String {
        self.firmware_version.clone()
    }

    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }
}

#[wasm_bindgen]
pub fn create_device(serial_number: String, firmware_version: String) -> Device {
    Device::new(serial_number, firmware_version)
}
