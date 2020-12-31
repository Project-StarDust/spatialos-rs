use spatialos_sys::{Schema_GetDoubleList, Schema_GetUint32List, Schema_GetEnumList, Schema_Object};

use super::Object;
use crate::schema::FieldId;

impl Object {
    pub fn get_double_list(&self, field_id: FieldId) -> Vec<f64> {
        let count = self.get_double_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetDoubleList(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_uint32_list(&self, field_id: FieldId) -> Vec<u32> {
        let count = self.get_uint32_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetUint32List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_bytes_list(&self, field_id: FieldId) -> Vec<Vec<u8>> {
        let count = self.get_bytes_count(field_id);
        (0..count).map(|i| self.index_bytes(field_id, i)).collect()
    }
    pub fn get_string_list(&self, field_id: FieldId) -> Vec<String> {
        let count = self.get_bytes_count(field_id);
        (0..count).map(|i| self.index_string(field_id, i)).collect()
    }

    pub fn get_optional_double_list(&self, field_id: FieldId) -> Option<Vec<f64>> {
        let count = self.get_double_count(field_id);
        if count > 0 {
            Some(self.get_double_list(field_id))
        } else {
            None
        }
    }

    pub fn get_optional_uint32_list(&self, field_id: FieldId) -> Option<Vec<u32>> {
        let count = self.get_uint32_count(field_id);
        if count > 0 {
            Some(self.get_uint32_list(field_id))
        } else {
            None
        }
    }

    pub fn get_optional_bytes_list(&self, field_id: FieldId) -> Option<Vec<Vec<u8>>> {
        let count = self.get_double_count(field_id);
        if count > 0 {
            Some(self.get_bytes_list(field_id))
        } else {
            None
        }
    }
    pub fn get_optional_string_list(&self, field_id: FieldId) -> Option<Vec<String>> {
        let count = self.get_bytes_count(field_id);
        if count > 0 {
            Some(self.get_string_list(field_id))
        } else {
            None
        }
    }

    pub fn get_enum_list<E: From<u32>>(&self, field_id: FieldId) -> Vec<E> {
        let count = self.get_enum_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetEnumList(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list.into_iter().map(E::from).collect::<Vec<E>>()
    }
}
