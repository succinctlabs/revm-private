extern "C" {
    pub fn syscall_bn254_add(p: *mut u32, q: *const u32);
}

#[inline]
pub fn bn254_add(p: &mut [u32; 16], q: &[u32; 16]) {
    unsafe { syscall_bn254_add(p.as_mut_ptr(), q.as_ptr()) }
}

// Convert a u8 array to a u32 array
pub fn u8_to_u32(arr: &[u8; 32]) -> [u32; 8] {
    let mut res = [0u32; 8];
    for i in 0..8 {
        res[i] = u32::from_le_bytes(arr[i * 4..(i + 1) * 4].try_into().unwrap());
    }
    res
}