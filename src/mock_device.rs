//! mock device pour tests

#[cfg(test)]
use crate::block_device::BlockDevice;
#[cfg(test)]
use crate::error::Fat32Error;

#[cfg(test)]
pub struct MockDevice {
    data: [u8; 512 * 1024],
}

#[cfg(test)]
impl MockDevice {
    pub fn new() -> Self {
        Self {
            data: [0; 512 * 1024],
        }
    }
    
    pub fn with_data(data: [u8; 512 * 1024]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
impl BlockDevice for MockDevice {
    fn read_sector(&self, sector: u32, buffer: &mut [u8]) -> Result<(), Fat32Error> {
        let offset = (sector as usize) * 512;
        if offset + 512 > self.data.len() {
            return Err(Fat32Error::InvalidSector);
        }
        buffer.copy_from_slice(&self.data[offset..offset + 512]);
        Ok(())
    }
    
    fn write_sector(&mut self, sector: u32, buffer: &[u8]) -> Result<(), Fat32Error> {
        let offset = (sector as usize) * 512;
        if offset + 512 > self.data.len() {
            return Err(Fat32Error::InvalidSector);
        }
        self.data[offset..offset + 512].copy_from_slice(buffer);
        Ok(())
    }
}

