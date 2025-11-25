//! structure FSInfo

/// FSInfo (512 octets)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FSInfo {
    pub lead_signature: u32,        // 0x41615252
    pub reserved1: [u8; 480],
    pub struct_signature: u32,      // 0x61417272
    pub free_count: u32,            // clusters libres
    pub next_free: u32,             // prochain cluster libre
    pub reserved2: [u8; 12],
    pub trail_signature: u32,       // 0xAA550000
}

impl FSInfo {
    /// # Safety
    /// buffer doit être 512 octets valides
    pub unsafe fn from_bytes(data: &[u8; 512]) -> Self {
        core::ptr::read_unaligned(data.as_ptr() as *const FSInfo)
    }
    
    pub fn is_valid(&self) -> bool {
        self.lead_signature == 0x41615252 
            && self.struct_signature == 0x61417772
            && self.trail_signature == 0xAA550000
    }
    
    /// retourne le nombre de clusters libres
    pub fn free_clusters(&self) -> Option<u32> {
        if self.free_count == 0xFFFFFFFF {
            None
        } else {
            Some(self.free_count)
        }
    }
    
    /// retourne le prochain cluster libre
    pub fn next_free_cluster(&self) -> Option<u32> {
        if self.next_free == 0xFFFFFFFF {
            None
        } else {
            Some(self.next_free)
        }
    }
}

