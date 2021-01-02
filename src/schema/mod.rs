use spatialos_sys::{
    Schema_ComponentId, Schema_CreateComponentData, Schema_CreateComponentUpdate, Schema_EntityId,
    Schema_FieldId, Schema_GetComponentDataFields, Schema_GetComponentUpdateFields,
    SCHEMA_MAP_KEY_FIELD_ID, SCHEMA_MAP_VALUE_FIELD_ID,
};

pub mod object;
pub use object::Object;

pub type EntityId = Schema_EntityId;
pub type FieldId = Schema_FieldId;
pub type ComponentId = Schema_ComponentId;

pub mod ffi {

    use ::spatialos_sys::{
        Schema_CommandRequest, Schema_CommandResponse, Schema_ComponentData, Schema_ComponentUpdate,
    };

    pub type CommandRequest = Schema_CommandRequest;
    pub type CommandResponse = Schema_CommandResponse;
    pub type ComponentData = Schema_ComponentData;
    pub type ComponentUpdate = Schema_ComponentUpdate;
}

pub const MAP_KEY_FIELD_ID: u32 = SCHEMA_MAP_KEY_FIELD_ID;
pub const MAP_VALUE_FIELD_ID: u32 = SCHEMA_MAP_VALUE_FIELD_ID;

#[derive(Debug, Clone)]
pub struct ComponentData {
    inner: Box<ffi::ComponentData>,
}

#[derive(Debug, Clone)]
pub struct ComponentUpdate {
    inner: Box<ffi::ComponentUpdate>,
}

impl ComponentData {
    pub fn new() -> Self {
        let inner = unsafe { Box::from_raw(Schema_CreateComponentData()) };
        Self { inner }
    }

    pub fn get_fields(&mut self) -> Object {
        Object::from(unsafe {
            Schema_GetComponentDataFields(&mut *self.inner as *mut ffi::ComponentData)
        })
    }
}

impl ComponentUpdate {
    pub fn new() -> Self {
        let inner = unsafe { Box::from_raw(Schema_CreateComponentUpdate()) };
        Self { inner }
    }

    pub fn get_fields(&mut self) -> Object {
        Object::from(unsafe {
            Schema_GetComponentUpdateFields(&mut *self.inner as *mut ffi::ComponentUpdate)
        })
    }
}

impl From<*mut ffi::ComponentData> for ComponentData {
    fn from(inner: *mut ffi::ComponentData) -> Self {
        let inner = unsafe { Box::from_raw(inner) };
        Self { inner }
    }
}

impl From<ComponentData> for *mut ffi::ComponentData {
    fn from(data: ComponentData) -> Self {
        Box::into_raw(data.inner)
    }
}

impl From<*mut ffi::ComponentUpdate> for ComponentUpdate {
    fn from(inner: *mut ffi::ComponentUpdate) -> Self {
        let inner = unsafe { Box::from_raw(inner) };
        Self { inner }
    }
}

impl Into<*mut ffi::ComponentUpdate> for ComponentUpdate {
    fn into(self) -> *mut ffi::ComponentUpdate {
        Box::into_raw(self.inner)
    }
}
