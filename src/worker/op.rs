use spatialos_sys::*;

use crate::const_to_vector;
use crate::worker::metrics::Metrics;
use crate::worker::Authority;
use crate::worker::ComponentId;
use crate::worker::ConnectionStatusCode;
use crate::worker::Entity;
use crate::worker::EntityId;
use crate::worker::LogLevel;
use crate::worker::RequestId;
use crate::worker::StatusCode;
use crate::worker::WorkerAttributes;
use std::ffi::CStr;

#[doc = " An op list, usually returned by Connectio::get_op_list."]
pub struct OpList {
    pub ops: Vec<WorkerOp>,
    inner: *mut Worker_OpList,
}

impl From<*mut Worker_OpList> for OpList {
    fn from(op_list: *mut Worker_OpList) -> Self {
        let ops = unsafe {
            let mut ops = Vec::new();
            for index in 0..(*op_list).op_count as isize {
                let op_ptr = (*op_list).ops.offset(index as isize);
                ops.push(WorkerOp::from(*op_ptr));
            }
            ops
        };
        Self {
            ops,
            inner: op_list,
        }
    }
}

impl Drop for OpList {
    fn drop(&mut self) {
        unsafe { Worker_OpList_Destroy(self.inner) }
    }
}

#[derive(Debug)]
#[doc = " Data for a single op contained within an op list."]
pub enum WorkerOp {
    Disconnect(DisconnectOp),
    FlagUpdate(FlagUpdateOp),
    LogMessage(LogMessageOp),
    Metrics(MetricsOp),
    CriticalSection(CriticalSectionOp),
    AddEntity(AddEntityOp),
    RemoveEntity(RemoveEntityOp),
    ReserveEntityIdsResponse(ReserveEntityIdsResponseOp),
    CreateEntityResponse(CreateEntityResponseOp),
    DeleteEntityResponse(DeleteEntityResponseOp),
    EntityQueryResponse(EntityQueryResponseOp),
    AddComponent(AddComponentOp),
    RemoveComponent(RemoveComponentOp),
    AuthorityChange(AuthorityChangeOp),
    ComponentUpdate(ComponentUpdateOp),
    CommandRequest(CommandRequestOp),
    CommandResponse(CommandResponseOp),
}

