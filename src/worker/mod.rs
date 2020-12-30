use spatialos_sys::*;

pub mod component_vtable;
pub mod connection;
pub mod constraint;
pub mod log_message;
pub mod metrics;
pub mod op;

use crate::worker::constraint::Constraint;
use crate::worker::constraint::EntityIdConstraint;
use crate::{const_to_vector, schema};
use std::ffi::CStr;
use std::os::raw::c_void;

pub type EntityId = i64;
pub type ComponentId = u32;
pub type RequestId = i64;
pub type CommandIndex = u32;

pub type CommandRequestHandle = c_void;
pub type CommandResponseHandle = c_void;
pub type ComponentDataHandle = c_void;
pub type ComponentUpdateHandle = c_void;

#[derive(Debug)]
#[doc = " Enum defining the severities of log messages that can be sent to SpatialOS and received from the"]
#[doc = " SDK."]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl From<Worker_LogLevel> for LogLevel {
    fn from(log_level: Worker_LogLevel) -> Self {
        match log_level {
            Worker_LogLevel::WORKER_LOG_LEVEL_DEBUG => Self::Debug,
            Worker_LogLevel::WORKER_LOG_LEVEL_INFO => Self::Info,
            Worker_LogLevel::WORKER_LOG_LEVEL_WARN => Self::Warn,
            Worker_LogLevel::WORKER_LOG_LEVEL_ERROR => Self::Error,
            Worker_LogLevel::WORKER_LOG_LEVEL_FATAL => Self::Fatal,
        }
    }
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        LogLevel::from(Worker_LogLevel::from(log_level))
    }
}

impl From<LogLevel> for Worker_LogLevel {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Debug => Self::WORKER_LOG_LEVEL_DEBUG,
            LogLevel::Info => Self::WORKER_LOG_LEVEL_INFO,
            LogLevel::Warn => Self::WORKER_LOG_LEVEL_WARN,
            LogLevel::Error => Self::WORKER_LOG_LEVEL_ERROR,
            LogLevel::Fatal => Self::WORKER_LOG_LEVEL_FATAL,
        }
    }
}

impl From<LogLevel> for u8 {
    fn from(log_level: LogLevel) -> Self {
        let log_level: Worker_LogLevel = log_level.into();
        log_level.into()
    }
}

#[derive(Debug)]
#[doc = " Enum defining the possible authority states for an entity component."]
pub enum Authority {
    NotAuthoritative,
    Authoritative,
    AuthorityLossImminent,
}

impl From<Worker_Authority> for Authority {
    fn from(authority: Worker_Authority) -> Self {
        match authority {
            Worker_Authority::WORKER_AUTHORITY_NOT_AUTHORITATIVE => Self::NotAuthoritative,
            Worker_Authority::WORKER_AUTHORITY_AUTHORITATIVE => Self::Authoritative,
            Worker_Authority::WORKER_AUTHORITY_AUTHORITY_LOSS_IMMINENT => {
                Self::AuthorityLossImminent
            }
        }
    }
}

impl From<u8> for Authority {
    fn from(authority: u8) -> Self {
        Authority::from(Worker_Authority::from(authority))
    }
}

