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
	let build_dir = freetype_native_dir.join("build");
	fs::create_dir(&build_dir).is_ok();
	Command::new("cmake")
		.arg("..")
		.arg(format!("-DFREETYPE_INCLUDE_DIR_freetype2={}",freetype_include))
		.arg(format!("-DFREETYPE_LIBRARY={}",freetype_lib))
		.arg(format!("-Dfreetype-gl_BUILD_DEMOS=OFF"))
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
	fs::create_dir(&build_dir).is_ok();
	Command::new("emcmake")
		.arg("cmake")
		.arg("..")
		.arg(format!("-DFREETYPE_INCLUDE_DIR_freetype2={}",freetype_include))
		.arg(format!("-DFREETYPE_LIBRARY={}",freetype_lib))
		.arg(format!("-DBUILD_DEMOS=OFF"))
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
}


fn main(){
	let target_triple = env::var("TARGET").unwrap();
	let target_os = target_triple.split("-").last().unwrap();
	if target_os == "linux" {
		build_linux()
	}else if target_os == "emscripten" {
		build_emscripten()
	}else{
		panic!("target OS {} not suported yet", target_os);
	}
}
