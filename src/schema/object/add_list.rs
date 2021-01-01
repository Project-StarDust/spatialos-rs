use spatialos_sys::{
    Schema_AddBoolList, Schema_AddDoubleList, Schema_AddEntityIdList, Schema_AddEnumList,
    Schema_AddFixed32List, Schema_AddFixed64List, Schema_AddFloatList, Schema_AddInt32List,
    Schema_AddInt64List, Schema_AddSfixed32List, Schema_AddSfixed64List, Schema_AddSint32List,
    Schema_AddSint64List, Schema_AddUint32List, Schema_AddUint64List, Schema_Object,
};

use super::Object;
use crate::schema::{EntityId, FieldId};

impl Object {
    pub fn add_entity_id_list(&mut self, field_id: FieldId, values: &[EntityId]) {
        unsafe {
            Schema_AddEntityIdList(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }
    pub fn add_bool_list(&mut self, field_id: FieldId, values: &[bool]) {
        unsafe {
            Schema_AddBoolList(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.iter().map(|c| *c as u8).collect::<Vec<_>>().as_ptr(),
                values.len() as u32,
            )
        }
    }
    pub fn add_float_list(&mut self, field_id: FieldId, values: &[f32]) {
        unsafe {
            Schema_AddFloatList(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }
    pub fn add_double_list(&mut self, field_id: FieldId, values: &[f64]) {
        unsafe {
            Schema_AddDoubleList(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_int32_list(&mut self, field_id: FieldId, values: &[i32]) {
        unsafe {
            Schema_AddInt32List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_int64_list(&mut self, field_id: FieldId, values: &[i64]) {
        unsafe {
            Schema_AddInt64List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_uint32_list(&mut self, field_id: FieldId, values: &[u32]) {
        unsafe {
            Schema_AddUint32List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_uint64_list(&mut self, field_id: FieldId, values: &[u64]) {
        unsafe {
            Schema_AddUint64List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_sint32_list(&mut self, field_id: FieldId, values: &[i32]) {
        unsafe {
            Schema_AddSint32List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_sint64_list(&mut self, field_id: FieldId, values: &[i64]) {
        unsafe {
            Schema_AddSint64List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_fixed32_list(&mut self, field_id: FieldId, values: &[u32]) {
        unsafe {
            Schema_AddFixed32List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_fixed64_list(&mut self, field_id: FieldId, values: &[u64]) {
        unsafe {
            Schema_AddFixed64List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_sfixed32_list(&mut self, field_id: FieldId, values: &[i32]) {
        unsafe {
            Schema_AddSfixed32List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }

    pub fn add_sfixed64_list(&mut self, field_id: FieldId, values: &[i64]) {
        unsafe {
            Schema_AddSfixed64List(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                values.as_ptr(),
                values.len() as u32,
            )
        }
    }
    pub fn add_bytes_list(&mut self, field_id: FieldId, values: &[&[u8]]) {
        for value in values {
            self.add_bytes(field_id, value)
        }
    }

    pub fn add_string_list(&mut self, field_id: FieldId, values: &[String]) {
        for value in values {
            self.add_string(field_id, value)
        }
    }

    pub fn add_enum_list<E>(&mut self, field_id: FieldId, values: &[E])
    where
        for<'a> &'a E: Into<u32>,
    {
        unsafe {
            Schema_AddEnumList(
                &mut *self.inner,
                field_id,
                values
                    .iter()
                    .map(|e| e.into())
                    .collect::<Vec<u32>>()
                    .as_ptr(),
                values.len() as u32,
            )
        }
    }
}
