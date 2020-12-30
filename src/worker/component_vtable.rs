use spatialos_sys::*;
use crate::worker::CommandIndex;
use crate::worker::CommandRequestHandle;
use crate::worker::CommandResponseHandle;
use crate::worker::ComponentDataHandle;
use crate::worker::ComponentId;
use crate::worker::ComponentUpdateHandle;
use std::os::raw::c_void;

pub type CommandRequestFreeFn =
    unsafe extern "C" fn(ComponentId, CommandIndex, *mut c_void, *mut Worker_CommandRequestHandle);

pub type CommandRequestCopyFn = unsafe extern "C" fn(
    ComponentId,
    CommandIndex,
    *mut c_void,
    *mut CommandRequestHandle,
) -> *mut CommandRequestHandle;

pub type CommandRequestDeserialize = unsafe extern "C" fn(
    ComponentId,
    CommandIndex,
    *mut c_void,
    *mut Schema_CommandRequest,
    *mut *mut CommandRequestHandle,
) -> u8;

pub type CommandRequestSerialize = unsafe extern "C" fn(
    ComponentId,
    CommandIndex,
    *mut c_void,
    *mut CommandRequestHandle,
    *mut *mut Schema_CommandRequest,
);

pub type CommandResponseFree =
    unsafe extern "C" fn(ComponentId, CommandIndex, *mut c_void, *mut CommandResponseHandle);

pub type CommandResponseCopy = unsafe extern "C" fn(
    ComponentId,
    CommandIndex,
    *mut c_void,
    *mut CommandResponseHandle,
) -> *mut CommandResponseHandle;

pub type CommandResponseDeserialize = unsafe extern "C" fn(
    ComponentId,
    CommandIndex,
    *mut c_void,
    *mut Schema_CommandResponse,
    *mut *mut CommandResponseHandle,
) -> u8;

pub type CommandResponseSerialize = unsafe extern "C" fn(
    ComponentId,
    CommandIndex,
    *mut c_void,
    *mut CommandResponseHandle,
    *mut *mut Schema_CommandResponse,
);

pub type ComponentDataFree =
    unsafe extern "C" fn(ComponentId, *mut c_void, *mut ComponentDataHandle);

pub type ComponentDataCopy = unsafe extern "C" fn(
    ComponentId,
    *mut c_void,
    *mut ComponentDataHandle,
) -> *mut ComponentDataHandle;

pub type ComponentDataDeserialize = unsafe extern "C" fn(
    ComponentId,
    *mut c_void,
    *mut Schema_ComponentData,
    *mut *mut ComponentDataHandle,
) -> u8;

pub type ComponentDataSerialize = unsafe extern "C" fn(
    ComponentId,
    *mut c_void,
    *mut ComponentDataHandle,
    *mut *mut Schema_ComponentData,
);

pub type ComponentUpdateFree =
    unsafe extern "C" fn(ComponentId, *mut c_void, *mut ComponentUpdateHandle);

pub type ComponentUpdateCopy = unsafe extern "C" fn(
    ComponentId,
    *mut c_void,
    *mut ComponentUpdateHandle,
) -> *mut ComponentUpdateHandle;

pub type ComponentUpdateDeserialize = unsafe extern "C" fn(
    ComponentId,
    *mut c_void,
    *mut Schema_ComponentUpdate,
    *mut *mut ComponentUpdateHandle,
) -> u8;

pub type ComponentUpdateSerialize = unsafe extern "C" fn(
    ComponentId,
    *mut c_void,
    *mut ComponentUpdateHandle,
    *mut *mut Schema_ComponentUpdate,
);

