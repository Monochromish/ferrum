use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! nerr {
  ($($arg:tt)*) => {
    napi::Error::from_reason(format!($($arg)*).to_owned())
  }
}

macro_rules! throw {
  ($($arg:tt)*) => {
    return crate::UniResult::Err(format!($($arg)*).into()).map_err(|e| e.into())
  }
}

#[macro_use]
extern crate napi_derive;

mod js;
mod library;
mod library_types;
mod sort;

mod data;
mod data_js;
mod filter;
mod page;
mod playlists;
mod sidebar_view;
mod tracks;

fn get_now_timestamp() -> i64 {
  let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(n) => n.as_millis() as i64,
    Err(_) => panic!("Generated timestamp is earlier than Unix Epoch"),
  };
  return timestamp;
}

fn sys_time_to_timestamp(sys_time: &SystemTime) -> i64 {
  let timestamp = match sys_time.duration_since(UNIX_EPOCH) {
    Ok(n) => n.as_millis() as i64,
    Err(_) => panic!("Timestamp is earlier than Unix Epoch"),
  };
  return timestamp;
}

fn str_to_option(s: String) -> Option<String> {
  match s.as_str() {
    "" => None,
    _ => Some(s),
  }
}

pub type UniResult<T> = std::result::Result<T, UniError>;

pub struct UniError {
  pub message: String,
}
impl From<String> for UniError {
  fn from(message: String) -> Self {
    Self { message }
  }
}
impl From<&str> for UniError {
  fn from(message: &str) -> Self {
    Self {
      message: message.to_string(),
    }
  }
}
impl From<UniError> for napi::Error {
  fn from(ue: UniError) -> Self {
    Self::from_reason(ue.message)
  }
}
impl From<napi::Error> for UniError {
  fn from(n_err: napi::Error) -> Self {
    UniError {
      message: n_err.reason,
    }
  }
}

fn path_to_json<J>(path: PathBuf) -> UniResult<J>
where
  J: DeserializeOwned,
{
  let file = match File::open(path) {
    Ok(f) => f,
    Err(err) => throw!("Error opening file: {}", err),
  };
  let reader = BufReader::new(file);
  let json = match serde_json::from_reader(reader) {
    Ok(json) => json,
    Err(err) => throw!("Error parsing file: {:?}", err),
  };
  Ok(json)
}
