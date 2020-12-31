use spatialos_sys::*;

use crate::worker::log_message::LogMessage;
use crate::worker::op::OpList;
use crate::worker::EntityQuery;
use crate::worker::RequestId;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Network connection type used by Worker_NetworkParameters.
pub enum NetworkConnectionType {
    /// (deprecated) Use this flag to connect over TCP.
    Tcp = 0,
    /// (deprecated) Use this flag to connect over RakNet.
    RakNet = 1,
    /// (deprecated) Use this flag to connect over KCP.
    Kcp = 2,
    /// Use this flag to connect over the modular KCP stack. Modular KCP connections run on a new
    /// network stack with additional optional features such as compression.
    ModularKcp = 3,
    /// Use this flag to connect over the modular TCP stack. Modular TCP connections run on a new
    /// network stack with additional optional features such as compression.
    ModularTcp = 4,
}

impl From<Worker_NetworkConnectionType> for NetworkConnectionType {
    fn from(connection_type: Worker_NetworkConnectionType) -> Self {
        match connection_type {
            Worker_NetworkConnectionType::WORKER_NETWORK_CONNECTION_TYPE_TCP => Self::Tcp,
            Worker_NetworkConnectionType::WORKER_NETWORK_CONNECTION_TYPE_RAKNET => Self::RakNet,
            Worker_NetworkConnectionType::WORKER_NETWORK_CONNECTION_TYPE_KCP => Self::Kcp,
            Worker_NetworkConnectionType::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_KCP => {
                Self::ModularKcp
            }
            Worker_NetworkConnectionType::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_TCP => {
                Self::ModularTcp
            }
        }
    }
}

impl From<NetworkConnectionType> for Worker_NetworkConnectionType {
    fn from(connection_type: NetworkConnectionType) -> Self {
        match connection_type {
            NetworkConnectionType::Tcp => Self::WORKER_NETWORK_CONNECTION_TYPE_TCP,
            NetworkConnectionType::RakNet => Self::WORKER_NETWORK_CONNECTION_TYPE_RAKNET,
            NetworkConnectionType::Kcp => Self::WORKER_NETWORK_CONNECTION_TYPE_KCP,
            NetworkConnectionType::ModularKcp => Self::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_KCP,
            NetworkConnectionType::ModularTcp => Self::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_TCP,
        }
    }
}

impl From<u8> for NetworkConnectionType {
    fn from(connection_type: u8) -> Self {
        NetworkConnectionType::from(Worker_NetworkConnectionType::from(connection_type))
    }
}

impl From<NetworkConnectionType> for u8 {
    fn from(connection_type: NetworkConnectionType) -> Self {
        let connection_type: Worker_NetworkConnectionType = connection_type.into();
        connection_type.into()
    }
}

/// Enum defining the possible network security types.
pub enum NetworkSecurityType {
    /// No encryption or security. Only safe for use in trusted environments.
    Insecure,
    /// Uses DTLS or TLS as approriate for UDP-based and TCP-based connections respectively.
    Tls,
}

impl From<Worker_NetworkSecurityType> for NetworkSecurityType {
    fn from(security_type: Worker_NetworkSecurityType) -> Self {
        match security_type {
            Worker_NetworkSecurityType::WORKER_NETWORK_SECURITY_TYPE_INSECURE => Self::Insecure,
            Worker_NetworkSecurityType::WORKER_NETWORK_SECURITY_TYPE_TLS => Self::Tls,
        }
    }
}

impl From<u8> for NetworkSecurityType {
    fn from(security_type: u8) -> Self {
        NetworkSecurityType::from(Worker_NetworkSecurityType::from(security_type))
    }
}

impl From<NetworkSecurityType> for Worker_NetworkSecurityType {
    fn from(security_type: NetworkSecurityType) -> Self {
        match security_type {
            NetworkSecurityType::Insecure => Self::WORKER_NETWORK_SECURITY_TYPE_INSECURE,
            NetworkSecurityType::Tls => Self::WORKER_NETWORK_SECURITY_TYPE_TLS,
        }
    }
}

