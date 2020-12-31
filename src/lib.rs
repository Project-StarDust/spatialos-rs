pub mod schema;
pub mod worker;

pub mod private_exports {
    pub use spatialos_sys::*;
}

pub use spatialos_sys::{
    Ngrpc_Buffer, Ngrpc_Call, Ngrpc_CallMetadata, Ngrpc_CallParameters, Ngrpc_Client, Ngrpc_Create,
    Ngrpc_Destroy, Ngrpc_DestroyCall, Ngrpc_FinishCall, Ngrpc_GetImprobableRootCertificate,
    Ngrpc_GetStatus, Ngrpc_MakeCall, Ngrpc_Parameters, Ngrpc_Receive, Ngrpc_Send, Ngrpc_Status,
    Ngrpc_StatusCode, Ngrpc_StatusCodeToString, Ngrpc_TlsParameters, Schema_AddBoolList,
    Schema_AddComponentUpdateClearedField, Schema_AddEntityIdList, Schema_AddFixed32List,
    Schema_AddFixed64List, Schema_AddFloatList, Schema_AddInt32List, Schema_AddInt64List,
    Schema_AddSfixed32List, Schema_AddSfixed64List, Schema_AddSint32List, Schema_AddSint64List,
    Schema_AddUint64List, Schema_AllocateObject, Schema_ApplyComponentUpdateToData, Schema_Bundle,
    Schema_Bundle_Destroy, Schema_Bundle_GetError, Schema_Bundle_Load, Schema_Clear,
    Schema_ClearComponentUpdateClearedFields, Schema_ClearField, Schema_ComponentId,
    Schema_ConvertComponentDataIntoUpdate, Schema_CopyCommandRequest, Schema_CopyCommandResponse,
    Schema_CopyComponentData, Schema_CopyComponentUpdate, Schema_CopyGenericData,
    Schema_CreateCommandRequest, Schema_CreateCommandResponse, Schema_CreateGenericData,
    Schema_DestroyCommandRequest, Schema_DestroyCommandResponse, Schema_DestroyComponentData,
    Schema_DestroyComponentUpdate, Schema_DestroyGenericData, Schema_EntityId,
    Schema_ErrorCallback, Schema_FieldId, Schema_GenericData, Schema_GetBoolList,
    Schema_GetCommandRequestObject, Schema_GetCommandResponseObject,
    Schema_GetComponentUpdateClearedFieldCount, Schema_GetComponentUpdateClearedFieldList,
    Schema_GetComponentUpdateEvents, Schema_GetComponentUpdateFields, Schema_GetEntityIdCount,
    Schema_GetEntityIdList, Schema_GetError, Schema_GetFixed32Count, Schema_GetFixed32List,
    Schema_GetFixed64Count, Schema_GetFixed64List, Schema_GetFloatList,
    Schema_GetGenericDataObject, Schema_GetInt32Count, Schema_GetInt32List, Schema_GetInt64List,
    Schema_GetSfixed32Count, Schema_GetSfixed32List, Schema_GetSfixed64Count,
    Schema_GetSfixed64List, Schema_GetSint32Count, Schema_GetSint32List, Schema_GetSint64Count,
    Schema_GetSint64List, Schema_GetUint64Count, Schema_GetUint64List,
    Schema_GetUniqueFieldIdCount, Schema_GetUniqueFieldIds, Schema_GetWriteBufferLength,
    Schema_IndexBool, Schema_IndexComponentUpdateClearedField, Schema_IndexDouble,
    Schema_IndexEntityId, Schema_IndexFixed32, Schema_IndexFixed64, Schema_IndexFloat,
    Schema_IndexInt32, Schema_IndexInt64, Schema_IndexSfixed32, Schema_IndexSfixed64,
    Schema_IndexSint32, Schema_IndexSint64, Schema_IndexUint32, Schema_IndexUint64,
    Schema_IsComponentUpdateFieldCleared, Schema_Json, Schema_JsonParameters, Schema_Json_Destroy,
    Schema_Json_DumpCommandRequest, Schema_Json_DumpCommandResponse, Schema_Json_DumpComponentData,
    Schema_Json_DumpComponentUpdate, Schema_Json_DumpObject, Schema_Json_GetJsonString,
    Schema_Json_GetLastError, Schema_Json_GetLastWarning, Schema_Json_LoadCommandRequest,
    Schema_Json_LoadCommandResponse, Schema_Json_LoadComponentData,
    Schema_Json_LoadComponentUpdate, Schema_Json_LoadObject, Schema_MergeComponentUpdateIntoUpdate,
    Schema_MergeFromBuffer, Schema_SerializeToBuffer, Schema_ShallowCopy, Schema_ShallowCopyField,
    Worker_AcquireCommandRequest, Worker_AcquireCommandResponse, Worker_AcquireComponentData,
    Worker_AcquireComponentUpdate, Worker_AllocateFunction,
    Worker_Alpha_CreateDevelopmentLoginTokensAsync,
    Worker_Alpha_CreateDevelopmentPlayerIdentityTokenAsync, Worker_Alpha_LoginTokenDetails,
    Worker_Alpha_LoginTokensRequest, Worker_Alpha_LoginTokensResponse,
    Worker_Alpha_LoginTokensResponseCallback, Worker_Alpha_LoginTokensResponseFuture,
    Worker_Alpha_LoginTokensResponseFuture_Destroy, Worker_Alpha_LoginTokensResponseFuture_Get,
    Worker_Alpha_PlayerIdentityTokenRequest, Worker_Alpha_PlayerIdentityTokenResponse,
    Worker_Alpha_PlayerIdentityTokenResponseCallback,
    Worker_Alpha_PlayerIdentityTokenResponseFuture,
    Worker_Alpha_PlayerIdentityTokenResponseFuture_Destroy,
    Worker_Alpha_PlayerIdentityTokenResponseFuture_Get, Worker_Alpha_SetAllocator,
    Worker_ApiVersion, Worker_ApiVersionStr, Worker_CommandParameters, Worker_CommandRequest,
    Worker_CommandRequestCopy, Worker_CommandRequestDeserialize, Worker_CommandRequestFree,
    Worker_CommandRequestOp, Worker_CommandRequestSerialize, Worker_CommandResponse,
    Worker_CommandResponseCopy, Worker_CommandResponseDeserialize, Worker_CommandResponseFree,
    Worker_CommandResponseSerialize, Worker_ComponentDataCopy, Worker_ComponentDataDeserialize,
    Worker_ComponentDataFree, Worker_ComponentDataSerialize, Worker_ComponentUpdate,
    Worker_ComponentUpdateCopy, Worker_ComponentUpdateDeserialize, Worker_ComponentUpdateFree,
    Worker_ComponentUpdateHandle, Worker_ComponentUpdateLoopback, Worker_ComponentUpdateSerialize,
    Worker_CompressionParameters, Worker_ConnectionFuture, Worker_ConnectionStatus,
    Worker_Connection_Alpha_Flush, Worker_Connection_DisableLogging,
    Worker_Connection_EnableLogging, Worker_Connection_GetConnectionStatusCode,
    Worker_Connection_GetConnectionStatusDetailString, Worker_Connection_GetWorkerAttributes,
    Worker_Connection_GetWorkerFlag, Worker_Connection_GetWorkerId, Worker_Connection_IsConnected,
    Worker_Connection_SendAddComponent, Worker_Connection_SendAuthorityLossImminentAcknowledgement,
    Worker_Connection_SendCommandFailure, Worker_Connection_SendCommandRequest,
    Worker_Connection_SendCommandResponse, Worker_Connection_SendComponentInterest,
    Worker_Connection_SendComponentUpdate, Worker_Connection_SendCreateEntityRequest,
    Worker_Connection_SendDeleteEntityRequest, Worker_Connection_SendMetrics,
    Worker_Connection_SendRemoveComponent, Worker_Connection_SendReserveEntityIdsRequest,
    Worker_Connection_SetProtocolLoggingEnabled, Worker_DeallocateFunction, Worker_Deployment,
    Worker_DeploymentList, Worker_DeploymentListCallback, Worker_DeploymentListFuture,
    Worker_DeploymentListFuture_Destroy, Worker_DeploymentListFuture_Get,
    Worker_ErasureCodecParameters, Worker_FlowControlParameters, Worker_GetWorkerFlagCallback,
    Worker_HeartbeatParameters, Worker_InterestOverride, Worker_KcpNetworkParameters,
    Worker_KcpTransportParameters, Worker_Locator, Worker_LocatorCredentialsTypes,
    Worker_LocatorParameters, Worker_Locator_ConnectAndQueueAsync, Worker_Locator_ConnectAsync,
    Worker_Locator_Create, Worker_Locator_Destroy, Worker_Locator_GetDeploymentListAsync,
    Worker_LogCallback, Worker_LogCallbackParameters, Worker_LogCategory, Worker_LogData,
    Worker_LogFilterCallback, Worker_LogFilterParameters, Worker_LoginTokenCredentials,
    Worker_LogsinkParameters, Worker_LogsinkType, Worker_ModularTcpNetworkParameters,
    Worker_Op_Union, Worker_PlayerIdentityCredentials, Worker_ProtocolLoggingParameters,
    Worker_QueueStatus, Worker_QueueStatusCallback, Worker_RakNetNetworkParameters,
    Worker_ReleaseCommandRequest, Worker_ReleaseCommandResponse, Worker_ReleaseComponentData,
    Worker_ReleaseComponentUpdate, Worker_Result, Worker_RotatingLogFileParameters,
    Worker_SnapshotInputStream, Worker_SnapshotInputStream_Create,
    Worker_SnapshotInputStream_Destroy, Worker_SnapshotInputStream_GetState,
    Worker_SnapshotInputStream_HasNext, Worker_SnapshotInputStream_ReadEntity,
    Worker_SnapshotOutputStream, Worker_SnapshotOutputStream_Create,
    Worker_SnapshotOutputStream_Destroy, Worker_SnapshotOutputStream_GetLastWarning,
    Worker_SnapshotOutputStream_GetState, Worker_SnapshotOutputStream_WriteEntity,
    Worker_SnapshotParameters, Worker_SnapshotState, Worker_SnapshotType, Worker_SteamCredentials,
    Worker_StreamState, Worker_TcpNetworkParameters, Worker_TcpTransportParameters,
    Worker_ThreadAffinityParameters, Worker_UpdateParameters,
};

#[allow(dead_code)]
pub(crate) fn mut_to_vector<T>(data: *mut T, size: isize) -> Vec<T> {
    if data.is_null() {
        Vec::new()
    } else {
        let mut datas = Vec::new();
        for index in 0..size {
            let data = unsafe {
                let data_ptr = data.offset(index);
                Box::from_raw(data_ptr)
            };
            datas.push(*data);
        }
        datas
    }
}

pub(crate) fn vector_to_owned_array<T>(mut data: Vec<T>) -> (*mut T, usize) {
    data.shrink_to_fit();
    assert!(data.len() == data.capacity());
    let ptr = data.as_mut_ptr();
    let len = data.len();
    std::mem::forget(data);
    (ptr, len)
}

pub(crate) fn const_to_vector<T: Clone>(data: *const T, size: isize) -> Vec<T> {
    if data.is_null() {
        Vec::new()
    } else {
        let mut datas = Vec::new();
        for index in 0..size {
            let data = unsafe {
                let data_ptr = data.offset(index);
                (*data_ptr).clone()
            };
            datas.push(data);
        }
        datas
    }
}
