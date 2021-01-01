use spatialos_sys::{
    Worker_ApiVersion, Worker_ApiVersionStr, Worker_Authority, Worker_CommandIndex,
    Worker_CommandRequestHandle, Worker_CommandResponseHandle, Worker_ComponentData,
    Worker_ComponentDataHandle, Worker_ComponentId, Worker_ComponentUpdateHandle,
    Worker_ConnectionStatusCode, Worker_Entity, Worker_EntityId, Worker_EntityQuery,
    Worker_LogLevel, Worker_RequestId, Worker_ResultType, Worker_StatusCode,
    Worker_WorkerAttributes,
};

pub mod component_vtable;
pub mod connection;
pub mod constraint;
pub mod log_message;
pub mod metrics;
pub mod op;

use crate::{const_to_string, worker::constraint::EntityIdConstraint};
use crate::{const_to_vector, schema};
use crate::{vector_to_owned_array, worker::constraint::Constraint};
use std::ffi::CStr;
use std::os::raw::c_void;

pub type EntityId = Worker_EntityId;
pub type ComponentId = Worker_ComponentId;
pub type RequestId = Worker_RequestId;
pub type CommandIndex = Worker_CommandIndex;

pub type CommandRequestHandle = Worker_CommandRequestHandle;
pub type CommandResponseHandle = Worker_CommandResponseHandle;
pub type ComponentDataHandle = Worker_ComponentDataHandle;
pub type ComponentUpdateHandle = Worker_ComponentUpdateHandle;

pub fn api_version() -> u32 {
    unsafe { Worker_ApiVersion() }
}

pub fn api_version_str() -> String {
    const_to_string(unsafe { Worker_ApiVersionStr() })
}

#[derive(Debug)]
/// Enum defining the severities of log messages that can be sent to SpatialOS and received from the
/// SDK.
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

impl Into<Worker_LogLevel> for LogLevel {
    fn into(self) -> Worker_LogLevel {
        match self {
            Self::Debug => Worker_LogLevel::WORKER_LOG_LEVEL_DEBUG,
            Self::Info => Worker_LogLevel::WORKER_LOG_LEVEL_INFO,
            Self::Warn => Worker_LogLevel::WORKER_LOG_LEVEL_WARN,
            Self::Error => Worker_LogLevel::WORKER_LOG_LEVEL_ERROR,
            Self::Fatal => Worker_LogLevel::WORKER_LOG_LEVEL_FATAL,
        }
    }
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        let log_level: Worker_LogLevel = log_level.into();
        log_level.into()
    }
}

impl Into<u8> for LogLevel {
    fn into(self) -> u8 {
        let log_level: Worker_LogLevel = self.into();
        log_level.into()
    }
}