impl From<NetworkSecurityType> for u8 {
    fn from(security_type: NetworkSecurityType) -> Self {
        let security_type: Worker_NetworkSecurityType = security_type.into();
        security_type.into()
    }
}

/// Parameters for configuring the stack for a modular KCP connection. Used by
/// Worker_NetworkParameters.
pub struct ModularKcpNetworkParameters {
    /// Type of encryption layer security to use, defined in Worker_NetworkSecurityType.
    pub security_type: NetworkSecurityType,
    /// Number of multiplexed KCP streams. Updates for entities are sharded across streams: the higher
    /// the multiplex level, the fewer entities might be impacted by a delayed update. Increasing the
    /// number of multiplexed streams may increase CPU usage.
    pub multiplex_level: u8,
    /// KCP parameters for messages sent from the bridge to the worker.
    pub downstream_kcp: Worker_KcpTransportParameters,
    /// KCP parameters for messages sent from the worker to the bridge.
    pub upstream_kcp: Worker_KcpTransportParameters,
    /// Erasure codec parameters for messages sent from the bridge to the worker.
    pub downstream_erasure_codec: *const Worker_ErasureCodecParameters,
    /// Erasure codec parameters for messages sent from the worker to the bridge.
    pub upstream_erasure_codec: *const Worker_ErasureCodecParameters,
    /// Heartbeat parameters for heartbeats from the bridge to the worker.
    pub downstream_heartbeat: *const Worker_HeartbeatParameters,
    /// Heartbeat parameters for heartbeats from the worker to the bridge.
    pub upstream_heartbeat: *const Worker_HeartbeatParameters,
    /// Compression parameters for messages sent from the bridge to the worker.
    pub downstream_compression: *const Worker_CompressionParameters,
    /// Compression parameters for messages sent from the worker to the bridge.
    pub upstream_compression: *const Worker_CompressionParameters,
    /// Flow control parameters.
    pub flow_control: *const Worker_FlowControlParameters,
}

impl From<Worker_ModularKcpNetworkParameters> for ModularKcpNetworkParameters {
    fn from(parameters: Worker_ModularKcpNetworkParameters) -> Self {
        Self {
            security_type: NetworkSecurityType::from(parameters.security_type),
            multiplex_level: parameters.multiplex_level,
            downstream_kcp: parameters.downstream_kcp,
            upstream_kcp: parameters.upstream_kcp,
            downstream_erasure_codec: parameters.downstream_erasure_codec,
            upstream_erasure_codec: parameters.upstream_erasure_codec,
            downstream_heartbeat: parameters.downstream_heartbeat,
            upstream_heartbeat: parameters.upstream_heartbeat,
            downstream_compression: parameters.downstream_compression,
            upstream_compression: parameters.upstream_compression,
            flow_control: parameters.flow_control,
        }
    }
}

impl From<ModularKcpNetworkParameters> for Worker_ModularKcpNetworkParameters {
    fn from(parameters: ModularKcpNetworkParameters) -> Self {
        Self {
            security_type: parameters.security_type.into(),
            multiplex_level: parameters.multiplex_level,
            downstream_kcp: parameters.downstream_kcp,
            upstream_kcp: parameters.upstream_kcp,
            downstream_erasure_codec: parameters.downstream_erasure_codec,
            upstream_erasure_codec: parameters.upstream_erasure_codec,
            downstream_heartbeat: parameters.downstream_heartbeat,
            upstream_heartbeat: parameters.upstream_heartbeat,
            downstream_compression: parameters.downstream_compression,
            upstream_compression: parameters.upstream_compression,
            flow_control: parameters.flow_control,
        }
    }
}

