use sqlx::{Postgres, Pool};

use crate::errors::ApiError;

pub type DbPool = Pool<Postgres>;

pub type APIResponse<T> = Result<T, ApiError>;