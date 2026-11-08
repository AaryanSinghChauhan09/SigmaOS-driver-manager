#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptor {
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverManagerError {
    EmptyName,
    DuplicateName(String),
    DuplicateDeviceMatch { vendor_id: u16, device_id: u16 },
    NotFound(String),
}

impl std::fmt::Display for DriverManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(f, "driver name must not be empty"),
            Self::DuplicateName(name) => write!(f, "driver name is already registered: {name}"),
            Self::DuplicateDeviceMatch { vendor_id, device_id } => {
                write!(f, "device match is already registered: {vendor_id:04x}:{device_id:04x}")
            }
            Self::NotFound(name) => write!(f, "driver is not registered: {name}"),
        }
    }
}

impl std::error::Error for DriverManagerError {}

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

    pub fn register_driver(
        &mut self,
        name: &str,
        vendor: u16,
        device: u16,
    ) -> Result<(), DriverManagerError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(DriverManagerError::EmptyName);
        }
        if self.drivers.iter().any(|driver| driver.name == name) {
            return Err(DriverManagerError::DuplicateName(name.to_string()));
        }
        if self.drivers.iter().any(|driver| {
            driver.supported_vendor == vendor && driver.supported_device == device
        }) {
            return Err(DriverManagerError::DuplicateDeviceMatch {
                vendor_id: vendor,
                device_id: device,
            });
        }
        self.drivers.push(DeviceDriver {
            name: name.to_string(),
            supported_vendor: vendor,
            supported_device: device,
            status: DriverStatus::Loaded,
        });
        Ok(())
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

    /// Query the lifecycle status without exposing mutable driver storage.
    pub fn status(&self, name: &str) -> Option<&DriverStatus> {
        self.drivers
            .iter()
            .find(|driver| driver.name == name)
            .map(|driver| &driver.status)
    }

    /// Detach an active driver and return it to the registered state.
    pub fn detach(&mut self, name: &str) -> Result<(), DriverManagerError> {
        let driver = self
            .drivers
            .iter_mut()
            .find(|driver| driver.name == name)
            .ok_or_else(|| DriverManagerError::NotFound(name.to_string()))?;
        if driver.status == DriverStatus::Active {
            driver.status = DriverStatus::Loaded;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_registration_and_match() {
        // Arrange
        let mut mgr = DriverManager::new();
        mgr.register_driver("virtio-net", 0x1af4, 0x1000).unwrap();
        let dev = DeviceDescriptor { vendor_id: 0x1af4, device_id: 0x1000, class_code: 0x02 };

        // Act
        let matched = mgr.match_and_attach(&dev);

        // Assert
        assert_eq!(matched, Some("virtio-net"));
        assert_eq!(mgr.driver_count(), 1);
    }
}
