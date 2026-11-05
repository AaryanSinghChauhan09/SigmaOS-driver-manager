#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptor {
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverStatus {
    Loaded,
    Active,
    Failed,
    Detached,
}

#[derive(Debug)]
pub struct DeviceDriver {
    pub name: String,
    pub supported_vendor: u16,
    pub supported_device: u16,
    pub status: DriverStatus,
}

#[derive(Debug, Default)]
pub struct DriverManager {
    drivers: Vec<DeviceDriver>,
}

impl DriverManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_driver(&mut self, name: &str, vendor: u16, device: u16) {
        self.drivers.push(DeviceDriver {
            name: name.to_string(),
            supported_vendor: vendor,
            supported_device: device,
            status: DriverStatus::Loaded,
        });
    }

    pub fn match_and_attach(&mut self, dev: &DeviceDescriptor) -> Option<&str> {
        for d in &mut self.drivers {
            if d.supported_vendor == dev.vendor_id && d.supported_device == dev.device_id {
                d.status = DriverStatus::Active;
                return Some(&d.name);
            }
        }
        None
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_registration_and_match() {
        // Arrange
        let mut mgr = DriverManager::new();
        mgr.register_driver("virtio-net", 0x1af4, 0x1000);
        let dev = DeviceDescriptor { vendor_id: 0x1af4, device_id: 0x1000, class_code: 0x02 };

        // Act
        let matched = mgr.match_and_attach(&dev);

        // Assert
        assert_eq!(matched, Some("virtio-net"));
        assert_eq!(mgr.driver_count(), 1);
    }
}