#[derive(Debug)]
#[doc = " Enum defining possible command status codes."]
pub enum StatusCode {
    #[doc = " The request was successfully executed and returned a response."]
    Success,
    #[doc = " The request timed out before a response was received. It can be retried, but carefully - this"]
    #[doc = " usually means the deployment is overloaded, so some sort of backoff should be used to avoid"]
    #[doc = " making the problem worse. This can also be caused by the target worker's handling code failing"]
    #[doc = " to respond to the command at all, perhaps due to a bug in its implementation."]
    Timeout,
    #[doc = " The target entity did not exist, or did not have the target component. This probably means the"]
    #[doc = " entity either hasn't been created yet or has already been deleted. It might make sense to retry"]
    #[doc = " the request if there is reason to believe the entity hasn't yet been created but will be soon."]
    NotFound,
    #[doc = " The request could not be executed by a worker, either because the worker lost authority over"]
    #[doc = " the entity while handling the request, the entity was deleted while handling the request, or no"]
    #[doc = " worker was authoritative over the entity at all. Assuming the deployment isn't irrecoverably"]
    #[doc = " broken (e.g. due to misconfigured loadbalancing or crash-looping workers) this is a transient"]
    #[doc = " failure and can be retried immediately."]
    AuthorityLost,
    #[doc = " The worker did not have the required permissions to make the request. Permissions do not change"]
    #[doc = " at runtime, so it doesn't make sense to retry the request."]
    PermissionDenied,
    #[doc = " The command was delivered successfully, but the handler rejected it. Either the command was"]
    #[doc = " delivered to a worker that explicitly rejected it by calling"]
    #[doc = " Worker_Connection_SendCommandFailure, or the request data was rejected as invalid by SpatialOS"]
    #[doc = " itself. In the latter case, in particular, Worker_Connection_SendCreateEntityRequest will"]
    #[doc = " return kApplicationError if an entity ID reservation has expired, and"]
    #[doc = " Worker_Connection_SendEntityQueryResult will return kApplicationError if the result set is"]
    #[doc = " incomplete."]
    ApplicationError,
    #[doc = " Some other error occurred. This likely indicates a bug in SpatialOS and should be reported."]
    InternalError,
}

impl From<Worker_StatusCode> for StatusCode {
    fn from(status_code: Worker_StatusCode) -> Self {
        match status_code {
            Worker_StatusCode::WORKER_STATUS_CODE_SUCCESS => Self::Success,
            Worker_StatusCode::WORKER_STATUS_CODE_TIMEOUT => Self::Timeout,
            Worker_StatusCode::WORKER_STATUS_CODE_NOT_FOUND => Self::NotFound,
            Worker_StatusCode::WORKER_STATUS_CODE_AUTHORITY_LOST => Self::AuthorityLost,
            Worker_StatusCode::WORKER_STATUS_CODE_PERMISSION_DENIED => Self::PermissionDenied,
            Worker_StatusCode::WORKER_STATUS_CODE_APPLICATION_ERROR => Self::ApplicationError,
            Worker_StatusCode::WORKER_STATUS_CODE_INTERNAL_ERROR => Self::InternalError,
        }
    }
}

impl From<u8> for StatusCode {
    fn from(status_code: u8) -> Self {
        StatusCode::from(Worker_StatusCode::from(status_code))
    }
}

#[derive(Debug)]
#[doc = " Possible status codes for a remote call, connection attempt, or connection migration attempt."]
pub enum ConnectionStatusCode {
    #[doc = " The remote call was successful, or we are successfully connected."]
    Success,
    #[doc = " Protocol violation, or some part of the system otherwise behaved in an unexpected way. Not"]
    #[doc = " expected to occur in normal operation."]
    InternalError,
    #[doc = " An argument provided by the caller was determined to be invalid. This is a local failure; no"]
    #[doc = " actual attempt was made to contact the host. Not retryable."]
    InvalidArgument,
    #[doc = " Failed due to a networking issue or otherwise unreachable host."]
    NetworkError,
    #[doc = " A timeout provided by the caller or enforced by the system was exceeded. Can be retried."]
    Timeout,
    #[doc = " Attempt was cancelled by the caller. Currently shouldn't happen; reserved for future use."]
    Cancelled,
    #[doc = " Made contact with the host, but the request was explicitly rejected. Unlikely to be retryable."]
    #[doc = " Possible causes include: the request was made to the wrong host; the host considered the"]
    #[doc = " request invalid for some other reason."]
    Rejected,
    #[doc = " The player identity token provided by the caller has expired. Generate a new one and retry."]
    PlayerIdentityTokenExpired,
    #[doc = " The login token provided by the caller has expired. Generate a new one and retry."]
    LoginTokenExpired,
    #[doc = " Failed because the deployment associated with the provided login token was at capacity."]
    #[doc = " Retryable."]
    CapacityExceeded,
    #[doc = " Failed due to rate-limiting of new connections to the deployment associated with the provided"]
    #[doc = " login token. Retryable."]
    RateExceeded,
    #[doc = " After a successful connection attempt, the server later explicitly terminated the connection."]
    #[doc = " Possible causes include: the deployment was stopped; the worker was killed due to"]
    #[doc = " unresponsiveness."]
    ServerShutdown,
}

