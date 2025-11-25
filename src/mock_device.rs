//! mock device pour tests

#[cfg(test)]
use crate::traits::block_device::BlockDevice;
#[cfg(test)]
use crate::utils::error::Fat32Error;

#[cfg(test)]
extern crate std;

#[cfg(test)]
pub struct MockDevice {
    data: std::vec::Vec<u8>, // utiliser Vec plutôt qu'un tableau géant
}

#[cfg(test)]
impl MockDevice {
    pub fn new() -> Self {
        Self {
            data: std::vec![0; 512 * 10000], // 5 MB
        }
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

