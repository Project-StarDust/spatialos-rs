use spatialos_sys::*;

use super::Object;
use crate::const_to_vector;
use crate::schema::{EntityId, FieldId};

impl Object {
    pub fn get_bool(&self, field_id: FieldId) -> bool {
        let result = unsafe { Schema_GetBool(&*self.inner as *const Schema_Object, field_id) };
        result > 0
    }

    pub fn get_double(&self, field_id: FieldId) -> f64 {
        unsafe { Schema_GetDouble(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_entity_id(&self, field_id: FieldId) -> EntityId {
        unsafe { Schema_GetEntityId(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_fixed32(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetFixed32(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_fixed64(&self, field_id: FieldId) -> u64 {
        unsafe { Schema_GetFixed64(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_float(&self, field_id: FieldId) -> f32 {
        unsafe { Schema_GetFloat(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_int32(&self, field_id: FieldId) -> i32 {
        unsafe { Schema_GetInt32(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_int64(&self, field_id: FieldId) -> i64 {
        unsafe { Schema_GetInt64(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_sfixed32(&self, field_id: FieldId) -> i32 {
        unsafe { Schema_GetSfixed32(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_sfixed64(&self, field_id: FieldId) -> i64 {
        unsafe { Schema_GetSfixed64(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_sint32(&self, field_id: FieldId) -> i32 {
        unsafe { Schema_GetSint32(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_sint64(&self, field_id: FieldId) -> i64 {
        unsafe { Schema_GetSint64(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_uint32(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetUint32(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_uint64(&self, field_id: FieldId) -> u64 {
        unsafe { Schema_GetUint64(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_object(&mut self, field_id: FieldId) -> Self {
        let inner = unsafe {
            Box::from_raw(Schema_GetObject(
                &mut *self.inner as *mut Schema_Object,
                field_id,
            ))
        };
        Self { inner }
    }

    pub fn get_bytes(&self, field_id: FieldId) -> Vec<u8> {
        let length = self.get_bytes_length(field_id);
        let bytes = unsafe { Schema_GetBytes(&*self.inner as *const Schema_Object, field_id) };
        const_to_vector(bytes, length as isize)
    }

    pub fn get_string(&self, field_id: FieldId) -> String {
        std::str::from_utf8(&self.get_bytes(field_id))
            .unwrap()
            .to_owned()
    }

    pub fn get_enum<E: From<u32>>(&self, field_id: FieldId) -> E {
        E::from(unsafe { Schema_GetEnum(&*self.inner as *const Schema_Object, field_id) })
    }
}
