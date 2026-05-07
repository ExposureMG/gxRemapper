use std::slice;

#[no_mangle]
pub extern "C" fn gx_remap_is_bad(spare_ptr: *const u8, is_large_block: bool) -> bool {
    if spare_ptr.is_null() {
        return false;
    }

    let spare = unsafe { slice::from_raw_parts(spare_ptr, 16) };
    
    if is_large_block {
        // Large block: Bad block marker at offset 0
        spare[0] != 0xFF
    } else {
        // Small block: Bad block marker at offset 5
        spare[5] != 0xFF
    }
}

#[no_mangle]
pub extern "C" fn gx_remap_find_reserve(
    current_block: u32,
    reserve_start: u32,
    total_blocks: u32,
    is_large_block: bool,
    read_block_callback: extern "C" fn(u32, *mut u8) -> i32,
) -> u32 {
    let mut spare = [0u8; 16];
    
    // We search for a reserve block that is NOT bad
    for block in reserve_start..total_blocks {
        if read_block_callback(block, spare.as_mut_ptr()) == 0 {
            if !gx_remap_is_bad(spare.as_ptr(), is_large_block) {
                return block;
            }
        }
    }
    
    0 // Return 0 to indicate no reserve block found
}
