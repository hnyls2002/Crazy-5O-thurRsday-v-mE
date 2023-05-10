use std::fs::{read_dir, File};
use std::io::{Result, Write};

// generate a link_app.S file to contain all the app's elf file
static SOURCE_PATH: &str = "../user/src/";
static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";
static LINK_APP_FILE: &str = "./src/link_app.S";

fn main() {
    // src file update may be needless
    // cause we always "make build" in "user/" then update the target file
    println!("cargo:rerun-if-changed={}", SOURCE_PATH);
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    build_insert_app_asm().unwrap();
}

fn build_insert_app_asm() -> Result<()> {
    let mut f = File::create(LINK_APP_FILE).unwrap();

    // get all app's name without ext
    let mut app_list: Vec<_> = read_dir(SOURCE_PATH.to_string() + "bin/")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_ext.drain(name_ext.find('.').unwrap()..name_ext.len());
            name_ext
        })
        .collect();
    app_list.sort();

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        app_list.len()
    )?;

    for i in 0..app_list.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, app_list.len() - 1)?;

    writeln!(
        f,
        r#"
.global _app_names
_app_names:"#
    )?;
    for app in app_list.iter() {
        writeln!(f, r#"    .string "{}""#, app)?;
    }

    for (idx, app) in app_list.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
    .align 3
app_{0}_start:
    .incbin "{2}{1}"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}
