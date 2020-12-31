use spatialos_sys::{Schema_AllocateBuffer, Schema_GetBytesLength, Schema_Object};

use crate::schema::FieldId;
use bytevec::ByteEncodable;

mod add;
mod add_list;
mod count;
mod get;
mod get_list;
mod index;

pub struct AllocatedBuffer {
    pub inner: *mut u8,
}

impl From<*mut u8> for AllocatedBuffer {
    fn from(data: *mut u8) -> Self {
        Self { inner: data }
    }
}

pub struct Object {
    inner: Box<Schema_Object>,
}

impl From<*mut Schema_Object> for Object {
    fn from(inner: *mut Schema_Object) -> Self {
        let inner = unsafe { Box::from_raw(inner) };
        Self { inner }
    }
}

impl Object {
    pub fn allocate_buffer<T: ByteEncodable>(&mut self, data: &[T]) -> AllocatedBuffer {
        let src = data.encode::<u8>().unwrap();
        let buffer = unsafe {
            let buffer = Schema_AllocateBuffer(
                &mut *self.inner as *mut Schema_Object,
                std::mem::size_of::<T>() as u32 * data.len() as u32,
            );
            std::ptr::copy_nonoverlapping(src.as_ptr(), buffer, data.len());
            buffer
        };
        AllocatedBuffer::from(buffer)
    }
    pub fn get_bytes_length(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetBytesLength(&*self.inner as *const Schema_Object, field_id) }
    }
}
