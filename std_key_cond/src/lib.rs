use key_base;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
    panic!("Called it!");
}