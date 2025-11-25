//! tests unitaires

#[cfg(test)]
mod tests {
    use crate::structures::boot_sector::BootSector;
    use crate::utils::fat::*;
    use crate::structures::dir_entry::*;
    use crate::operations::file_ops::*;
    use crate::utils::helpers::*;
    use crate::utils::validator::*;

    #[test]
    fn test_fat_is_eoc() {
        assert!(is_eoc(0x0FFFFFF8));
        assert!(is_eoc(0x0FFFFFFF));
        assert!(!is_eoc(0x00000002));
    }

    #[test]
    fn test_fat_is_free() {
        assert!(is_free(0x00000000));
        assert!(!is_free(0x00000002));
    }

    #[test]
    fn test_fat_is_bad() {
        assert!(is_bad(0x0FFFFFF7));
        assert!(!is_bad(0x00000002));
    }
    
    #[test]
    fn test_fat_mask() {
        assert_eq!(mask_cluster(0xF0000005), 0x00000005);
        assert_eq!(mask_cluster(0x0FFFFFFF), 0x0FFFFFFF);
    }

    #[test]
    fn test_boot_sector_size() {
        assert_eq!(core::mem::size_of::<BootSector>(), 512);
    }

    #[test]
    fn test_dir_entry_size() {
        assert_eq!(core::mem::size_of::<DirEntry>(), 32);
    }

    #[test]
    fn test_dir_entry_attributes() {
        let mut entry = DirEntry {
            name: [0; 11],
            attributes: ATTR_READ_ONLY,
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
        };
        
        assert!(entry.is_read_only());
        assert!(!entry.is_directory());
        
        entry.attributes = ATTR_DIRECTORY;
        assert!(entry.is_directory());
        assert!(!entry.is_read_only());
    }

    #[test]
    fn test_dir_entry_cluster() {
        let entry = DirEntry {
            name: [0; 11],
            attributes: 0,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: 0x0001,
            last_write_time: 0,
            last_write_date: 0,
            first_cluster_low: 0x0002,
            file_size: 0,
        };
        
        assert_eq!(entry.first_cluster(), 0x00010002);
    }
    
    #[test]
    fn test_boot_sector_cluster_size() {
        let bs = BootSector {
            jmp_boot: [0; 3],
            oem_name: [0; 8],
            bytes_per_sector: 512,
            sectors_per_cluster: 8,
            reserved_sector_count: 32,
            num_fats: 2,
            root_entry_count: 0,
            total_sectors_16: 0,
            media_type: 0xF8,
            fat_size_16: 0,
            sectors_per_track: 0,
            num_heads: 0,
            hidden_sectors: 0,
            total_sectors_32: 1024000,
            fat_size_32: 1000,
            ext_flags: 0,
            fs_version: 0,
            root_cluster: 2,
            fs_info_sector: 1,
            backup_boot_sector: 6,
            reserved: [0; 12],
            drive_number: 0x80,
            reserved1: 0,
            boot_signature: 0x29,
            volume_id: 0,
            volume_label: [0; 11],
            fs_type: [0; 8],
            boot_code: [0; 420],
            signature: 0xAA55,
        };
        
        assert_eq!(bs.cluster_size(), 4096);
        assert!(bs.is_valid());
    }
    
    #[test]
    fn test_boot_sector_calculations() {
        let bs = BootSector {
            jmp_boot: [0; 3],
            oem_name: [0; 8],
            bytes_per_sector: 512,
            sectors_per_cluster: 8,
            reserved_sector_count: 32,
            num_fats: 2,
            root_entry_count: 0,
            total_sectors_16: 0,
            media_type: 0xF8,
            fat_size_16: 0,
            sectors_per_track: 0,
            num_heads: 0,
            hidden_sectors: 0,
            total_sectors_32: 1024000,
            fat_size_32: 1000,
            ext_flags: 0,
            fs_version: 0,
            root_cluster: 2,
            fs_info_sector: 1,
            backup_boot_sector: 6,
            reserved: [0; 12],
            drive_number: 0x80,
            reserved1: 0,
            boot_signature: 0x29,
            volume_id: 0,
            volume_label: [0; 11],
            fs_type: [0; 8],
            boot_code: [0; 420],
            signature: 0xAA55,
        };
        
        assert_eq!(bs.fat_start_sector(), 32);
        assert_eq!(bs.data_start_sector(), 2032);
        assert_eq!(bs.cluster_to_sector(2), 2032);
        assert_eq!(bs.cluster_to_sector(3), 2040);
        assert_eq!(bs.total_sectors(), 1024000);
        assert_eq!(bs.fat_size(), 1000);
    }
    
    #[test]
    fn test_format_short_name() {
        let name1 = format_short_name("test.txt");
        assert_eq!(&name1[0..4], b"TEST");
        assert_eq!(&name1[8..11], b"TXT");
        
        let name2 = format_short_name("readme");
        assert_eq!(&name2[0..6], b"README");
    }
    