#[derive(Debug)]
/// Enum defining the possible authority states for an entity component.
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
/// Enum defining possible command status codes.
pub enum StatusCode {
    /// The request was successfully executed and returned a response.
    Success,
    /// The request timed out before a response was received. It can be retried, but carefully - this
    /// usually means the deployment is overloaded, so some sort of backoff should be used to avoid
    /// making the problem worse. This can also be caused by the target worker's handling code failing
    /// to respond to the command at all, perhaps due to a bug in its implementation.
    Timeout,
    /// The target entity did not exist, or did not have the target component. This probably means the
    /// entity either hasn't been created yet or has already been deleted. It might make sense to retry
    /// the request if there is reason to believe the entity hasn't yet been created but will be soon.
    NotFound,
    /// The request could not be executed by a worker, either because the worker lost authority over
    /// the entity while handling the request, the entity was deleted while handling the request, or no
    /// worker was authoritative over the entity at all. Assuming the deployment isn't irrecoverably
    /// broken (e.g. due to misconfigured loadbalancing or crash-looping workers) this is a transient
    /// failure and can be retried immediately.
    AuthorityLost,
    /// The worker did not have the required permissions to make the request. Permissions do not change
    /// at runtime, so it doesn't make sense to retry the request.
    PermissionDenied,
    /// The command was delivered successfully, but the handler rejected it. Either the command was
    /// delivered to a worker that explicitly rejected it by calling
    /// Worker_Connection_SendCommandFailure, or the request data was rejected as invalid by SpatialOS
    /// itself. In the latter case, in particular, Worker_Connection_SendCreateEntityRequest will
    /// return kApplicationError if an entity ID reservation has expired, and
    /// Worker_Connection_SendEntityQueryResult will return kApplicationError if the result set is
    /// incomplete.
    ApplicationError,
    /// Some other error occurred. This likely indicates a bug in SpatialOS and should be reported.
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
/// Possible status codes for a remote call, connection attempt, or connection migration attempt.
pub enum ConnectionStatusCode {
    /// The remote call was successful, or we are successfully connected.
    Success,
    /// Protocol violation, or some part of the system otherwise behaved in an unexpected way. Not
    /// expected to occur in normal operation.
    InternalError,
    /// An argument provided by the caller was determined to be invalid. This is a local failure; no
    /// actual attempt was made to contact the host. Not retryable.
    InvalidArgument,
    /// Failed due to a networking issue or otherwise unreachable host.
    NetworkError,
    /// A timeout provided by the caller or enforced by the system was exceeded. Can be retried.
    Timeout,
    /// Attempt was cancelled by the caller. Currently shouldn't happen; reserved for future use.
    Cancelled,
    /// Made contact with the host, but the request was explicitly rejected. Unlikely to be retryable.
    /// Possible causes include: the request was made to the wrong host; the host considered the
    /// request invalid for some other reason.
    Rejected,
    /// The player identity token provided by the caller has expired. Generate a new one and retry.
    PlayerIdentityTokenExpired,
    /// The login token provided by the caller has expired. Generate a new one and retry.
    LoginTokenExpired,
    /// Failed because the deployment associated with the provided login token was at capacity.
    /// Retryable.
    CapacityExceeded,
    /// Failed due to rate-limiting of new connections to the deployment associated with the provided
    /// login token. Retryable.
    RateExceeded,
    /// After a successful connection attempt, the server later explicitly terminated the connection.
    /// Possible causes include: the deployment was stopped; the worker was killed due to
    /// unresponsiveness.
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
/// Worker attributes that are part of a worker's runtime configuration.
pub struct WorkerAttributes {
    /// Number of worker attributes.
    pub attribute_count: u32,
    /// Will be empty if there are no attributes associated with the worker.
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

impl Into<Worker_ResultType> for ResultType {
    fn into(self) -> Worker_ResultType {
        match self {
            Self::Count => Worker_ResultType::WORKER_RESULT_TYPE_COUNT,
            Self::Snapshot => Worker_ResultType::WORKER_RESULT_TYPE_SNAPSHOT,
        }
    }
}

impl From<u8> for ResultType {
    fn from(result_type: u8) -> Self {
        let result_type: Worker_ResultType = result_type.into();
        result_type.into()
    }
}

impl Into<u8> for ResultType {
    fn into(self) -> u8 {
        let result_type: Worker_ResultType = self.into();
        result_type.into()
    }
}

/// An entity query.
pub struct EntityQuery {
    /// The constraint for this query.
    pub constraint: Constraint,
    /// Result type for this query, using Worker_ResultType.
    pub result_type: ResultType,
    /// Pointer to component ID data for a snapshot result type. None means all component IDs.
    pub snapshot_result_type_component_ids: Vec<ComponentId>,
}

impl Default for EntityQuery {
    fn default() -> Self {
        Self {
            constraint: Constraint::EntityId(EntityIdConstraint { entity_id: 0 }),
            result_type: ResultType::Count,
            snapshot_result_type_component_ids: Vec::new(),
        }
    }
}

impl From<Worker_EntityQuery> for EntityQuery {
    fn from(query: Worker_EntityQuery) -> Self {
        let snapshot_result_type_component_ids = const_to_vector(
            query.snapshot_result_type_component_ids,
            query.snapshot_result_type_component_id_count as isize,
        );
        Self {
            constraint: query.constraint.into(),
            result_type: query.result_type.into(),
            snapshot_result_type_component_ids,
        }
    }
}

impl Into<Worker_EntityQuery> for EntityQuery {
    fn into(self) -> Worker_EntityQuery {
        let (snapshot_result_type_component_ids, snapshot_result_type_component_id_count) =
            vector_to_owned_array(self.snapshot_result_type_component_ids);
        Worker_EntityQuery {
            constraint: self.constraint.into(),
            result_type: self.result_type.into(),
            snapshot_result_type_component_id_count: snapshot_result_type_component_id_count as u32,
            snapshot_result_type_component_ids,
        }
    }
}

/// Represents an entity with an ID and a component data snapshot.
#[derive(Debug, Clone)]
pub struct Entity {
    /// The ID of the entity.
    pub entity_id: Worker_EntityId,
    /// Number of components for the entity.
    pub component_count: u32,
    /// Array of initial component data for the entity.
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

impl Into<Worker_Entity> for Entity {
    fn into(self) -> Worker_Entity {
        let components = self
            .components
            .into_iter()
            .map(|c| c.into())
            .collect::<Vec<_>>();
        let (components, component_count) = vector_to_owned_array(components);
        Worker_Entity {
            entity_id: self.entity_id,
            component_count: component_count as u32,
            components,
        }
    }
}

/// An object used to represent a component data snapshot by either raw schema data or some
/// user-defined handle type.
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

impl Into<Worker_ComponentData> for ComponentData {
    fn into(self) -> Worker_ComponentData {
        Worker_ComponentData {
            reserved: self.reserved,
            component_id: self.component_id,
            schema_type: self.schema_type.into(),
            user_handle: self.user_handle,
        }
    }
}