impl From<Worker_ConnectionStatusCode> for ConnectionStatusCode {
    fn from(status_code: Worker_ConnectionStatusCode) -> Self {
        match status_code {
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_SUCCESS => Self::Success,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_INTERNAL_ERROR => Self::InternalError,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_INVALID_ARGUMENT => Self::InvalidArgument,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_NETWORK_ERROR => Self::NetworkError,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_TIMEOUT => Self::Timeout,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_CANCELLED => Self::Cancelled,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_REJECTED => Self::Rejected,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_PLAYER_IDENTITY_TOKEN_EXPIRED => {
                Self::PlayerIdentityTokenExpired
            }
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_LOGIN_TOKEN_EXPIRED => Self::LoginTokenExpired,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_CAPACITY_EXCEEDED => Self::CapacityExceeded,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_RATE_EXCEEDED => Self::RateExceeded,
            Worker_ConnectionStatusCode::WORKER_CONNECTION_STATUS_CODE_SERVER_SHUTDOWN => Self::ServerShutdown,
        }
    }
}

impl From<u8> for ConnectionStatusCode {
    fn from(status_code: u8) -> Self {
        ConnectionStatusCode::from(Worker_ConnectionStatusCode::from(status_code))
    }
}

#[derive(Debug)]
#[doc = " Worker attributes that are part of a worker's runtime configuration."]
pub struct WorkerAttributes {
    #[doc = " Number of worker attributes."]
    pub attribute_count: u32,
    #[doc = " Will be empty if there are no attributes associated with the worker."]
    pub attributes: Vec<String>,
}

impl From<Worker_WorkerAttributes> for WorkerAttributes {
    fn from(worker_attributes: Worker_WorkerAttributes) -> Self {
        if worker_attributes.attributes.is_null() {
            Self {
                attribute_count: worker_attributes.attribute_count,
                attributes: Vec::new(),
            }
        } else {
            let attributes = unsafe {
                let index = 0;
                let mut attributes = Vec::new();
                loop {
                    let char_ptr = worker_attributes.attributes.offset(index);
                    if (*char_ptr).is_null() {
                        break;
                    } else {
                        let attribute = CStr::from_ptr(*char_ptr)
                            .to_str()
                            .map(|s| s.to_owned())
                            .unwrap();
                        attributes.push(attribute);
                    }
                }
                attributes
            };
            Self {
                attribute_count: worker_attributes.attribute_count,
                attributes,
            }
        }
    }
}

pub enum ResultType {
    Count,
    Snapshot,
}

impl From<Worker_ResultType> for ResultType {
    fn from(result_type: Worker_ResultType) -> Self {
        match result_type {
            Worker_ResultType::WORKER_RESULT_TYPE_COUNT => Self::Count,
            Worker_ResultType::WORKER_RESULT_TYPE_SNAPSHOT => Self::Snapshot,
        }
    }
}

impl From<u8> for ResultType {
    fn from(result_type: u8) -> Self {
        ResultType::from(Worker_ResultType::from(result_type))
    }
}

impl From<ResultType> for Worker_ResultType {
    fn from(result_type: ResultType) -> Self {
        match result_type {
            ResultType::Count => Self::WORKER_RESULT_TYPE_COUNT,
            ResultType::Snapshot => Self::WORKER_RESULT_TYPE_SNAPSHOT,
        }
    }
}

