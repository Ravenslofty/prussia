use core::str;

/// A ROMDIR entry.
#[repr(packed)]
pub struct Entry {
    name: [u8; 10],
    _something: u16,
    size: u32,
}

const START_OF_ROM: usize = 0x1fc0_0000;
const END_OF_ROM: usize = 0x2000_0000;

/// Search for an entry in the ROM contents directory, returning a pointer to it if it exists.
pub fn lookup(name: &str) -> Option<usize> {
    let mut entry_addr: usize = START_OF_ROM;

    // Search for the start of the ROM directory.
    while entry_addr < END_OF_ROM {
        let entry_ptr = entry_addr as *const Entry;
        let entry_name = unsafe { &(*entry_ptr).name };
        match str::from_utf8(entry_name).ok() {
            // The start of the ROM directory is marked by the "RESET" entry.
            Some(s) => {
                if s == "RESET" {
                    break;
                } else {
                    entry_addr += 1;
                }
            }
            // Failure to decode could be due to pointing at garbage.
            None => entry_addr += 1,
        }
    }

    // If we couldn't find the "RESET" entry, then this BIOS is very broken.
    if entry_addr == END_OF_ROM {
        panic!("Couldn't find start of ROM directory. This indicates a broken ROM build.");
    }

    // Check pointer alignment; MIPS requires 4 byte alignment for Entry::size.
    if entry_addr % 4 != 0 {
        panic!(
            "Start of ROM directory is not correctly aligned. This indicates a broken ROM build."
        );
    }

    // Get the start of the entry, if it exists.
    let mut start_addr = START_OF_ROM;

    // Search for the matching ROM directory entry.
    while entry_addr < END_OF_ROM && start_addr < END_OF_ROM {
        let entry_ptr = entry_addr as *const Entry;
        let entry_name = unsafe { &(*entry_ptr).name };
        let entry_size = unsafe { (*entry_ptr).size };
        match str::from_utf8(entry_name).ok() {
            Some(s) => {
                if s == name {
                    return Some(start_addr);
                } else {
                    entry_addr += 16;
                    start_addr += entry_size as usize;
                }
            }
            // The name should always be a valid ASCII (and thus UTF-8) string.
            None => panic!(
                "Failed to decode ROMDIR entry name at {}. This indicates a broken ROM build.",
                entry_addr
            ),
        }
    }

    // If we didn't find it in the ROM directory (which is probably a bug), return none.
    None
}
