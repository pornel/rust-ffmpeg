use ffi::*;
use std::os::raw::c_int;

bitflags! {
    pub struct Flags: c_int {
        const FORCED = AV_SUBTITLE_FLAG_FORCED;
    }
}
