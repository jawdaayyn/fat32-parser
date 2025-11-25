//! parser principal FAT32

use crate::boot_sector::BootSector;
use crate::block_device::BlockDevice;
use crate::dir_entry::DirEntry;
use crate::error::Fat32Error;
use crate::fat;
use crate::fsinfo::FSInfo;

/// parser FAT32
/// 
/// # Exemples
/// 
/// ```no_run
/// use fat32_parser::parser::Fat32Parser;
/// use fat32_parser::block_device::BlockDevice;
/// 
/// // créer un parser avec un device
/// let parser = Fat32Parser::new(mon_device)?;
/// 
/// // charger FSInfo
/// parser.load_fsinfo()?;
/// 
/// // lire le répertoire racine
/// let entries = parser.read_root_dir()?;
/// ```
pub struct Fat32Parser<D: BlockDevice> {
    device: D,
    boot_sector: BootSector,
    fsinfo: Option<FSInfo>,
}

impl<D: BlockDevice> Fat32Parser<D> {
    /// crée un nouveau parser
    pub fn new(device: D) -> Result<Self, Fat32Error> {
        let mut buffer = [0u8; 512];
        device.read_sector(0, &mut buffer)?;
        
        let boot_sector = unsafe { BootSector::from_bytes(&buffer) };
        
        if !boot_sector.is_valid() {
            return Err(Fat32Error::InvalidSignature);
        }
        
        Ok(Self {
            device,
            boot_sector,
            fsinfo: None,
        })
    }
    
    /// retourne le boot sector
    pub fn boot_sector(&self) -> &BootSector {
        &self.boot_sector
    }
    
    /// charge FSInfo
    pub fn load_fsinfo(&mut self) -> Result<(), Fat32Error> {
        let mut buffer = [0u8; 512];
        let fsinfo_sector = self.boot_sector.fs_info_sector as u32;
        self.device.read_sector(fsinfo_sector, &mut buffer)?;
        
        let fsinfo = unsafe { FSInfo::from_bytes(&buffer) };
        
        if !fsinfo.is_valid() {
            return Err(Fat32Error::InvalidSignature);
        }
        
        self.fsinfo = Some(fsinfo);
        Ok(())
    }
    
    /// lit une entrée de la FAT
    pub fn read_fat_entry(&self, cluster: u32) -> Result<u32, Fat32Error> {
        if cluster < 2 {
            return Err(Fat32Error::InvalidCluster);
        }
        
        let fat_offset = cluster * 4;
        let fat_sector = self.boot_sector.fat_start_sector() + (fat_offset / 512);
        let entry_offset = (fat_offset % 512) as usize;
        
        let mut buffer = [0u8; 512];
        self.device.read_sector(fat_sector, &mut buffer)?;
        
        let entry = u32::from_le_bytes([
            buffer[entry_offset],
            buffer[entry_offset + 1],
            buffer[entry_offset + 2],
            buffer[entry_offset + 3],
        ]) & 0x0FFFFFFF;
        
        Ok(entry)
    }
    
    /// lit un cluster complet
    pub fn read_cluster(&self, cluster: u32, buffer: &mut [u8]) -> Result<(), Fat32Error> {
        let first_sector = self.boot_sector.cluster_to_sector(cluster);
        let sectors_per_cluster = self.boot_sector.sectors_per_cluster as u32;
        
        for i in 0..sectors_per_cluster {
            let offset = (i * 512) as usize;
            self.device.read_sector(first_sector + i, &mut buffer[offset..offset + 512])?;
        }
        
        Ok(())
    }
    
    /// lit les entrées du répertoire racine
    pub fn read_root_dir(&self) -> Result<[DirEntry; 16], Fat32Error> {
        let root_cluster = self.boot_sector.root_cluster;
        let cluster_size = self.boot_sector.cluster_size() as usize;
        
        let mut buffer = [0u8; 4096];
        self.read_cluster(root_cluster, &mut buffer[..cluster_size])?;
        
        let mut entries = [DirEntry {
            name: [0; 11],
            attributes: 0,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: 0,
            last_write_time: 0,
            last_write_date: 0,
            first_cluster_low: 0,
            file_size: 0,
        }; 16];
        
        for i in 0..16 {
            let offset = i * 32;
            entries[i] = unsafe {
                core::ptr::read_unaligned(buffer[offset..].as_ptr() as *const DirEntry)
            };
        }
        
        Ok(entries)
    }
    
    /// suit une chaîne de clusters
    pub fn follow_cluster_chain(&self, start_cluster: u32) -> Result<[u32; 128], Fat32Error> {
        let mut chain = [0u32; 128];
        let mut current = start_cluster;
        let mut count = 0;
        
        while !fat::is_eoc(current) && count < 128 {
            chain[count] = current;
            count += 1;
            current = self.read_fat_entry(current)?;
        }
        
        Ok(chain)
    }
    
    /// retourne le FSInfo
    pub fn fsinfo(&self) -> Option<&FSInfo> {
        self.fsinfo.as_ref()
    }
    
    /// écrit dans un cluster
    pub fn write_cluster(&mut self, cluster: u32, data: &[u8]) -> Result<(), Fat32Error> {
        let first_sector = self.boot_sector.cluster_to_sector(cluster);
        let sectors_per_cluster = self.boot_sector.sectors_per_cluster as u32;
        
        for i in 0..sectors_per_cluster {
            let offset = (i * 512) as usize;
            let end = offset + 512;
            if end <= data.len() {
                self.device.write_sector(first_sector + i, &data[offset..end])?;
            }
        }
        
        Ok(())
    }
    
    /// trouve un cluster libre
    pub fn find_free_cluster(&self) -> Result<u32, Fat32Error> {
        let total_clusters = self.boot_sector.total_sectors_32 / self.boot_sector.sectors_per_cluster as u32;
        
        for cluster in 2..total_clusters {
            let entry = self.read_fat_entry(cluster)?;
            if fat::is_free(entry) {
                return Ok(cluster);
            }
        }
        
        Err(Fat32Error::NotFound)
    }
    
    /// écrit une entrée dans la FAT
    pub fn write_fat_entry(&mut self, cluster: u32, value: u32) -> Result<(), Fat32Error> {
        if cluster < 2 {
            return Err(Fat32Error::InvalidCluster);
        }
        
        let fat_offset = cluster * 4;
        let fat_sector = self.boot_sector.fat_start_sector() + (fat_offset / 512);
        let entry_offset = (fat_offset % 512) as usize;
        
        let mut buffer = [0u8; 512];
        self.device.read_sector(fat_sector, &mut buffer)?;
        
        let masked_value = value & 0x0FFFFFFF;
        let bytes = masked_value.to_le_bytes();
        buffer[entry_offset] = bytes[0];
        buffer[entry_offset + 1] = bytes[1];
        buffer[entry_offset + 2] = bytes[2];
        buffer[entry_offset + 3] = bytes[3];
        
        self.device.write_sector(fat_sector, &buffer)?;
        
        Ok(())
    }
}

