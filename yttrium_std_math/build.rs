fn main() {
	println!("cargo:rustc-link-lib=qalculate");
	cxx_build::bridge("src/lib.rs").file("cpp/qalc.cpp")
		.flag_if_supported("-std=c++14").compile("libstd_math.so");
	println!("cargo:rerun-if-changed=src/lib.rs");
	println!("cargo:rerun-if-changed=cpp/qalc.cpp");
	println!("cargo:rerun-if-changed=cpp/qalc.hpp");
}