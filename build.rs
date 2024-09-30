fn main() -> Result<(), Box<dyn std::error::Error>> {
	let server = {
		#[cfg(feature = "server")] { true }
		#[cfg(not(feature = "server"))] { false }
	};

	let client = {
		#[cfg(feature = "client")] { true }
		#[cfg(not(feature = "client"))] { false }
	};

	let transport = {
		#[cfg(any(feature = "server", feature = "client"))] { true }
		#[cfg(not(any(feature = "server", feature = "client")))] { false }
	};

	Ok
		(tonic_build::configure()
			.build_server(server)
			.build_client(client)
			.build_transport(transport)
			.compile_protos(
				&[
					"proto/common.proto",
					"proto/cursor.proto",
					"proto/files.proto",
					"proto/auth.proto",
					"proto/session.proto",
					"proto/workspace.proto",
					"proto/buffer.proto",
				],
				&["proto"],
			)?)
}
