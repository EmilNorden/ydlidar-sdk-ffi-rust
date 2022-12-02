#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for LaserConfig {
    fn default() -> Self {
        Self {
            min_angle: 0.0,
            max_angle:0.0,
            angle_increment: 0.0,
            time_increment: 0.0,
            max_range: 0.0,
            min_range: 0.0,
            scan_time: 0.0,
        }
    }
}

impl Default for LaserFan {
    fn default() -> Self {
        Self {
            config: LaserConfig::default(),
            npoints: 0,
            points: std::ptr::null_mut(),
            stamp: 0,
        }
    }
}

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