/// Parameters for configuring the network connection.
pub struct NetworkParameters {
    /// Set this flag to non-zero to connect to SpatialOS using the externally-visible IP address. This
    /// flag must be set when connecting externally (i.e. from outside the cloud) to a cloud
    /// deployment.
    pub use_external_ip: u8,
    /// Type of network connection to use when connecting to SpatialOS, defined in
    /// NetworkConnectionType.
    pub connection_type: NetworkConnectionType,
    /// (deprecated) Parameters used if the WORKER_NETWORK_CONNECTION_TYPE_RAKNET flag is set.
    pub raknet: Worker_RakNetNetworkParameters,
    /// (deprecated) Parameters used if the WORKER_NETWORK_CONNECTION_TYPE_TCP flag is set.
    pub tcp: Worker_TcpNetworkParameters,
    /// (deprecated) Parameters used if the WORKER_NETWORK_CONNECTION_TYPE_KCP flag is set.
    pub kcp: Worker_KcpNetworkParameters,
    /// Parameters used if the WORKER_NETWORK_CONNECTION_TYPE_MODULAR_KCP flag is set.
    pub modular_kcp: ModularKcpNetworkParameters,
    /// Parameters used if the WORKER_NETWORK_CONNECTION_TYPE_MODULAR_TCP flag is set.
    pub modular_tcp: Worker_ModularTcpNetworkParameters,
    /// Timeout for the connection to SpatialOS to be established.
    pub connection_timeout_millis: u64,
    /// Default timeout for worker commands if one is not specified when command is sent.
    pub default_command_timeout_millis: u32,
}

impl From<Worker_NetworkParameters> for NetworkParameters {
    fn from(parameters: Worker_NetworkParameters) -> Self {
        Self {
            use_external_ip: parameters.use_external_ip,
            connection_type: NetworkConnectionType::from(parameters.connection_type),
            raknet: parameters.raknet,
            tcp: parameters.tcp,
            kcp: parameters.kcp,
            modular_kcp: ModularKcpNetworkParameters::from(parameters.modular_kcp),
            modular_tcp: parameters.modular_tcp,
            connection_timeout_millis: parameters.connection_timeout_millis,
            default_command_timeout_millis: parameters.default_command_timeout_millis,
        }
    }
}

impl From<NetworkParameters> for Worker_NetworkParameters {
    fn from(parameters: NetworkParameters) -> Self {
        Self {
            use_external_ip: parameters.use_external_ip,
            connection_type: parameters.connection_type.into(),
            raknet: parameters.raknet,
            tcp: parameters.tcp,
            kcp: parameters.kcp,
            modular_kcp: parameters.modular_kcp.into(),
            modular_tcp: parameters.modular_tcp,
            connection_timeout_millis: parameters.connection_timeout_millis,
            default_command_timeout_millis: parameters.default_command_timeout_millis,
        }
    }
}

/// Parameters for creating a Worker_Connection and connecting to SpatialOS.
pub struct ConnectionParameters {
    /// Worker type (platform).
    pub worker_type: String,
    /// Network parameters.
    pub network: NetworkParameters,
    /// Number of messages that can be stored on the send queue. When the send queue is full, calls to
    /// Worker_Connection_Send functions can block.
    pub send_queue_capacity: u32,
    /// Number of messages that can be stored on the receive queue. When the receive queue is full,
    /// SpatialOS can apply QoS and drop messages to the worker.
    pub receive_queue_capacity: u32,
    /// Number of messages logged by the SDK that can be stored in the log message queue. When the log
    /// message queue is full, messages logged by the SDK can be dropped.
    pub log_message_queue_capacity: u32,
    /// The Connection tracks several internal metrics, such as send and receive queue statistics. This
    /// parameter controls how frequently the Connection will return a MetricsOp reporting its built-in
    /// metrics. If set to zero, this functionality is disabled.
    pub built_in_metrics_report_period_millis: u32,
    /// (Deprecated) Parameters for configuring legacy protocol logging parameters.
    pub protocol_logging: Worker_ProtocolLoggingParameters,
    /// (Deprecated) Whether to enable legacy protocol logging at startup.
    pub enable_protocol_logging_at_startup: u8,
    /// Number of logsinks configured.
    pub logsink_count: u32,
    /// Array of logsinks that receive filtered log messages from the SDK.
    pub logsinks: *const Worker_LogsinkParameters,
    /// Whether to enable all logsinks at startup. Note that this is automatically true if
    /// enable_protocol_logging_at_startup is set to true.
    pub enable_logging_at_startup: u8,
    /// Whether to enable dynamic components.
    /// If this field is true, add and remove component ops are emitted on authority change. These ops,
    /// like all add and remove component ops, must be treated in an idempotent way (i.e. they replace
    /// any existing value on the worker for the component).
    pub enable_dynamic_components: u8,
    /// Parameters for configuring thread affinity.
    pub thread_affinity: Worker_ThreadAffinityParameters,
    /// Number of component vtables.
    pub component_vtable_count: u32,
    /// Component vtable for each component that the connection will deal with.
    pub component_vtables: *const Worker_ComponentVtable,
    /// Default vtable used when a component is not registered. Only used if not NULL.
    pub default_component_vtable: *const Worker_ComponentVtable,
}

