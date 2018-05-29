extern crate cmake;

use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn build_unix() {
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

fn build_windows(){
	// panic!("{:#?}", env::vars().collect::<Vec<_>>());
	let freetype_root = env::var("DEP_FREETYPE2_ROOT").unwrap();
	let freetype_root = Path::new(&freetype_root);
	let freetype_include = freetype_root.join("include");
	let freetype_link = freetype_root.join("lib");
	let freetype_lib = freetype_link.join("freetype.lib");
	let dst = cmake::Config::new("freetype-gl")
				.define("FREETYPE_INCLUDE_DIRS", freetype_include)
				.define("FREETYPE_LIBRARY", freetype_lib)
                .define("freetype-gl_BUILD_DEMOS", "OFF")
				.define("freetype-gl_BUILD_TESTS", "OFF")
				.define("freetype-gl_WITH_GLEW", "OFF")
				.define("freetype-gl_BUILD_APIDOC", "OFF")
				.build_target("freetype-gl")
                .build();
	
	#[cfg(debug_assertions)]
	{
		let lib_dir = dst.join("build").join("Debug");
		println!("cargo:rustc-link-search=native={}", lib_dir.display());
	}

	#[cfg(not(debug_assertions))]
	{
		let lib_dir = dst.join("build").join("Release");
		println!("cargo:rustc-link-search=native={}", lib_dir.display());
	}

	println!("cargo:rustc-link-lib=static=freetype-gl");
}

fn main(){
	let target_triple = env::var("TARGET").unwrap();
	if target_triple.contains("linux") {
		build_unix()
	}else if target_triple.contains("darwin") {
		build_unix()
	}else if target_triple.contains("windows") {
		build_windows()
	}else if target_triple.contains("emscripten") {
		build_emscripten()
	}else{
		panic!("target OS {} not suported yet", target_triple);
	}
}
