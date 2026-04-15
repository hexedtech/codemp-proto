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

/// buffer synchronisation protocol types and procedures
pub mod buffer {
	tonic::include_proto!("buffer");

	impl BufferPath {
		pub fn standardize(path: &str) -> String {
			// TODO canonicalize makes it absolute and fails if path doesn't exist
			//      we need some other way!
			path.to_string()
		}
	}

	impl<T: AsRef<str>> From<T> for BufferPath {
		fn from(value: T) -> Self {
			Self { path: BufferPath::standardize(value.as_ref()) }
		}
	}

	impl std::fmt::Display for BufferPath {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", BufferPath::standardize(&self.path))
		}
	}
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