impl From<Worker_ConnectionParameters> for ConnectionParameters {
    fn from(parameters: Worker_ConnectionParameters) -> Self {
        let worker_type = unsafe { CStr::from_ptr(parameters.worker_type) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            network: NetworkParameters::from(parameters.network),
            send_queue_capacity: parameters.send_queue_capacity,
            receive_queue_capacity: parameters.receive_queue_capacity,
            log_message_queue_capacity: parameters.log_message_queue_capacity,
            built_in_metrics_report_period_millis: parameters.built_in_metrics_report_period_millis,
            protocol_logging: parameters.protocol_logging,
            enable_protocol_logging_at_startup: parameters.enable_protocol_logging_at_startup,
            logsink_count: parameters.logsink_count,
            logsinks: parameters.logsinks,
            enable_logging_at_startup: parameters.enable_logging_at_startup,
            enable_dynamic_components: parameters.enable_dynamic_components,
            thread_affinity: parameters.thread_affinity,
            component_vtable_count: parameters.component_vtable_count,
            component_vtables: parameters.component_vtables,
            default_component_vtable: parameters.default_component_vtable,
            worker_type,
        }
    }
}

impl From<ConnectionParameters> for Worker_ConnectionParameters {
    fn from(parameters: ConnectionParameters) -> Self {
        let worker_type = CString::new(parameters.worker_type).unwrap();
        Self {
            network: parameters.network.into(),
            send_queue_capacity: parameters.send_queue_capacity,
            receive_queue_capacity: parameters.receive_queue_capacity,
            log_message_queue_capacity: parameters.log_message_queue_capacity,
            built_in_metrics_report_period_millis: parameters.built_in_metrics_report_period_millis,
            protocol_logging: parameters.protocol_logging,
            enable_protocol_logging_at_startup: parameters.enable_protocol_logging_at_startup,
            logsink_count: parameters.logsink_count,
            logsinks: parameters.logsinks,
            enable_logging_at_startup: parameters.enable_logging_at_startup,
            enable_dynamic_components: parameters.enable_dynamic_components,
            thread_affinity: parameters.thread_affinity,
            component_vtable_count: parameters.component_vtable_count,
            component_vtables: parameters.component_vtables,
            default_component_vtable: parameters.default_component_vtable,
            worker_type: worker_type.into_raw() as *const c_char,
        }
    }
}

impl Default for ConnectionParameters {
    /// Returns a new ConnectionParameters with default values set.
    fn default() -> Self {
        ConnectionParameters::from(unsafe { Worker_DefaultConnectionParameters() })
    }
}

pub struct ConnectionFuture {
    inner: *mut Worker_ConnectionFuture,
}

impl Drop for ConnectionFuture {
    fn drop(&mut self) {
        unsafe { Worker_ConnectionFuture_Destroy(self.inner) }
    }
}

