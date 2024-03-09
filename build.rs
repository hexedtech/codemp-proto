fn main() -> Result<(), Box<dyn std::error::Error>> {
	tonic_build::configure()
		.compile(
			&[
				"proto/common.proto",
				"proto/cursor.proto",
				"proto/files.proto",
				"proto/auth.proto",
				"proto/workspace.proto",
				"proto/buffer.proto",
			],
			&["proto"],
		)?;
	Ok(())
 }
