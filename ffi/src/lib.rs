mod result;

use bdwgc_alloc::Allocator;
use result::FfiResult;
use std::alloc::Layout;
use std::fs::File;
use std::io::{Read, Write};
use std::os::raw::{c_int, c_void};
use std::os::unix::io::FromRawFd;

extern "C" {
    fn _ein_os_main(
        stack: *mut ffi::cps::Stack,
        continuation: extern "C" fn(*mut ffi::cps::Stack, f64) -> ffi::cps::Result,
        argument: ffi::None,
    ) -> ffi::cps::Result;
}

const DEFAULT_ALIGNMENT: usize = 8;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[no_mangle]
pub extern "C" fn _ein_malloc(size: usize) -> *mut c_void {
    (unsafe { std::alloc::alloc(Layout::from_size_align(size, DEFAULT_ALIGNMENT).unwrap()) })
        as *mut c_void
}

#[no_mangle]
pub extern "C" fn _ein_realloc(pointer: *mut c_void, size: usize) -> *mut c_void {
    // Layouts are ignored by the bdwgc global allocator.
    (unsafe {
        std::alloc::realloc(
            pointer as *mut u8,
            Layout::from_size_align(0, DEFAULT_ALIGNMENT).unwrap(),
            size,
        )
    }) as *mut c_void
}

#[no_mangle]
pub extern "C" fn main() -> c_int {
    unsafe { Allocator::initialize() }

    let mut stack = ffi::cps::Stack::new();

    unsafe { _ein_os_main(&mut stack, exit, ffi::None::new()) };

    unreachable!()
}

extern "C" fn exit(_: *mut ffi::cps::Stack, code: f64) -> ffi::cps::Result {
    std::process::exit(code as i32)
}

#[no_mangle]
extern "C" fn _ein_os_fd_read(
    fd: ffi::Number,
    buffer_size: ffi::Number,
) -> *const FfiResult<ffi::EinString> {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };
    let mut buffer = vec![0; f64::from(buffer_size) as usize];

    let count = match file.read(&mut buffer) {
        Ok(count) => count,
        Err(error) => return FfiResult::from_io_error(error),
    };
    buffer.resize(count, 0);

    std::mem::forget(file);

    FfiResult::ok(ffi::EinString::from(buffer))
}

#[no_mangle]
extern "C" fn _ein_os_fd_write(
    fd: ffi::Number,
    buffer: ffi::EinString,
) -> *const FfiResult<ffi::Number> {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    let byte_count = match file.write(buffer.as_slice()) {
        Ok(count) => count,
        Err(error) => return FfiResult::from_io_error(error),
    };

    std::mem::forget(file);

    FfiResult::ok((byte_count as f64).into())
}

#[no_mangle]
extern "C" fn _ein_os_fd_readdir(
    fd: ffi::Number,
    buffer_size: ffi::Number,
) -> *const FfiResult<ffi::EinString> {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };
    let mut buffer = vec![0; f64::from(buffer_size) as usize];

    let count = match file.read(&mut buffer) {
        Ok(count) => count,
        Err(error) => return FfiResult::from_io_error(error),
    };
    buffer.resize(count, 0);

    std::mem::forget(file);

    FfiResult::ok(ffi::EinString::from(buffer))
}
