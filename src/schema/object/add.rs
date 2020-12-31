use spatialos_sys::{
    Schema_AddBool, Schema_AddBytes, Schema_AddDouble, Schema_AddEntityId, Schema_AddEnum,
    Schema_AddFixed32, Schema_AddFixed64, Schema_AddFloat, Schema_AddInt32, Schema_AddInt64,
    Schema_AddObject, Schema_AddSfixed32, Schema_AddSfixed64, Schema_AddSint32, Schema_AddSint64,
    Schema_AddUint32, Schema_AddUint64, Schema_Object,
};

use super::Object;
use crate::schema::{EntityId, FieldId};
use std::borrow::Borrow;

impl Object {
    pub fn add_bool<D: Borrow<bool>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddBool(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned() as u8,
            )
        }
    }

    pub fn add_double<D: Borrow<f64>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddDouble(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_entity_id<D: Borrow<EntityId>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddEntityId(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_fixed32<D: Borrow<u32>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddFixed32(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_fixed64<D: Borrow<u64>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddFixed64(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }
    pub fn add_float<D: Borrow<f32>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddFloat(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_int32<D: Borrow<i32>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddInt32(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_int64<D: Borrow<i64>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddInt64(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_sfixed32<D: Borrow<i32>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddSfixed32(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_sfixed64<D: Borrow<i64>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddSfixed64(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_sint32<D: Borrow<i32>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddSint32(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_sint64<D: Borrow<i64>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddSint64(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_uint32<D: Borrow<u32>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddUint32(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_uint64<D: Borrow<u64>>(&mut self, field_id: FieldId, value: D) {
        unsafe {
            Schema_AddUint64(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().to_owned(),
            )
        }
    }

    pub fn add_object(&mut self, field_id: FieldId) -> Self {
        let inner = unsafe {
            Box::from_raw(Schema_AddObject(
                &mut *self.inner as *mut Schema_Object,
                field_id,
            ))
        };
        Self { inner }
    }

    pub fn add_bytes(&mut self, field_id: FieldId, values: &[u8]) {
        let buffer = self.allocate_buffer(values);
        unsafe {
            Schema_AddBytes(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                buffer.inner,
                values.len() as u32,
            )
        }
    }

    pub fn add_string<S: AsRef<str>>(&mut self, field_id: FieldId, value: S) {
        self.add_bytes(field_id, value.as_ref().as_bytes())
    }

    pub fn add_enum<E, V>(&mut self, field_id: FieldId, value: E)
    where
        for<'a> &'a V: Into<u32>,
        E: Borrow<V>,
    {
        unsafe {
            Schema_AddEnum(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.borrow().into(),
            )
        }
    }
}
