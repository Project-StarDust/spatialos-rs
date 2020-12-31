use spatialos_sys::*;

use crate::worker::EntityId;
use crate::worker::LogLevel;
use std::ffi::CStr;
use std::ffi::CString;

/// Parameters for sending a log message to SpatialOS.
pub struct LogMessage {
    /// The severity of the log message; defined in the LogLevel enumeration.
    pub level: LogLevel,
    /// The name of the logger.
    pub logger_name: String,
    /// The full log message.
    pub message: String,
    /// The ID of the entity this message relates to, or NULL for none.
    pub entity_id: Option<EntityId>,
}

impl From<Worker_LogMessage> for LogMessage {
    fn from(log_message: Worker_LogMessage) -> Self {
        let logger_name = unsafe { CStr::from_ptr(log_message.logger_name) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        let message = unsafe { CStr::from_ptr(log_message.message) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            level: log_message.level.into(),
            entity_id: if log_message.entity_id.is_null() {
                None
            } else {
                Some(unsafe { *log_message.entity_id })
            },
            logger_name,
            message,
        }
    }
}

impl From<LogMessage> for Worker_LogMessage {
    fn from(log_message: LogMessage) -> Self {
        let message = CString::new(log_message.message).unwrap();
        let logger_name = CString::new(log_message.logger_name).unwrap();
        if let Some(entity_id) = log_message.entity_id {
            Self {
                level: log_message.level.into(),
                message: message.into_raw(),
                logger_name: logger_name.into_raw(),
                entity_id: &entity_id as *const EntityId,
            }
        } else {
            Self {
                level: log_message.level.into(),
                message: message.into_raw(),
                logger_name: logger_name.into_raw(),
                entity_id: std::ptr::null(),
            }
        }
    }
}

impl LogMessage {
    pub fn new<S: AsRef<str>>(
        level: LogLevel,
        logger_name: S,
        message: S,
        entity_id: Option<EntityId>,
    ) -> Self {
        Self {
            level,
            logger_name: logger_name.as_ref().to_string(),
            message: message.as_ref().to_string(),
            entity_id,
        }
    }
}
