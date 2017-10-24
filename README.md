## Harfbuzz Rust

Unsafe harfbuzz bindings for rust. Bindings are generated
using bindgen.

### Example

```rust
extern crate harfbuzz_sys;
use std::path::{Path};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::ffi::CStr;

fn main() {
    let path = Path::new("path to font file");
    
    if !path.exists() {
        panic!("Font file not found at path {:?}", path);
    }
    
    let mut file = File::open(path).expect(&format!("Cannot open font file {:?}", path));

    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect(&format!("Cannot read font file {:?}", path));
    let file_size = bytes.len() as u32;

    unsafe {
        let blob = harfbuzz_sys::hb_blob_create(
            CStr::from_bytes_with_nul_unchecked(&bytes).as_ptr(),
            file_size,
            harfbuzz_sys::hb_memory_mode_t::HB_MEMORY_MODE_READONLY,
            ptr::null_mut(),
            None
        );

        let face = harfbuzz_sys::hb_face_create(blob, 0);
        let font = harfbuzz_sys::hb_font_create(face);

        if harfbuzz_sys::hb_ot_math_has_data(face) == 0 {
            println!("This font does not have math constants table");
            return;
        }
        
        println!("{}", font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SCRIPT_PERCENT_SCALE_DOWN);
        
        harfbuzz_sys::hb_blob_destroy(blob);
        harfbuzz_sys::hb_face_destroy(face);
        harfbuzz_sys::hb_font_destroy(font);
    };
}
```

See full example in `examples/ot_math_constants.rs`

### License
Apache License 2.0. See `LICENSE.md`.