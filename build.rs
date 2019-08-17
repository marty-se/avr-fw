use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::Write;

const CPU_FREQ: u32 = 8000000;
const USB_FREQ: u32 = CPU_FREQ;
const BOARD: &str = "CULV3"; // See LUFA/Drivers/Board/Board.h
const ARCH: &str = "AVR8";  // Other options include XMEGA and UC3. See LUFA sources.

fn make_lufa_lib(cdcglue_dir: &Path) {
    let mcu = avr_mcu::current::mcu().expect("Invalid current target");
    let mcu_name = mcu.device.name.to_lowercase();

    let output = Command::new("make")
        .arg("lib")
        .current_dir(cdcglue_dir)
        .env("MCU", mcu_name)
        .env("ARCH", ARCH)
        .env("BOARD", BOARD)
        .env("F_CPU", format!("{}", CPU_FREQ))
        .env("F_USB", format!("{}", USB_FREQ))
        .env("OPTIMIZATION", "s")
        .env("TARGET", "cdcglue")
        .env("SRC", "Descriptors.c $(LUFA_SRC_USB) $(LUFA_SRC_USBCLASS)")
        .env("LUFA_PATH", "../../lufa/LUFA/")
        .env("CC_FLAGS", "-DUSE_LUFA_CONFIG_HEADER -IConfig/")
        .output()
        .expect("Failed to execute make");

    std::io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());

    println!("cargo:rustc-link-search={}", cdcglue_dir.display());
    //println!("cargo:rustc-link-search={}", "/usr/lib/avr/lib/avr5");
    //println!("cargo:rustc-link-search={}", "/usr/lib/gcc/avr/5.4.0/avr5");
    println!("cargo:rustc-link-lib=static=cdcglue");
}

fn blacklist_types(builder: bindgen::Builder) -> bindgen::Builder {
    builder.blacklist_type("SCSI.*")
}

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cdcglue_dir = manifest_dir.join("cdcglue");

    make_lufa_lib(&cdcglue_dir);

    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("crate::rust_ctypes")
        .header("cdcglue/cdcglue.h")
        .clang_arg("-I/usr/lib/avr/include")
        .clang_arg("-I/usr/lib/gcc/avr/5.4.0/include")
        .clang_arg("-I/usr/lib/gcc/avr/5.4.0/include-fixed")
        .clang_arg("-I../lufa/")
        .clang_arg("-ffreestanding");

    let mcu = avr_mcu::current::mcu().expect("Invalid current target");

    builder = builder.clang_arg(format!("-D{}", mcu.c_preprocessor_name))
        .clang_arg(format!("-DF_CPU={}", CPU_FREQ))
        .clang_arg(format!("-DF_USB={}", USB_FREQ))
        .clang_arg("--target=avr")
        .rustfmt_bindings(true);

    builder = blacklist_types(builder);

    let bindings =
        builder
        .generate()
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_out_buf = out_path.join("bindings.rs");
    let bindings_out_path = bindings_out_buf.as_path();
    bindings
        .write_to_file(bindings_out_path)
        .expect("Couldn't write bindings!");
    
    // rustfmt doesn't work in the AVR toolchain, so let's execute it separately.
    let rustfmt_output = Command::new("rustup")
        .arg("run")
        .arg("nightly")
        .arg("rustfmt")
        .arg(bindings_out_path)
        .output()
        .expect("rustfmt failed");

    std::io::stderr().write_all(&rustfmt_output.stderr).unwrap();
    assert!(rustfmt_output.status.success());
}