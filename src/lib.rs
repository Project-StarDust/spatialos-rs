pub mod schema;
pub mod worker;

pub mod private_exports {
    pub use spatialos_sys::*;
}

pub fn mut_to_vector<T>(data: *mut T, size: isize) -> Vec<T> {
    if data.is_null() {
        Vec::new()
    } else {
        let mut datas = Vec::new();
        for index in 0..size {
            let data = unsafe {
                let data_ptr = data.offset(index);
                Box::from_raw(data_ptr)
            };
            datas.push(*data);
        }
        datas
    }
}

pub(crate) fn const_to_vector<T: Clone>(data: *const T, size: isize) -> Vec<T> {
    if data.is_null() {
        Vec::new()
    } else {
        let mut datas = Vec::new();
        for index in 0..size {
            let data = unsafe {
                let data_ptr = data.offset(index);
                (*data_ptr).clone()
            };
            datas.push(data);
        }
        datas
    }
}
