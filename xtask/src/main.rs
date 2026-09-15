//xtask for your help to not go insane
use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn build() { //fuck you cargo build it please
    let st = Command::new("cargo")
        .args(["build", "-p", "Diafragma_OS", "--target", "x86_64-unknown-none"])
        .status()
        .expect("cargo commit suicide oh no :c");
    if !st.success() {
        eprintln!("build has failed you");
        std::process::exit(1);
    }
    println!("build done WOOOO :D");
}

fn iso() { //make iso work
    build(); //freash so i dont get something that is broken
    fs::create_dir_all("iso_root/boot").unwrap();
    fs::create_dir_all("iso_root/EFI/BOOT").unwrap();

    let elf = "target/x86_64-unknown-none/debug/Diafragma_OS";
    assert!(Path::new(elf).exists(), "uhm elf missing build die");
    fs::copy(elf, "iso_root/boot/kernel.elf").unwrap();
    fs::copy("limine.conf", "iso_root/boot/limine.conf").unwrap();
    fs::copy("limine.conf", "iso_root/limine.conf").unwrap(); //limine looks in both, cover your butthole
    fs::copy("/usr/share/limine/BOOTX64.EFI","iso_root/EFI/BOOT/BOOTX64.EFI").unwrap();
    fs::copy("/usr/share/limine/limine-uefi-cd.bin", "iso_root/limine-uefi-cd.bin").unwrap();

    let st = Command::new("xorriso")
        .args([
            "-as", "mkisofs",
            "--efi-boot", "limine-uefi-cd.bin",
            "-efi-boot-part", "--efi-boot-image",
            "--protective-msdos-label", 
            "iso_root", "-o", "rustos.iso",
        ])
        .status()
        .expect("xorriso suicided oh no...");
    if !st.success() {
        eprintln!("yo cuh iso no generate something die");
        std::process::exit(1);
    }
    println!("iso done slap into ventoy");
}
fn main() {
    let arg = env::args().nth(1).unwrap_or("build".into());
    match arg.as_str() {
        "build" => build(),
        "iso" => iso(),
        _ => println!("cargo run -p xtask -- build to run it"),
    }
}
