use spatialos_sys::{
    Schema_IndexBool, Schema_IndexBytes, Schema_IndexBytesLength, Schema_IndexDouble,
    Schema_IndexEntityId, Schema_IndexEnum, Schema_IndexFixed32, Schema_IndexFixed64,
    Schema_IndexFloat, Schema_IndexInt32, Schema_IndexInt64, Schema_IndexObject,
    Schema_IndexSfixed32, Schema_IndexSfixed64, Schema_IndexSint32, Schema_IndexSint64,
    Schema_IndexUint32, Schema_IndexUint64, Schema_Object,
};

use crate::{
    const_to_vector,
    schema::{EntityId, FieldId},
};

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

    pub fn index_bool(&self, field_id: FieldId, index: u32) -> bool {
        unsafe { Schema_IndexBool(&*self.inner as *const Schema_Object, field_id, index) == 1 }
    }

    pub fn index_float(&self, field_id: FieldId, index: u32) -> f32 {
        unsafe { Schema_IndexFloat(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_double(&self, field_id: FieldId, index: u32) -> f64 {
        unsafe { Schema_IndexDouble(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_int32(&self, field_id: FieldId, index: u32) -> i32 {
        unsafe { Schema_IndexInt32(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_int64(&self, field_id: FieldId, index: u32) -> i64 {
        unsafe { Schema_IndexInt64(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_uint32(&self, field_id: FieldId, index: u32) -> u32 {
        unsafe { Schema_IndexUint32(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_uint64(&self, field_id: FieldId, index: u32) -> u64 {
        unsafe { Schema_IndexUint64(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_fixed32(&self, field_id: FieldId, index: u32) -> u32 {
        unsafe { Schema_IndexFixed32(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_fixed64(&self, field_id: FieldId, index: u32) -> u64 {
        unsafe { Schema_IndexFixed64(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_sfixed32(&self, field_id: FieldId, index: u32) -> i32 {
        unsafe { Schema_IndexSfixed32(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_sfixed64(&self, field_id: FieldId, index: u32) -> i64 {
        unsafe { Schema_IndexSfixed64(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_sint32(&self, field_id: FieldId, index: u32) -> i32 {
        unsafe { Schema_IndexSint32(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_sint64(&self, field_id: FieldId, index: u32) -> i64 {
        unsafe { Schema_IndexSint64(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_entity_id(&self, field_id: FieldId, index: u32) -> EntityId {
        unsafe { Schema_IndexEntityId(&*self.inner as *const Schema_Object, field_id, index) }
    }

    pub fn index_enum<E: From<u32>>(&self, field_id: FieldId, index: u32) -> E {
        E::from(unsafe { Schema_IndexEnum(&*self.inner as *const Schema_Object, field_id, index) })
    }
}
