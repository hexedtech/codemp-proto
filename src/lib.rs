#[allow(non_snake_case)]
pub mod common {
	tonic::include_proto!("common");

	impl From<uuid::Uuid> for Identity {
		fn from(id: uuid::Uuid) -> Self {
			Identity { id: id.to_string() }
		}
	}
}

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

pub mod buffer {
	tonic::include_proto!("buffer");
}

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

pub mod workspace {
	tonic::include_proto!("workspace");
}

pub mod auth {
	tonic::include_proto!("auth");
}
