use spatialos_sys::{
    Schema_GetBoolCount, Schema_GetBytesCount, Schema_GetDoubleCount, Schema_GetEntityIdCount,
    Schema_GetEnumCount, Schema_GetFixed32Count, Schema_GetFixed64Count, Schema_GetFloatCount,
    Schema_GetInt32Count, Schema_GetInt64Count, Schema_GetObjectCount, Schema_GetSfixed32Count,
    Schema_GetSfixed64Count, Schema_GetSint32Count, Schema_GetSint64Count, Schema_GetUint32Count,
    Schema_GetUint64Count, Schema_Object,
};

use super::Object;
use crate::schema::FieldId;

impl Object {
    pub fn get_sint64_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetSint64Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_sint32_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetSint32Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_sfixed64_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetSfixed64Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_sfixed32_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetSfixed32Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_int32_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetInt32Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_fixed64_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetFixed64Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_fixed32_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetFixed32Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_entity_id_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetEntityIdCount(&*self.inner as *const Schema_Object, field_id) }
    }
    pub fn get_bool_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetBoolCount(&*self.inner as *const Schema_Object, field_id) }
    }
    pub fn get_double_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetDoubleCount(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_float_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetFloatCount(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_uint32_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetUint32Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_uint64_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetUint64Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_int64_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetInt64Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_object_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetObjectCount(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_bytes_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_enum_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetEnumCount(&*self.inner as *const Schema_Object, field_id) }
    }
}
