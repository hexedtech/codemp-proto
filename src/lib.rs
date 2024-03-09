#[allow(non_snake_case)]
pub mod proto {
	pub mod common {
		tonic::include_proto!("common");

		impl From<uuid::Uuid> for Identity {
			fn from(id: uuid::Uuid) -> Self {
				Identity { id: id.to_string() }
			}
		}

		impl From<&uuid::Uuid> for Identity {
			fn from(id: &uuid::Uuid) -> Self {
				Identity { id: id.to_string() }
			}
		}

		impl From<Identity> for uuid::Uuid {
			fn from(value: Identity) -> Self {
				uuid::Uuid::parse_str(&value.id).expect("invalid uuid in identity")
			}
		}

		impl From<&Identity> for uuid::Uuid {
			fn from(value: &Identity) -> Self {
				uuid::Uuid::parse_str(&value.id).expect("invalid uuid in identity")
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
	}
	pub mod workspace {
		tonic::include_proto!("workspace");
	}
	pub mod auth {
		tonic::include_proto!("auth");
	}
}
