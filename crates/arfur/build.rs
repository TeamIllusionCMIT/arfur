use std::{
    env,
    path::{Path, PathBuf},
};

use arfur_build::{LibraryType, Runner};

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    use LibraryType::*;

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut runner = Runner::new(
        "2022.4.1",
        "2022.4.0",
        vec![
            HAL,
            HALHeaders,
            Visa,
            Netcomm,
            Chipobject,
            Runtime,
            WPIUtil,
            WPIUtilHeaders,
            WPILibC,
            WPILibCHeaders,
            WPIMath,
            WPIMathHeaders,
            NetworkTables,
            NetworkTablesHeaders,
            CSCore,
            CSCoreHeaders,
        ],
        &Path::new(&out_dir),
    );
    runner.run().await?;

    let path = PathBuf::from(format!("{out_dir}/raw/"));
    let mut b = autocxx_build::Builder::new("src/io/gyroscopes.rs", &[&path])
        .extra_clang_args(&["-std=c++17", "-stdlib=libc++"])
        .build()?;
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-stdlib=libc++")
        .compile("arfur-gyros");

    Ok(())
}