impl From<ResultType> for u8 {
    fn from(result_type: ResultType) -> Self {
        Worker_ResultType::from(result_type).into()
    }
}

#[doc = " An entity query."]
pub struct EntityQuery {
    #[doc = " The constraint for this query."]
    pub constraint: Constraint,
    #[doc = " Result type for this query, using Worker_ResultType."]
    pub result_type: ResultType,
    #[doc = " Number of component IDs in the array for a snapshot result type."]
    pub snapshot_result_type_component_id_count: u32,
    #[doc = " Pointer to component ID data for a snapshot result type. None means all component IDs."]
    pub snapshot_result_type_component_ids: Vec<ComponentId>,
}

impl Default for EntityQuery {
    fn default() -> Self {
        Self {
            constraint: Constraint::EntityId(EntityIdConstraint { entity_id: 0 }),
            result_type: ResultType::Count,
            snapshot_result_type_component_id_count: 0,
            snapshot_result_type_component_ids: Vec::new(),
        }
    }
}

impl From<Worker_EntityQuery> for EntityQuery {
    fn from(query: Worker_EntityQuery) -> Self {
        let snapshot_result_type_component_ids =
            if query.snapshot_result_type_component_ids.is_null() {
                Vec::new()
            } else {
                unsafe {
                    let mut component_ids = Vec::new();
                    for index in 0..query.snapshot_result_type_component_id_count {
                        let component_id_ptr = query
                            .snapshot_result_type_component_ids
                            .offset(index as isize);
                        component_ids.push(*component_id_ptr);
                    }
                    component_ids
                }
            };
        Self {
            constraint: query.constraint.into(),
            result_type: query.result_type.into(),
            snapshot_result_type_component_id_count: query.snapshot_result_type_component_id_count,
            snapshot_result_type_component_ids,
        }
    }
}

impl From<EntityQuery> for Worker_EntityQuery {
    fn from(query: EntityQuery) -> Self {
        let mut vec = query.snapshot_result_type_component_ids;
        vec.shrink_to_fit();
        let vec_ptr = vec.as_mut_ptr();
        std::mem::forget(vec);
        Self {
            constraint: query.constraint.into(),
            result_type: query.result_type.into(),
            snapshot_result_type_component_id_count: query.snapshot_result_type_component_id_count,
            snapshot_result_type_component_ids: vec_ptr,
        }
    }
}

#[doc = " Represents an entity with an ID and a component data snapshot."]
#[derive(Debug, Clone)]
pub struct Entity {
    #[doc = " The ID of the entity."]
    pub entity_id: Worker_EntityId,
    #[doc = " Number of components for the entity."]
    pub component_count: u32,
    #[doc = " Array of initial component data for the entity."]
    pub components: Vec<ComponentData>,
}

impl From<Worker_Entity> for Entity {
    fn from(entity: Worker_Entity) -> Self {
        let components = const_to_vector(entity.components, entity.component_count as isize)
            .into_iter()
            .map(ComponentData::from)
            .collect();
        Self {
            entity_id: entity.entity_id,
            component_count: entity.component_count,
            components,
        }
    }
}

#[doc = " An object used to represent a component data snapshot by either raw schema data or some"]
#[doc = " user-defined handle type."]
#[derive(Debug, Clone)]
pub struct ComponentData {
    pub reserved: *mut c_void,
    pub component_id: ComponentId,
    pub schema_type: schema::ComponentData,
    pub user_handle: *mut ComponentDataHandle,
}

impl From<Worker_ComponentData> for ComponentData {
    fn from(data: Worker_ComponentData) -> Self {
        Self {
            reserved: data.reserved,
            component_id: data.component_id,
            schema_type: data.schema_type.into(),
            user_handle: data.user_handle,
        }
    }
}
