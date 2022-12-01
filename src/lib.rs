#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{c_void, CString};
    use super::*;

    #[test]
    fn sanity_set_get_options() {
        let mut instance = unsafe { lidarCreate() };

        let value = "/dev/ydlidar";
        let ok = unsafe {
            setlidaropt(
                instance,
                LidarProperty_LidarPropSerialPort.try_into().unwrap(),
                value.as_ptr() as *const c_void,
                value.len().try_into().unwrap(),
            )
        };
        assert!(ok);

        let mut value_buffer = [0u8; 12];
        let ok = unsafe {
            getlidaropt(
                instance,
                LidarProperty_LidarPropSerialPort.try_into().unwrap(),
                value_buffer.as_mut_ptr() as *mut c_void,
                12)
        };
        assert!(ok);

        let received_value = CString::new(value_buffer).unwrap().into_string().unwrap();

        assert_eq!(value, received_value);

        unsafe { lidarDestroy(&mut instance) };
    }
}
