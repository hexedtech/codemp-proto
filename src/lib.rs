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

	impl From<uuid::Uuid> for Identity {
		fn from(id: uuid::Uuid) -> Self {
			let (hi, lo) = id.as_u64_pair();
			Identity { hi, lo }
		}
	}

	impl From<&uuid::Uuid> for Identity {
		fn from(id: &uuid::Uuid) -> Self {
			let (hi, lo) = id.as_u64_pair();
			Identity { hi, lo }
		}
	}

	impl From<Identity> for uuid::Uuid {
		fn from(value: Identity) -> Self {
			uuid::Uuid::from_u64_pair(value.hi, value.lo)
		}
	}

	impl From<&Identity> for uuid::Uuid {
		fn from(value: &Identity) -> Self {
			uuid::Uuid::from_u64_pair(value.hi, value.lo)
		}
	}

	impl Identity {
		pub fn uuid(&self) -> uuid::Uuid {
			uuid::Uuid::from(self)
		}
	}
}

/// filetree related types
pub mod files {
	tonic::include_proto!("files");

	impl From<String> for BufferNode {
		fn from(value: String) -> Self {
			BufferNode { path: value }
		}
	}

	impl From<&str> for BufferNode {
		fn from(value: &str) -> Self {
			BufferNode {
				path: value.to_string(),
			}
		}
	}

	impl From<BufferNode> for String {
		fn from(value: BufferNode) -> Self {
			value.path
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
