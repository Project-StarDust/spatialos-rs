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
    /// Component ID that this vtable is for. If this is the default vtable, this field is ignored.
    pub component_id: ComponentId,
    /// User data which will be passed directly to the callbacks supplied below.
    pub user_data: *mut c_void,
    /// The function pointers below are only necessary in order to use the user_handle fields present
    /// in each of the Worker_CommandRequest, Worker_CommandResponse, Worker_ComponentData and
    /// Worker_ComponentUpdate types, for the given component ID (or for all components without an
    /// explicit vtable, if this is the default vtable), in order to offload serialization and
    /// deserialization work to internal SDK threads.
    ///
    /// For simplest usage of the SDK, all function pointers can be set to NULL, and only the
    /// schema_type field should be used in each type.
    ///
    /// In order to support usage of the user_handle field on instances of the corresponding type when
    /// used as input data to the SDK, X_serialize() must be provided.
    ///
    /// In order to support usage of the user_handle field on instances of the corresponding type when
    /// received as output data to the SDK, X_deserialize() must be provided.
    ///
    /// X_free() should free resources associated with the result of calling X_deserialize() or
    /// X_copy() (if provided).
    ///
    /// This decision can be made on a per-component, per-handle-type, and per-direction (input or
    /// output) basis. In the case of providing data to the SDK, the asynchronous serialization flow
    /// can be disabled even on a per-call basis by providing a non-NULL schema_type pointer instead of
    /// a user_handle pointer. The concrete types pointed to by the user_handle fields may differ
    /// between components or between handle types.
    ///
    /// All of the functions below, if provided, will be called from arbitrary internal SDK threads,
    /// and therefore must be thread-safe. A single user_handle pointer will not be passed to multiple
    /// callbacks concurrently, but a user_handle may be copied twice and the _results_ of those copies
    /// may be used concurrently.
    ///
    /// For a concrete example, consider calling Worker_Connection_SendComponentUpdate() with
    /// short-circuiting enabled. The SDK will call component_update_copy() twice on the provided
    /// user_handle. One copy will be used for the outgoing flow, and will be serialized with
    /// component_update_serialize() and subsequently freed with component_update_free(). Concurrently,
    /// the other copy will be passed back to the user as part of a Worker_OpList and freed with
    /// component_update_free() when the OpList is deallocated (or, if its lifetime is extended with
    /// Worker_AcquireComponentUpdate(), when the last reference is released by the user with
    /// Worker_ReleaseComponentUpdate()).
    ///
    /// In general, the two most obvious strategies are:
    /// 1) reference-counting. Have X_copy() (atomically) increase a reference count and return the
    ///    same pointer it was given, have X_free() (atomically) decrease the reference count and
    ///    deallocate if zero. X_deserialize() should allocate a new object with reference count of 1,
    ///    set the reference count of any new handle passed into the SDK to 1 initially and call
    ///    X_free() manually afterwards. In this case, data owned by the user_handle should never be
    ///    mutated after its first use. (This is the approach used internally for the schema_type.)
    /// 2) deep-copying. Have X_copy() allocate an entirely new deep copy of the object, and X_free()
    ///    deallocate directly. In this case, user_handles can be mutated freely.
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
