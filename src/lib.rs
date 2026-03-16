//! # CodeMP Protocol - cooperative development
//! `codemp-proto` is the gRPC protocol specification powering [`codemp`](https://code.mp).
//!
//! This is built on top of [tonic] and provides both clientside and serverside service
//! implementations.
#![doc(html_logo_url = "https://code.mp/logo-round.png")]

/// common types across services
#[allow(non_snake_case)]
pub mod common {
	tonic::include_proto!("common");
}

/// filetree related types
pub mod files {
	use std::str::FromStr;

	tonic::include_proto!("files");

	impl BufferPath {
		pub fn standardize(path: &str) -> String {
			let path_raw = std::path::PathBuf::from_str(path).expect("infallible");
			match path_raw.canonicalize() {
				Ok(p) => p.display().to_string(),
				Err(_e) => {
					// TODO log it? we don't have tracing here
					path_raw.display().to_string()
				},
			}
		}
	}

	impl<T: AsRef<str>> From<T> for BufferPath {
		fn from(value: T) -> Self {
			Self { path: BufferPath::standardize(value.as_ref()) }
		}
	}

	impl From<BufferPath> for String {
		fn from(value: BufferPath) -> Self {
			BufferPath::standardize(&value.path)
		}
	}
}

/// buffer synchronisation protocol types and procedures
pub mod buffer {
	tonic::include_proto!("buffer");
}

/// cursor position protocol types and procedures
pub mod cursor {
	tonic::include_proto!("cursor");

	impl From<RowCol> for (i32, i32) {
		fn from(pos: RowCol) -> (i32, i32) {
			(pos.row, pos.col)
		}
	}

	impl From<(i32, i32)> for RowCol {
		fn from((row, col): (i32, i32)) -> Self {
			RowCol { row, col }
		}
	}

	impl PartialOrd for RowCol {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			match self.row.partial_cmp(&other.row) {
				Some(core::cmp::Ordering::Equal) => {}
				ord => return ord,
			}
			self.col.partial_cmp(&other.col)
		}
	}
}

/// workspace state protocol types and procedures
pub mod workspace {
	tonic::include_proto!("workspace");
}

/// session management protocol types and procedures
pub mod session {
	tonic::include_proto!("session");
}

/// authentication and authorization protocol types and procedures
pub mod auth {
	tonic::include_proto!("auth");
}