impl From<Worker_Op> for WorkerOp {
    fn from(op: Worker_Op) -> Self {
        match Worker_OpType::from(op.op_type as u8) {
            Worker_OpType::WORKER_OP_TYPE_DISCONNECT => {
                Self::Disconnect(DisconnectOp::from(unsafe { op.op.disconnect }))
            }
            Worker_OpType::WORKER_OP_TYPE_FLAG_UPDATE => {
                Self::FlagUpdate(FlagUpdateOp::from(unsafe { op.op.flag_update }))
            }
            Worker_OpType::WORKER_OP_TYPE_LOG_MESSAGE => {
                Self::LogMessage(LogMessageOp::from(unsafe { op.op.log_message }))
            }
            Worker_OpType::WORKER_OP_TYPE_METRICS => {
                Self::Metrics(MetricsOp::from(unsafe { op.op.metrics }))
            }
            Worker_OpType::WORKER_OP_TYPE_CRITICAL_SECTION => {
                Self::CriticalSection(CriticalSectionOp::from(unsafe { op.op.critical_section }))
            }
            Worker_OpType::WORKER_OP_TYPE_ADD_ENTITY => {
                Self::AddEntity(AddEntityOp::from(unsafe { op.op.add_entity }))
            }
            Worker_OpType::WORKER_OP_TYPE_REMOVE_ENTITY => {
                Self::RemoveEntity(RemoveEntityOp::from(unsafe { op.op.remove_entity }))
            }
            Worker_OpType::WORKER_OP_TYPE_RESERVE_ENTITY_IDS_RESPONSE => {
                Self::ReserveEntityIdsResponse(ReserveEntityIdsResponseOp::from(unsafe {
                    op.op.reserve_entity_ids_response
                }))
            }
            Worker_OpType::WORKER_OP_TYPE_CREATE_ENTITY_RESPONSE => {
                Self::CreateEntityResponse(CreateEntityResponseOp::from(unsafe {
                    op.op.create_entity_response
                }))
            }
            Worker_OpType::WORKER_OP_TYPE_DELETE_ENTITY_RESPONSE => {
                Self::DeleteEntityResponse(DeleteEntityResponseOp::from(unsafe {
                    op.op.delete_entity_response
                }))
            }
            Worker_OpType::WORKER_OP_TYPE_ENTITY_QUERY_RESPONSE => {
                Self::EntityQueryResponse(EntityQueryResponseOp::from(unsafe {
                    op.op.entity_query_response
                }))
            }
            Worker_OpType::WORKER_OP_TYPE_ADD_COMPONENT => {
                Self::AddComponent(AddComponentOp::from(unsafe { op.op.add_component }))
            }
            Worker_OpType::WORKER_OP_TYPE_REMOVE_COMPONENT => {
                Self::RemoveComponent(RemoveComponentOp::from(unsafe { op.op.remove_component }))
            }
            Worker_OpType::WORKER_OP_TYPE_AUTHORITY_CHANGE => {
                Self::AuthorityChange(AuthorityChangeOp::from(unsafe { op.op.authority_change }))
            }
            Worker_OpType::WORKER_OP_TYPE_COMPONENT_UPDATE => {
                Self::ComponentUpdate(ComponentUpdateOp::from(unsafe { op.op.component_update }))
            }
            Worker_OpType::WORKER_OP_TYPE_COMMAND_REQUEST => {
                Self::CommandRequest(CommandRequestOp::from(unsafe { op.op.command_request }))
            }
            Worker_OpType::WORKER_OP_TYPE_COMMAND_RESPONSE => {
                Self::CommandResponse(CommandResponseOp::from(unsafe { op.op.command_response }))
            }
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a disconnect message from the SDK."]
pub struct DisconnectOp {
    #[doc = " A value from the Worker_ConnectionStatusCode enumeration."]
    pub status_code: ConnectionStatusCode,
    #[doc = " A string giving detailed information on the reason for disconnecting."]
    pub reason: String,
}

impl From<Worker_DisconnectOp> for DisconnectOp {
    fn from(op: Worker_DisconnectOp) -> Self {
        let reason = unsafe { CStr::from_ptr(op.reason) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            status_code: ConnectionStatusCode::from(op.connection_status_code),
            reason,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a log message from the SDK."]
#[doc = " Note: Worker_LogMessageOp has been deprecated and will be removed in a future version of"]
#[doc = " SpatialOS."]
pub struct LogMessageOp {
    #[doc = " The severity of the log message; defined in the LogLevel enumeration."]
    pub level: LogLevel,
    #[doc = " The message."]
    pub message: String,
}

impl From<Worker_LogMessageOp> for LogMessageOp {
    fn from(op: Worker_LogMessageOp) -> Self {
        let message = unsafe { CStr::from_ptr(op.message) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            level: LogLevel::from(op.level),
            message,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for an AddComponent operation."]
pub struct AddComponentOp {
    #[doc = " The ID of the entity for which a component was added."]
    pub entity_id: EntityId,
    #[doc = " The initial data for the new component. Deserialized with the corresponding vtable deserialize"]
    #[doc = " function and freed with the vtable free function when the OpList is destroyed."]
    pub data: Worker_ComponentData,
}

impl From<Worker_AddComponentOp> for AddComponentOp {
    fn from(op: Worker_AddComponentOp) -> Self {
        Self {
            entity_id: op.entity_id,
            data: op.data,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for an AddEntity operation."]
pub struct AddEntityOp {
    #[doc = " The ID of the entity that was added to the worker's view of the simulation."]
    pub entity_id: EntityId,
}

impl From<Worker_AddEntityOp> for AddEntityOp {
    fn from(op: Worker_AddEntityOp) -> Self {
        Self {
            entity_id: op.entity_id,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for an AuthorityChange operation."]
pub struct AuthorityChangeOp {
    #[doc = " The ID of the entity for which there was an authority change."]
    pub entity_id: EntityId,
    #[doc = " The ID of the component over which the worker's authority has changed."]
    pub component_id: ComponentId,
    #[doc = " The authority state of the component, using the Authority enumeration."]
    pub authority: Authority,
}

impl From<Worker_AuthorityChangeOp> for AuthorityChangeOp {
    fn from(op: Worker_AuthorityChangeOp) -> Self {
        Self {
            entity_id: op.entity_id,
            component_id: op.component_id,
            authority: Authority::from(op.authority),
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a CommandRequest operation."]
pub struct CommandRequestOp {
    #[doc = " The incoming command request ID."]
    pub request_id: RequestId,
    #[doc = " The ID of the entity for which there was a command request."]
    pub entity_id: EntityId,
    #[doc = " Upper bound on request timeout provided by the platform."]
    pub timeout_millis: u32,
    #[doc = " The ID of the worker that sent the request."]
    pub caller_worker_id: String,
    #[doc = " The attributes of the worker that sent the request."]
    pub caller_attribute_set: WorkerAttributes,
    #[doc = " The command request data. Deserialized with the corresponding vtable deserialize function and"]
    #[doc = " freed with the vtable free function when the OpList is destroyed."]
    pub request: Worker_CommandRequest,
}

impl From<Worker_CommandRequestOp> for CommandRequestOp {
    fn from(op: Worker_CommandRequestOp) -> Self {
        let caller_worker_id = unsafe { CStr::from_ptr(op.caller_worker_id) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            request_id: op.request_id,
            entity_id: op.entity_id,
            timeout_millis: op.timeout_millis,
            caller_attribute_set: WorkerAttributes::from(op.caller_attribute_set),
            request: op.request,
            caller_worker_id,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a CommandResponse operation."]
pub struct CommandResponseOp {
    #[doc = " The ID of the command request for which there was a command response."]
    pub request_id: RequestId,
    #[doc = " The ID of the entity originally targeted by the command request."]
    pub entity_id: EntityId,
    #[doc = " Status code of the response, using StatusCode."]
    pub status_code: StatusCode,
    #[doc = " The error message."]
    pub message: String,
    #[doc = " The command response data. Deserialized with the corresponding vtable deserialize function and"]
    #[doc = " freed with the vtable free function when the OpList is destroyed."]
    pub response: Worker_CommandResponse,
}

impl From<Worker_CommandResponseOp> for CommandResponseOp {
    fn from(op: Worker_CommandResponseOp) -> Self {
        let message = unsafe { CStr::from_ptr(op.message) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            request_id: op.request_id,
            entity_id: op.entity_id,
            status_code: StatusCode::from(op.status_code),
            response: op.response,
            message,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a ComponentUpdate operation."]
pub struct ComponentUpdateOp {
    #[doc = " The ID of the entity for which there was a component update."]
    pub entity_id: EntityId,
    #[doc = " The new component data for the updated entity. Deserialized with the corresponding vtable"]
    #[doc = " deserialize function and freed with the vtable free function when the OpList is destroyed."]
    pub update: Worker_ComponentUpdate,
}

impl From<Worker_ComponentUpdateOp> for ComponentUpdateOp {
    fn from(op: Worker_ComponentUpdateOp) -> Self {
        Self {
            entity_id: op.entity_id,
            update: op.update,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a CreateEntity operation."]
pub struct CreateEntityResponseOp {
    #[doc = " The ID of the request for which there was a response."]
    pub request_id: RequestId,
    #[doc = " Status code of the response, using Worker_StatusCode."]
    pub status_code: StatusCode,
    #[doc = " The error message."]
    pub message: String,
    #[doc = " If successful, the entity ID of the newly created entity."]
    pub entity_id: EntityId,
}

impl From<Worker_CreateEntityResponseOp> for CreateEntityResponseOp {
    fn from(op: Worker_CreateEntityResponseOp) -> Self {
        let message = unsafe { CStr::from_ptr(op.message) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            request_id: op.request_id,
            status_code: StatusCode::from(op.status_code),
            entity_id: op.entity_id,
            message,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a critical section boundary (enter or leave) operation."]
pub struct CriticalSectionOp {
    #[doc = " Whether the protocol is entering a critical section (true) or leaving it (false)."]
    pub in_critical_section: bool,
}

impl From<Worker_CriticalSectionOp> for CriticalSectionOp {
    fn from(op: Worker_CriticalSectionOp) -> Self {
        Self {
            in_critical_section: match op.in_critical_section {
                0 => false,
                1 => true,
                _ => panic!("Invalid byte"),
            },
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a DeleteEntity operation."]
pub struct DeleteEntityResponseOp {
    #[doc = " The ID of the delete entity request for which there was a command response."]
    pub request_id: RequestId,
    #[doc = " The ID of the target entity of this request."]
    pub entity_id: EntityId,
    #[doc = " Status code of the response, using StatusCode."]
    pub status_code: StatusCode,
    #[doc = " The error message."]
    pub message: String,
}

impl From<Worker_DeleteEntityResponseOp> for DeleteEntityResponseOp {
    fn from(op: Worker_DeleteEntityResponseOp) -> Self {
        let message = unsafe { CStr::from_ptr(op.message) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            request_id: op.request_id,
            status_code: StatusCode::from(op.status_code),
            entity_id: op.entity_id,
            message,
        }
    }
}

#[derive(Debug)]
#[doc = " A response indicating the result of an entity query request."]
pub struct EntityQueryResponseOp {
    #[doc = " The ID of the entity query request for which there was a response."]
    pub request_id: RequestId,
    #[doc = " Status code of the response, using StatusCode."]
    pub status_code: StatusCode,
    #[doc = " The error message."]
    pub message: String,
    #[doc = " Number of entities in the result set. Reused to indicate the result itself for CountResultType"]
    #[doc = " queries."]
    pub result_count: u32,
    #[doc = " Array of entities in the result set. Will be NULL if the query was a count query. Snapshot data"]
    #[doc = " in the result is deserialized with the corresponding vtable deserialize function and freed with"]
    #[doc = " the vtable free function when the OpList is destroyed."]
    pub results: Vec<Entity>,
}

impl From<Worker_EntityQueryResponseOp> for EntityQueryResponseOp {
    fn from(op: Worker_EntityQueryResponseOp) -> Self {
        let message = unsafe { CStr::from_ptr(op.message) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        let results = const_to_vector(op.results, op.result_count as isize)
            .into_iter()
            .map(|e| e.into())
            .collect();
        Self {
            request_id: op.request_id,
            status_code: StatusCode::from(op.status_code),
            result_count: op.result_count,
            results,
            message,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a FlagUpdate operation."]
pub struct FlagUpdateOp {
    #[doc = " The name of the updated worker flag."]
    pub name: String,
    #[doc = " The new value of the updated worker flag."]
    #[doc = " A None value indicates that the flag has been deleted."]
    pub value: Option<String>,
}

impl From<Worker_FlagUpdateOp> for FlagUpdateOp {
    fn from(op: Worker_FlagUpdateOp) -> Self {
        let name = unsafe { CStr::from_ptr(op.name) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        if op.value.is_null() {
            Self { name, value: None }
        } else {
            let value = unsafe { CStr::from_ptr(op.value) }
                .to_str()
                .map(|s| s.to_owned())
                .unwrap();
            Self {
                name,
                value: Some(value),
            }
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a set of built-in metrics reported by the SDK."]
pub struct MetricsOp {
    pub metrics: Metrics,
}

impl From<Worker_MetricsOp> for MetricsOp {
    fn from(op: Worker_MetricsOp) -> Self {
        Self {
            metrics: Metrics::from(op.metrics),
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a RemoveComponent operation."]
pub struct RemoveComponentOp {
    #[doc = " The ID of the entity for which a component was removed."]
    pub entity_id: EntityId,
    #[doc = " The ID of the component that was removed."]
    pub component_id: ComponentId,
}

impl From<Worker_RemoveComponentOp> for RemoveComponentOp {
    fn from(op: Worker_RemoveComponentOp) -> Self {
        Self {
            entity_id: op.entity_id,
            component_id: op.component_id,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a RemoveEntity operation."]
pub struct RemoveEntityOp {
    #[doc = " The ID of the entity that was removed from the worker's view of the simulation."]
    pub entity_id: EntityId,
}

impl From<Worker_RemoveEntityOp> for RemoveEntityOp {
    fn from(op: Worker_RemoveEntityOp) -> Self {
        Self {
            entity_id: op.entity_id,
        }
    }
}

#[derive(Debug)]
#[doc = " Data for a ReserveEntityIdsResponse operation."]
pub struct ReserveEntityIdsResponseOp {
    #[doc = " The ID of the reserve entity ID request for which there was a response."]
    pub request_id: RequestId,
    #[doc = " Status code of the response, using StatusCode."]
    pub status_code: StatusCode,
    #[doc = " The error message."]
    pub message: String,
    #[doc = " If successful, an ID which is the first in a contiguous range of newly allocated entity"]
    #[doc = " IDs which are guaranteed to be unused in the current deployment."]
    pub first_entity_id: EntityId,
    #[doc = " If successful, the number of IDs reserved in the contiguous range, otherwise 0."]
    pub number_of_entity_ids: u32,
}

impl From<Worker_ReserveEntityIdsResponseOp> for ReserveEntityIdsResponseOp {
    fn from(op: Worker_ReserveEntityIdsResponseOp) -> Self {
        let message = unsafe { CStr::from_ptr(op.message) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            request_id: op.request_id,
            status_code: StatusCode::from(op.status_code),
            first_entity_id: op.first_entity_id,
            number_of_entity_ids: op.number_of_entity_ids,
            message,
        }
    }
}
