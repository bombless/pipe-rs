extern crate winapi;
extern crate kernel32;

use winapi::{HANDLE, DWORD};
use winapi::winnt::{GENERIC_READ, GENERIC_WRITE, FILE_ATTRIBUTE_NORMAL};
use winapi::fileapi::OPEN_ALWAYS;
use std::os::windows::prelude::*;
use std::ffi::OsStr;
use kernel32::{CreateFileW, CloseHandle, ReadFile};

use std::io::{Read, Error};

pub struct File(HANDLE);

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut read: DWORD = 0;
        let buf_len = buf.len() as DWORD;
        if 0 == unsafe {
            ReadFile(self.0,
                buf.as_mut_ptr() as *mut _,
                buf_len,
                &mut read,
                0 as *mut _)
        } {
            unimplemented!()
        } else {
            Ok(read as usize)
        }
    }
}

impl File {
    pub fn open(name: &str) -> Result<File, String> {
        let osstr = OsStr::new(name).encode_wide();
        let osstr = osstr.chain(Some(0).into_iter()).collect::<Vec<_>>();
        let handle = unsafe {
            CreateFileW(osstr.as_ptr(),
                        GENERIC_READ | GENERIC_WRITE,
                        0,
                        0 as *mut _,
                        OPEN_ALWAYS,
                        FILE_ATTRIBUTE_NORMAL,
                        0 as *mut _)
        };
        if handle != 0 as *mut _ {
            Ok(File(handle))
        } else {
            unimplemented!()
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

#[test]
fn test() {
    let file = File::open("src/lib.rs").unwrap();
    let bytes = file.bytes().take(20).map(Result::unwrap).collect::<Vec<_>>();
    assert_eq!(bytes, b"extern crate winapi;")
}