pub struct ComponentVtable {
    #[doc = " Component ID that this vtable is for. If this is the default vtable, this field is ignored."]
    pub component_id: ComponentId,
    #[doc = " User data which will be passed directly to the callbacks supplied below."]
    pub user_data: *mut c_void,
    #[doc = " The function pointers below are only necessary in order to use the user_handle fields present"]
    #[doc = " in each of the Worker_CommandRequest, Worker_CommandResponse, Worker_ComponentData and"]
    #[doc = " Worker_ComponentUpdate types, for the given component ID (or for all components without an"]
    #[doc = " explicit vtable, if this is the default vtable), in order to offload serialization and"]
    #[doc = " deserialization work to internal SDK threads."]
    #[doc = ""]
    #[doc = " For simplest usage of the SDK, all function pointers can be set to NULL, and only the"]
    #[doc = " schema_type field should be used in each type."]
    #[doc = ""]
    #[doc = " In order to support usage of the user_handle field on instances of the corresponding type when"]
    #[doc = " used as input data to the SDK, X_serialize() must be provided."]
    #[doc = ""]
    #[doc = " In order to support usage of the user_handle field on instances of the corresponding type when"]
    #[doc = " received as output data to the SDK, X_deserialize() must be provided."]
    #[doc = ""]
    #[doc = " X_free() should free resources associated with the result of calling X_deserialize() or"]
    #[doc = " X_copy() (if provided)."]
    #[doc = ""]
    #[doc = " This decision can be made on a per-component, per-handle-type, and per-direction (input or"]
    #[doc = " output) basis. In the case of providing data to the SDK, the asynchronous serialization flow"]
    #[doc = " can be disabled even on a per-call basis by providing a non-NULL schema_type pointer instead of"]
    #[doc = " a user_handle pointer. The concrete types pointed to by the user_handle fields may differ"]
    #[doc = " between components or between handle types."]
    #[doc = ""]
    #[doc = " All of the functions below, if provided, will be called from arbitrary internal SDK threads,"]
    #[doc = " and therefore must be thread-safe. A single user_handle pointer will not be passed to multiple"]
    #[doc = " callbacks concurrently, but a user_handle may be copied twice and the _results_ of those copies"]
    #[doc = " may be used concurrently."]
    #[doc = ""]
    #[doc = " For a concrete example, consider calling Worker_Connection_SendComponentUpdate() with"]
    #[doc = " short-circuiting enabled. The SDK will call component_update_copy() twice on the provided"]
    #[doc = " user_handle. One copy will be used for the outgoing flow, and will be serialized with"]
    #[doc = " component_update_serialize() and subsequently freed with component_update_free(). Concurrently,"]
    #[doc = " the other copy will be passed back to the user as part of a Worker_OpList and freed with"]
    #[doc = " component_update_free() when the OpList is deallocated (or, if its lifetime is extended with"]
    #[doc = " Worker_AcquireComponentUpdate(), when the last reference is released by the user with"]
    #[doc = " Worker_ReleaseComponentUpdate())."]
    #[doc = ""]
    #[doc = " In general, the two most obvious strategies are:"]
    #[doc = " 1) reference-counting. Have X_copy() (atomically) increase a reference count and return the"]
    #[doc = "    same pointer it was given, have X_free() (atomically) decrease the reference count and"]
    #[doc = "    deallocate if zero. X_deserialize() should allocate a new object with reference count of 1,"]
    #[doc = "    set the reference count of any new handle passed into the SDK to 1 initially and call"]
    #[doc = "    X_free() manually afterwards. In this case, data owned by the user_handle should never be"]
    #[doc = "    mutated after its first use. (This is the approach used internally for the schema_type.)"]
    #[doc = " 2) deep-copying. Have X_copy() allocate an entirely new deep copy of the object, and X_free()"]
    #[doc = "    deallocate directly. In this case, user_handles can be mutated freely."]
    pub command_request_free: Option<CommandRequestFreeFn>,
    pub command_request_copy: Option<CommandRequestCopyFn>,
    pub command_request_deserialize: Option<CommandRequestDeserialize>,
    pub command_request_serialize: Option<CommandRequestSerialize>,
    pub command_response_free: Option<CommandResponseFree>,
    pub command_response_copy: Option<CommandResponseCopy>,
    pub command_response_deserialize: Option<CommandResponseDeserialize>,
    pub command_response_serialize: Option<CommandResponseSerialize>,
    pub component_data_free: Option<ComponentDataFree>,
    pub component_data_copy: Option<ComponentDataCopy>,
    pub component_data_deserialize: Option<ComponentDataDeserialize>,
    pub component_data_serialize: Option<ComponentDataSerialize>,
    pub component_update_free: Option<ComponentUpdateFree>,
    pub component_update_copy: Option<ComponentUpdateCopy>,
    pub component_update_deserialize: Option<ComponentUpdateDeserialize>,
    pub component_update_serialize: Option<ComponentUpdateSerialize>,
}

impl From<ComponentVtable> for Worker_ComponentVtable {
    fn from(vtable: ComponentVtable) -> Self {
        Self {
            component_id: vtable.component_id,
            user_data: vtable.user_data,
            command_request_free: vtable.command_request_free,
            command_request_copy: vtable.command_request_copy,
            command_request_deserialize: vtable.command_request_deserialize,
            command_request_serialize: vtable.command_request_serialize,
            command_response_free: vtable.command_response_free,
            command_response_copy: vtable.command_response_copy,
            command_response_deserialize: vtable.command_response_deserialize,
            command_response_serialize: vtable.command_response_serialize,
            component_data_free: vtable.component_data_free,
            component_data_copy: vtable.component_data_copy,
            component_data_deserialize: vtable.component_data_deserialize,
            component_data_serialize: vtable.component_data_serialize,
            component_update_free: vtable.component_update_free,
            component_update_copy: vtable.component_update_copy,
            component_update_deserialize: vtable.component_update_deserialize,
            component_update_serialize: vtable.component_update_serialize,
        }
    }
}

impl From<Worker_ComponentVtable> for ComponentVtable {
    fn from(vtable: Worker_ComponentVtable) -> Self {
        Self {
            component_id: vtable.component_id,
            user_data: vtable.user_data,
            command_request_free: vtable.command_request_free,
            command_request_copy: vtable.command_request_copy,
            command_request_deserialize: vtable.command_request_deserialize,
            command_request_serialize: vtable.command_request_serialize,
            command_response_free: vtable.command_response_free,
            command_response_copy: vtable.command_response_copy,
            command_response_deserialize: vtable.command_response_deserialize,
            command_response_serialize: vtable.command_response_serialize,
            component_data_free: vtable.component_data_free,
            component_data_copy: vtable.component_data_copy,
            component_data_deserialize: vtable.component_data_deserialize,
            component_data_serialize: vtable.component_data_serialize,
            component_update_free: vtable.component_update_free,
            component_update_copy: vtable.component_update_copy,
            component_update_deserialize: vtable.component_update_deserialize,
            component_update_serialize: vtable.component_update_serialize,
        }
    }
}
