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

	#[cfg(feature = "lua")]
	{
		builder = builder
			.message_attribute(".", "#[derive(mlua_serde_derive::LuaSerde)]");
	}

	#[cfg(feature = "java")]
	{
		builder = builder
			.message_attribute(".", "#[jni_toolbox::jclass(package = \"mp.code.proto\")]");
	}

	#[cfg(feature = "js")]
	{
		builder = builder
			.enum_attribute(".", "#[napi_derive::napi]")
			.message_attribute(".", "#[napi_derive::napi(object)]");
	}

	#[cfg(feature = "py")]
	{
		builder = builder
			.message_attribute(".", "#[pyo3::pyclass(get_all, from_py_object)]");
	}

	Ok(builder
		.build_server(server)
		.build_client(client)
		.build_transport(transport)
		.compile_protos(
			&[
				"proto/common.proto",
				"proto/cursor.proto",
				"proto/auth.proto",
				"proto/session.proto",
				"proto/workspace.proto",
				"proto/buffer.proto",
			],
			&["proto"],
		)?)
}
