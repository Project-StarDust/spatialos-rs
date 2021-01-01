use spatialos_sys::{
    Schema_GetBoolList, Schema_GetDoubleList, Schema_GetEntityIdList, Schema_GetEnumList,
    Schema_GetFixed32List, Schema_GetFixed64List, Schema_GetFloatList, Schema_GetInt32List,
    Schema_GetInt64List, Schema_GetSfixed32List, Schema_GetSfixed64List, Schema_GetSint32List,
    Schema_GetSint64List, Schema_GetUint32List, Schema_GetUint64List, Schema_Object,
};

use super::Object;
use crate::schema::{EntityId, FieldId};

impl Object {
    pub fn get_sint64_list(&self, field_id: FieldId) -> Vec<i64> {
        let count = self.get_sint64_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetSint64List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_sint32_list(&self, field_id: FieldId) -> Vec<i32> {
        let count = self.get_sint32_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetSint32List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }
    pub fn get_sfixed64_list(&self, field_id: FieldId) -> Vec<i64> {
        let count = self.get_sfixed64_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetSfixed64List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_sfixed32_list(&self, field_id: FieldId) -> Vec<i32> {
        let count = self.get_sfixed32_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetSfixed32List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_int64_list(&self, field_id: FieldId) -> Vec<i64> {
        let count = self.get_int64_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetInt64List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_int32_list(&self, field_id: FieldId) -> Vec<i32> {
        let count = self.get_int32_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetInt32List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_fixed64_list(&self, field_id: FieldId) -> Vec<u64> {
        let count = self.get_fixed64_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetFixed64List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_fixed32_list(&self, field_id: FieldId) -> Vec<u32> {
        let count = self.get_fixed32_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetFixed32List(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_entity_id_list(&self, field_id: FieldId) -> Vec<EntityId> {
        let count = self.get_entity_id_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetEntityIdList(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

    pub fn get_bool_list(&self, field_id: FieldId) -> Vec<bool> {
        let count = self.get_bool_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetBoolList(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list.into_iter().map(|b| b == 1).collect()
    }

    pub fn get_float_list(&self, field_id: FieldId) -> Vec<f32> {
        let count = self.get_float_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetFloatList(
                &*self.inner as *const Schema_Object,
                field_id,
                list.as_mut_ptr(),
            )
        }
        list
    }

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

    pub fn get_uint64_list(&self, field_id: FieldId) -> Vec<u64> {
        let count = self.get_uint64_count(field_id);
        let mut list = Vec::with_capacity(count as usize);
        unsafe {
            Schema_GetUint64List(
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
