use crate::{
    pkg::{PackageError, PackageErrorKind},
    ErrorCommons, RuntimeError,
};
use min_sqlite3_sys::prelude::MinSqliteWrapperError;

#[macro_export]
macro_rules! try_bind_val {
    ($sql: expr, $c_index: expr, $val: expr) => {
        let status = $sql.bind_val($c_index, $val);
        if status != min_sqlite3_sys::prelude::SqlitePrimaryResult::Ok {
            $sql.kill();
            return Err(ehandle::db::SqlErrorKind::FailedExecuting(Some(format!(
                "Raised 'SqlitePrimaryResult::{:?}' error status on binding parameter to SQL query.",
                status
            )))
            .throw()
            .into());
        }
    };
}

#[macro_export]
macro_rules! try_execute_prepared {
    ($sql: expr, $err: expr) => {
        match $sql.execute_prepared() {
            min_sqlite3_sys::prelude::PreparedStatementStatus::FoundRow => {
                min_sqlite3_sys::prelude::PreparedStatementStatus::FoundRow
            }
            min_sqlite3_sys::prelude::PreparedStatementStatus::Done => {
                min_sqlite3_sys::prelude::PreparedStatementStatus::Done
            }
            _ => {
                $sql.kill();
                return Err(ehandle::db::SqlErrorKind::FailedExecuting($err)
                    .throw()
                    .into());
            }
        }
    };
}

#[macro_export]
macro_rules! try_execute {
    ($db: expr, $statement: expr, $err: expr) => {
        match $db.execute($statement, crate::SQL_NO_CALLBACK_FN)? {
            min_sqlite3_sys::prelude::SqlitePrimaryResult::Ok => SqlitePrimaryResult::Ok,
            _ => {
                return Err(ehandle::db::SqlErrorKind::FailedExecuting($err)
                    .throw()
                    .into());
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum MigrationErrorKind {
    VersionCouldNotSet(Option<String>),
    SqliteWrapperError(Option<String>),
    FailedExecuting(Option<String>),
}

impl ErrorCommons<MigrationError> for MigrationErrorKind {
    #[inline(always)]
    fn as_str(&self) -> &str {
        match self {
            Self::VersionCouldNotSet(_) => "VersionCouldNotSet",
            Self::FailedExecuting(_) => "FailedExecuting",
            Self::SqliteWrapperError(_) => "SqliteWrapperError",
        }
    }

    #[inline(always)]
    fn throw(&self) -> MigrationError {
        match self {
            Self::VersionCouldNotSet(ref err) => MigrationError {
                kind: self.clone(),
                reason: err
                    .as_ref()
                    .unwrap_or(&String::from("Migration version could not set."))
                    .to_owned(),
            },
            Self::SqliteWrapperError(ref err) => MigrationError {
                kind: self.clone(),
                reason: err
                    .as_ref()
                    .unwrap_or(&String::from(
                        "An error has been occur from Sqlite wrapper library.",
                    ))
                    .to_owned(),
            },
            Self::FailedExecuting(ref err) => MigrationError {
                kind: self.clone(),
                reason: err
                    .as_ref()
                    .unwrap_or(&String::from(
                        "Sqlite has returned the error status as a response of the SQL query.",
                    ))
                    .to_owned(),
            },
        }
    }
}

#[derive(Debug)]
pub struct MigrationError {
    pub kind: MigrationErrorKind,
    pub reason: String,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum SqlErrorKind {
    FailedExecuting(Option<String>),
    FailedParameterBinding(Option<String>),
}

#[derive(Debug)]
pub struct SqlError {
    pub kind: SqlErrorKind,
    pub reason: String,
}

impl ErrorCommons<SqlError> for SqlErrorKind {
    #[inline(always)]
    fn as_str(&self) -> &str {
        match self {
            Self::FailedExecuting(_) => "FailedExecuting",
            Self::FailedParameterBinding(_) => "FailedParameterBinding",
        }
    }

    #[inline(always)]
    fn throw(&self) -> SqlError {
        match self {
            Self::FailedExecuting(ref err) => SqlError {
                kind: self.clone(),
                reason: err
                    .as_ref()
                    .unwrap_or(&String::from(
                        "Sqlite has returned the error status as a response of the SQL query.",
                    ))
                    .to_owned(),
            },
            Self::FailedParameterBinding(ref err) => SqlError {
                kind: self.clone(),
                reason: err
                    .as_ref()
                    .unwrap_or(&String::from(
                        "SQLite returned raised an error on binding parameter to SQL query.",
                    ))
                    .to_owned(),
            },
        }
    }
}

impl From<MigrationError> for RuntimeError {
    #[inline(always)]
    fn from(error: MigrationError) -> Self {
        RuntimeError {
            kind: error.kind.as_str().to_string(),
            reason: error.reason,
        }
    }
}

impl From<MinSqliteWrapperError<'_>> for RuntimeError {
    #[inline(always)]
    fn from(error: MinSqliteWrapperError) -> Self {
        RuntimeError {
            kind: error.kind.to_string(),
            reason: error.reason,
        }
    }
}

impl From<MinSqliteWrapperError<'_>> for MigrationError {
    #[inline(always)]
    fn from(error: MinSqliteWrapperError) -> Self {
        MigrationErrorKind::SqliteWrapperError(Some(error.reason)).throw()
    }
}

impl From<SqlError> for RuntimeError {
    #[inline(always)]
    fn from(error: SqlError) -> Self {
        RuntimeError {
            kind: error.kind.as_str().to_string(),
            reason: error.reason,
        }
    }
}

impl From<SqlError> for PackageError {
    #[inline(always)]
    fn from(error: SqlError) -> Self {
        PackageErrorKind::InstallationFailed(Some(error.reason)).throw()
    }
}

impl From<SqlError> for MigrationError {
    #[inline(always)]
    fn from(error: SqlError) -> Self {
        MigrationErrorKind::FailedExecuting(Some(error.reason)).throw()
    }
}

impl From<MinSqliteWrapperError<'_>> for SqlError {
    #[inline(always)]
    fn from(error: MinSqliteWrapperError) -> Self {
        SqlErrorKind::FailedExecuting(Some(error.reason)).throw()
    }
}
