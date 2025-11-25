#![no_std]
#![no_main]

use core::panic::PanicInfo;

// modules du parser FAT32
pub mod boot_sector;

// boucle pour les alertes no_std.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}