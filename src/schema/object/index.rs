use spatialos_sys::*;

use crate::{const_to_vector, schema::FieldId};

use super::Object;

impl Object {
    pub fn index_object(&mut self, field_id: FieldId, index: u32) -> Self {
        Self::from(unsafe {
            Schema_IndexObject(&mut *self.inner as *mut Schema_Object, field_id, index)
        })
    }

    pub fn index_bytes_length(&self, field_id: FieldId, index: u32) -> u32 {
        unsafe { Schema_IndexBytesLength(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_bytes(&self, field_id: FieldId, index: u32) -> Vec<u8> {
        let count = self.index_bytes_length(field_id, index);
        let bytes =
            unsafe { Schema_IndexBytes(&*self.inner as *const Schema_Object, field_id, index) };
        const_to_vector(bytes, count as isize)
    }

    pub fn index_string(&self, field_id: FieldId, index: u32) -> String {
        std::str::from_utf8(&self.index_bytes(field_id, index))
            .unwrap()
            .to_owned()
    }

    pub fn index_enum<E: From<u32>>(&self, field_id: FieldId, index: u32) -> E {
        E::from(unsafe { Schema_IndexEnum(&*self.inner as *const Schema_Object, field_id, index) })
    }
}
