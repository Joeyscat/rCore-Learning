const FS_STDOUT: usize = 1;

pub fn sys_write(fs: usize, buf: *const u8, len: usize) -> isize {
    match fs {
        FS_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fs in sys_write!")
        }
    }
}
