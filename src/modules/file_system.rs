use psp::{sys::*, dprintln};
use core::ffi::c_void;

static PATH_TO_FILE: &str = "./files/example.json";

/// Operations over files from file system
pub unsafe fn file_sys() {
    // Basic configuration
    let file_ptr = self::PATH_TO_FILE.as_ptr();
    let fd = sceIoOpen(file_ptr, IoOpenFlags::RD_WR, 0777);

    // Read file data
    let mut file_buff: [u8; 19] = [0; 19];
    let rd_op = sceIoRead(fd, file_buff.as_mut_ptr() as *mut c_void, 1500);

    // Write datas to file
    let to_write = r##"{ "output_text": "output_test" }"##;
    let wr_op = sceIoWrite(fd, to_write.as_ptr() as *const c_void, to_write.len());

    // Close file descriptor
    sceIoClose(fd);

    // Pript results of all stuff
    psp::dprintln!("Readed {rd_op:} bytes from file\nWrited: {wr_op:} bytes to file");
}

static PATH_TO_DIR: &str = "./files";

/// Operations over directories from file system
pub unsafe fn dir_sys() {
    // Basic configuration
    let dir_ptr = self::PATH_TO_DIR.as_ptr();
    let dird = sceIoDopen(dir_ptr);

    // Read directory
    let mut dir_buff = SceIoDirent { 
        d_stat: SceIoStat {
            st_mode: IoStatMode::IFDIR,
            st_attr: IoStatAttr::IFDIR,
            st_size: 0,
            st_atime: ScePspDateTime::default(),
            st_ctime: ScePspDateTime::default(),
            st_mtime: ScePspDateTime::default(),
            st_private: [0; 6]
        },
        d_name: [0; 256],
        d_private: "".as_ptr() as *mut c_void,
        dummy: 0
    };
    match sceIoDread(dird, &mut dir_buff as *mut SceIoDirent) {
        i if i == 0 => dprintln!("Readed directory"),
        _ => dprintln!("Error while read directory")
    };

    // Create new directory
    match sceIoMkdir("./files/new_directory".as_ptr(), 0777) {
        i if i == 0 => dprintln!(""),
        i if i < 0 => dprintln!("Could not created a new directory"),
        _ => ()
    }

    // Close directory descriptor after all
    sceIoDclose(dird);
}
