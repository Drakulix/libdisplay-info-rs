use std::ffi::CStr;

mod edid;
pub use self::edid::*;

#[derive(Debug)]
pub struct Info(*mut display_info_sys::di_info);

impl Info {
    pub fn parse(data: impl AsRef<[u8]>) -> Result<Info, ParseError> {
        let data = data.as_ref();
        let info =
            unsafe { display_info_sys::di_info_parse_edid(data.as_ptr() as *const _, data.len()) };

        let error = unsafe { display_info_sys::di_info_get_failure_msg(info) };

        if error.is_null() {
            Ok(Info(info))
        } else {
            Err(ParseError(
                unsafe { CStr::from_ptr(error) }
                    .to_string_lossy()
                    .into_owned(),
            ))
        }
    }

    pub fn serial(&self) -> Option<String> {
        let serial = unsafe { display_info_sys::di_info_get_serial(self.0) };
        if serial.is_null() {
            None
        } else {
            let result = unsafe { CStr::from_ptr(serial).to_string_lossy().into_owned() };
            unsafe {
                libc::free(serial as *mut _);
            }
            Some(result)
        }
    }

    pub fn model(&self) -> Option<String> {
        let model = unsafe { display_info_sys::di_info_get_model(self.0) };
        if model.is_null() {
            None
        } else {
            let result = unsafe { CStr::from_ptr(model).to_string_lossy().into_owned() };
            unsafe {
                libc::free(model as *mut _);
            }
            Some(result)
        }
    }

    pub fn make(&self) -> Option<String> {
        let make = unsafe { display_info_sys::di_info_get_make(self.0) };
        if make.is_null() {
            None
        } else {
            let result = unsafe { CStr::from_ptr(make).to_string_lossy().into_owned() };
            unsafe {
                libc::free(make as *mut _);
            }
            Some(result)
        }
    }

    pub fn edid<'a>(&'a self) -> Option<Edid<'a>> {
        let edid = unsafe { display_info_sys::di_info_get_edid(self.0) };
        if edid.is_null() {
            None
        } else {
            Some(Edid(edid, std::marker::PhantomData))
        }
    }
}

impl Drop for Info {
    fn drop(&mut self) {
        unsafe {
            display_info_sys::di_info_destroy(self.0);
        }
    }
}

#[derive(Debug)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ParseError {}
