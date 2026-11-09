use std::result::Result as StdResult;

use super::core::AcrithisError;

pub trait ResultExt<T, E> {
    fn map_usage<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_config<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_io<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_database<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_hash<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_graph<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_validation<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_permission<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_not_found<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
    fn map_conflict<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError>;
}

// Assisted by: DeepSeek
// Author: Givi Tsvariani
// Type was written by me, the API for the impl was written by me, DeepSeek was used to fill in the
// functions
impl<T, E : std::fmt::Display> ResultExt<T, E> for StdResult<T, E> {
    fn map_usage<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::usage(message).with_cause(e))
    }

    fn map_config<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::config(message).with_cause(e))
    }

    fn map_io<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::io(message).with_cause(e))
    }

    fn map_database<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::database(message).with_cause(e))
    }

    fn map_hash<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::hash(message).with_cause(e))
    }

    fn map_graph<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::graph(message).with_cause(e))
    }

    fn map_validation<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::validation(message).with_cause(e))
    }

    fn map_permission<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::permission(message).with_cause(e))
    }

    fn map_not_found<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::not_found(message).with_cause(e))
    }

    fn map_conflict<M : Into<String>>(self, message : M) -> StdResult<T, AcrithisError> {
        self.map_err(|e : E| AcrithisError::conflict(message).with_cause(e))
    }
}
