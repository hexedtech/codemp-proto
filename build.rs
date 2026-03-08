fn main() -> Result<(), Box<dyn std::error::Error>> {
	let server = cfg!(feature = "server");
	let client = cfg!(feature = "client");
	let transport = cfg!(any(feature = "server", feature = "client"));

	#[allow(unused_mut)]
	let mut builder = tonic_prost_build::configure();

	#[cfg(feature = "serde")]
	{
		builder = builder
			.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
	}

	Ok(builder
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
