use spatialos_sys::*;

use super::Object;
use crate::schema::FieldId;

impl Object {
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
