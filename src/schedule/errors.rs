use std::fmt;

use crate::Id;

#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleError {
    /// Task ID is already present in the schedule
    DuplicateTaskId(Id),
    /// A time value was NaN, which is not allowed
    NaNTime,
    /// New interval overlaps with an existing interval
    OverlapsExisting { new_id: Id, existing_id: Id },
    /// Task ID was not found in the schedule
    TaskNotFound(Id),
}

impl fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduleError::DuplicateTaskId(id) => {
                write!(f, "Task ID {id} already exists in schedule")
            }
            ScheduleError::NaNTime => {
                write!(f, "Time value cannot be NaN")
            }
            ScheduleError::OverlapsExisting {
                new_id,
                existing_id,
            } => {
                write!(f, "Task {new_id} overlaps with existing task {existing_id}")
            }
            ScheduleError::TaskNotFound(id) => {
                write!(f, "Task ID {id} not found in schedule")
            }
        }
    }
}

impl std::error::Error for ScheduleError {}