    #[test]
    fn test_create_file_entry() {
        let name = format_short_name("test.txt");
        let entry = create_file_entry(name, 0x12345, 1024);
        
        assert_eq!(entry.first_cluster(), 0x12345);
        assert_eq!(entry.file_size, 1024);
        assert!(entry.is_file());
        assert!(!entry.is_directory());
    }
    
    #[test]
    fn test_create_dir_entry() {
        let name = format_short_name("mydir");
        let entry = create_dir_entry(name, 0x100);
        
        assert_eq!(entry.first_cluster(), 0x100);
        assert_eq!(entry.file_size, 0);
        assert!(entry.is_directory());
        assert!(!entry.is_file());
    }
    
    #[test]
    fn test_is_valid_name() {
        assert!(is_valid_name("test.txt"));
        assert!(is_valid_name("readme"));
        assert!(is_valid_name("a.b"));
        assert!(!is_valid_name(""));
        assert!(!is_valid_name("verylongname.txt"));
        assert!(!is_valid_name("bad*name"));
    }
    
    #[test]
    fn test_names_match() {
        let name1 = format_short_name("test.txt");
        let name2 = format_short_name("test.txt");
        let name3 = format_short_name("other.txt");
        
        assert!(names_match(&name1, &name2));
        assert!(!names_match(&name1, &name3));
    }
    
    #[test]
    fn test_mock_device() {
        use crate::mock_device::MockDevice;
        use crate::traits::block_device::BlockDevice;
        
        let mut device = MockDevice::new();
        let mut buffer = [0u8; 512];
        buffer[0] = 0xAA;
        buffer[511] = 0x55;
        
        device.write_sector(0, &buffer).unwrap();
        
        let mut read_buffer = [0u8; 512];
        device.read_sector(0, &mut read_buffer).unwrap();
        
        assert_eq!(read_buffer[0], 0xAA);
        assert_eq!(read_buffer[511], 0x55);
    }
    
    #[test]
    fn test_file_info_from_entry() {
        use crate::operations::file_info::FileInfo;
        
        let entry = create_file_entry(format_short_name("test.txt"), 100, 2048);
        let info = FileInfo::from_dir_entry(&entry);
        
        assert_eq!(info.first_cluster, 100);
        assert_eq!(info.size, 2048);
        assert!(!info.is_directory);
    }
    
    #[test]
    fn test_short_name_to_string() {
        let name = format_short_name("test.txt");
        let result = short_name_to_string(&name);
        
        assert_eq!(&result[0..8], b"TEST.TXT");
    }
    
    #[test]
    fn test_lfn_checksum() {
        let name = format_short_name("test.txt");
        let checksum = lfn_checksum(&name);
        
        assert!(checksum > 0);
    }
    
    #[test]
    fn test_constants() {
        use crate::utils::constants::*;
        
        assert_eq!(SECTOR_SIZE, 512);
        assert_eq!(BOOT_SIGNATURE, 0xAA55);
        assert_eq!(FIRST_VALID_CLUSTER, 2);
    }
    
    #[test]
    fn test_dir_entry_special() {
        let mut entry = create_file_entry(format_short_name("test.txt"), 100, 512);
        
        assert!(!entry.is_dot());
        assert!(!entry.is_dotdot());
        
        entry.mark_deleted();
        assert!(entry.is_empty());
    }
    
    #[test]
    fn test_error_result_type() {
        use crate::utils::error::{Fat32Error, Result};
        
        let ok_result: Result<u32> = Ok(42);
        let err_result: Result<u32> = Err(Fat32Error::NotFound);
        
        assert!(ok_result.is_ok());
        assert!(err_result.is_err());
    }
    
    #[test]
    fn test_validate_boot_sector() {
        let valid_bs = BootSector {
            jmp_boot: [0xEB, 0x58, 0x90],
            oem_name: *b"MSWIN4.1",
            bytes_per_sector: 512,
            sectors_per_cluster: 8,
            reserved_sector_count: 32,
            num_fats: 2,
            root_entry_count: 0,
            total_sectors_16: 0,
            media_type: 0xF8,
            fat_size_16: 0,
            sectors_per_track: 63,
            num_heads: 255,
            hidden_sectors: 0,
            total_sectors_32: 1024000,
            fat_size_32: 1000,
            ext_flags: 0,
            fs_version: 0,
            root_cluster: 2,
            fs_info_sector: 1,
            backup_boot_sector: 6,
            reserved: [0; 12],
            drive_number: 0x80,
            reserved1: 0,
            boot_signature: 0x29,
            volume_id: 0x12345678,
            volume_label: *b"NO NAME    ",
            fs_type: *b"FAT32   ",
            boot_code: [0; 420],
            signature: 0xAA55,
        };
        
        assert!(validate_boot_sector(&valid_bs));
    }
    
    #[test]
    fn test_is_valid_cluster() {
        assert!(is_valid_cluster(2));
        assert!(is_valid_cluster(100));
        assert!(!is_valid_cluster(0));
        assert!(!is_valid_cluster(1));
        assert!(!is_valid_cluster(0x0FFFFFF8));
    }
}