impl ConnectionFuture {
    /// Connect to a SpatialOS deployment via a receptionist. This is the flow used to connect a managed
    /// worker running in the cloud alongside the deployment, and also to connect any local worker to a
    /// (local or remote) deployment via a locally-running receptionist.
    ///
    /// The hostname and port would typically be provided by SpatialOS on the command-line, if this is a
    /// managed worker on the cloud, or otherwise be predetermined (e.g. localhost:7777 for the default
    /// receptionist of a locally-running deployment).
    ///
    /// Returns a Worker_ConnectionFuture that can be used to obtain a Worker_Connection
    /// by using Worker_ConnectionFuture_Get. Caller is responsible for destroying it when no
    /// longer needed by using Worker_ConnectionFuture_Destroy.
    pub fn connect_async<S: AsRef<str>>(
        hostname: S,
        port: u16,
        worker_id: S,
        params: ConnectionParameters,
    ) -> Self {
        let hostname = CString::new(hostname.as_ref()).unwrap();
        let worker_id = CString::new(worker_id.as_ref()).unwrap();
        unsafe {
            Self {
                inner: Worker_ConnectAsync(
                    hostname.as_ptr() as *const c_char,
                    port,
                    worker_id.as_ptr() as *const c_char,
                    &params.into() as *const Worker_ConnectionParameters,
                ),
            }
        }
    }

    /// Gets the result of a ConnectionFuture, waiting for up to *timeout_millis to
    /// become available (or forever if timeout_millis is None). It returns None in case of a timeout.
    ///
    /// It is an error to call this method again once it has succeeded (e.g. not timed out) once.
    pub fn get(&mut self, timeout_millis: Option<u32>) -> Option<Connection> {
        let connection = if let Some(timeout_millis) = timeout_millis {
            unsafe { Worker_ConnectionFuture_Get(self.inner, &timeout_millis as *const u32) }
        } else {
            unsafe { Worker_ConnectionFuture_Get(self.inner, std::ptr::null()) }
        };
        if connection.is_null() {
            None
        } else {
            Some(Connection::from(connection))
        }
    }
}

pub struct Connection {
    inner: *mut Worker_Connection,
}

impl Connection {
    /// Sends a log message from the worker to SpatialOS.
    pub fn send_log_message(&mut self, log_message: LogMessage) {
        unsafe {
            Worker_Connection_SendLogMessage(
                self.inner,
                &Worker_LogMessage::from(log_message) as *const Worker_LogMessage,
            )
        }
    }

    /// Retrieves the list of operations that have occurred since the last call to this function.
    ///
    /// If timeout_millis is non-zero, the function will block until there is at least one operation to
    /// return, or the timeout has been exceeded. If the timeout is exceeded, an empty list will be
    /// returned.
    ///
    /// If timeout_millis is zero the function is non-blocking.
    ///
    /// It is the caller's responsibility to destroy the returned Worker_OpList with the
    /// Worker_OpList_Destroy function.
    ///
    /// Note: All data contained within the op-list (such as component data or updates) is owned by
    /// Worker_OpList, and must not be passed directly to another function in the SDK, such as
    /// Worker_Connection_SendComponentUpdate, without copying the data first. Otherwise, a double free
    /// could occur.
    pub fn get_op_list(&mut self, timeout_millis: u32) -> OpList {
        OpList::from(unsafe { Worker_Connection_GetOpList(self.inner, timeout_millis) })
    }

    /// Queries SpatialOS for entity data.
    ///
    /// Returns RequestId -1 if the query constraint or result type are not valid.
    pub fn send_entity_query_request(
        &mut self,
        entity_query: EntityQuery,
        timeout_millis: Option<u32>,
    ) -> RequestId {
        let query: Worker_EntityQuery = entity_query.into();
        if let Some(timeout_millis) = timeout_millis {
            unsafe {
                Worker_Connection_SendEntityQueryRequest(
                    self.inner,
                    &query as *const Worker_EntityQuery,
                    &timeout_millis as *const u32,
                )
            }
        } else {
            unsafe {
                Worker_Connection_SendEntityQueryRequest(
                    self.inner,
                    &query as *const Worker_EntityQuery,
                    std::ptr::null(),
                )
            }
        }
    }
}

impl From<*mut Worker_Connection> for Connection {
    fn from(connection: *mut Worker_Connection) -> Self {
        Self { inner: connection }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { Worker_Connection_Destroy(self.inner) }
    }
}
