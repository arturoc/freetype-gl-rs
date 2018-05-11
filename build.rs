use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn build_linux() {
	let freetype_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freetype_include = env::var("DEP_FREETYPE2_INCLUDE_SEARCH").unwrap_or("/usr/include".to_string());
	let freetype_link = env::var("DEP_FREETYPE2_LINK_SEARCH").unwrap_or("/usr/lib/".to_string());
	let freetype_lib = format!("{}/{}",freetype_link,"libfreetype.a");
	let prev_cflags = env::var("CFLAGS").unwrap_or("".to_string());
	let cflags = format!("{} {} -I{}",prev_cflags,"-fPIC",freetype_include);
	let freetype_native_dir = Path::new(&freetype_dir).join("freetype-gl");
	let build_dir = freetype_native_dir.join("build_linux");
	fs::remove_dir_all(&build_dir).is_ok();
	fs::create_dir(&build_dir).is_ok();
	Command::new("cmake")
		.arg("..")
		.arg(format!("-DFREETYPE_INCLUDE_DIRS={}",freetype_include))
		.arg(format!("-DFREETYPE_LIBRARY={}",freetype_lib))
		.arg(format!("-Dfreetype-gl_BUILD_DEMOS=OFF"))
		.arg(format!("-Dfreetype-gl_BUILD_TESTS=OFF"))
		.arg(format!("-Dfreetype-gl_WITH_GLEW=OFF"))
		.arg(format!("-Dfreetype-gl_BUILD_APIDOC=OFF"))
		.env("CFLAGS",&cflags)
		.current_dir(&build_dir)
		.status().unwrap();
	Command::new("make")
		.current_dir(&build_dir)
		.status().unwrap();
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("libfreetype-gl.a");
	fs::copy(build_dir.join("libfreetype-gl.a"),dest_path).unwrap();
	println!("cargo:rustc-flags= -L native={}",out_dir);
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=src/lib.rs");
	println!("cargo:rerun-if-changed=src");
	println!("cargo:rerun-if-changed=freetype-gl/texture-font.c");
	println!("cargo:rerun-if-changed=freetype-gl/texture-font.h");
}


fn build_emscripten() {
	let freetype_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freetype_include = env::var("DEP_FREETYPE2_INCLUDE_SEARCH").unwrap_or("/usr/include".to_string());
	let freetype_link = env::var("DEP_FREETYPE2_LINK_SEARCH").unwrap_or("/usr/lib/".to_string());
	let freetype_lib = format!("{}/{}",freetype_link,"libfreetype.a");
	let prev_cflags = env::var("CFLAGS").unwrap_or("".to_string());
	let cflags = format!("{} {} -I{}",prev_cflags,"-fPIC",freetype_include);
	let freetype_native_dir = Path::new(&freetype_dir).join("freetype-gl");
	let build_dir = freetype_native_dir.join("build_emscripten");
	// fs::remove_dir_all(&build_dir).is_ok();
	fs::create_dir(&build_dir).is_ok();
	Command::new("emcmake")
		.arg("cmake")
		.arg("..")
		.arg(format!("-DFREETYPE_INCLUDE_DIRS={}",freetype_include))
		.arg(format!("-DFREETYPE_LIBRARY={}",freetype_lib))
		.arg(format!("-Dfreetype-gl_BUILD_DEMOS=OFF"))
		.arg(format!("-Dfreetype-gl_BUILD_TESTS=OFF"))
		.arg(format!("-Dfreetype-gl_BUILD_APIDOC=OFF"))
		.arg(format!("-Dfreetype-gl_WITH_GLEW=OFF"))
		.env("CFLAGS",&cflags)
		.current_dir(&build_dir)
		.status().unwrap();
	Command::new("emmake")
		.arg("make")
		.current_dir(&build_dir)
		.status().unwrap();
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("libfreetype-gl.a");
	fs::copy(build_dir.join("libfreetype-gl.a"),dest_path).unwrap();
	println!("cargo:rustc-flags= -L native={}",out_dir);
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=src/lib.rs");
	println!("cargo:rerun-if-changed=src");
	println!("cargo:rerun-if-changed=freetype-gl/texture-font.c");
	println!("cargo:rerun-if-changed=freetype-gl/texture-font.h");
}

fn main(){
	let target_triple = env::var("TARGET").unwrap();
	if target_triple.contains("linux") {
		build_linux()
	}else if target_triple.contains("emscripten") {
		build_emscripten()
	}else{
		panic!("target OS {} not suported yet", target_triple);
	}
}
