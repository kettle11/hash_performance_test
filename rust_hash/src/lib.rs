use core::cell::RefCell;
use std::io::Write;

thread_local! {
    /// Data sent from the host.
    /// Unique to this Wasm thread.
    pub static DATA_FROM_HOST: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

/// Called by the host to reserve scratch space to pass data into kwasm.
/// returns a pointer to the allocated data.
#[no_mangle]
pub extern "C" fn reserve_space(space: usize) -> *mut u8 {
    DATA_FROM_HOST.with(|d| {
        let mut d = d.borrow_mut();
        d.clear();
        d.reserve(space);
        // This was a few ms faster than d.resize(space, 0);
        unsafe { d.set_len(space) };
        d.as_mut_ptr()
    })
}

#[no_mangle]
pub extern "C" fn hash_data() {
    DATA_FROM_HOST.with(|d| {
        let mut d = d.borrow_mut();
        let result = xxhash_rust::xxh3::xxh3_128(&d);
        d.clear();
        d.write(&result.to_be_bytes()).unwrap();
    })
}
