use std::env;

fn main() {
    println!("{: <30}: {}", "CARGO"                  , env!("CARGO"));
    println!("{: <30}: {}", "CARGO_HOME"             , env!("CARGO_HOME"));
    println!("{: <30}: {}", "CARGO_MANIFEST_DIR"     , env!("CARGO_MANIFEST_DIR"));
    println!("{: <30}: {}", "CARGO_PKG_AUTHORS"      , env!("CARGO_PKG_AUTHORS"));
    println!("{: <30}: {}", "CARGO_PKG_DESCRIPTION"  , env!("CARGO_PKG_DESCRIPTION"));
    println!("{: <30}: {}", "CARGO_PKG_HOMEPAGE"     , env!("CARGO_PKG_HOMEPAGE"));
    println!("{: <30}: {}", "CARGO_PKG_LICENSE"      , env!("CARGO_PKG_LICENSE"));
    println!("{: <30}: {}", "CARGO_PKG_LICENSE_FILE" , env!("CARGO_PKG_LICENSE_FILE"));
    println!("{: <30}: {}", "CARGO_PKG_NAME"         , env!("CARGO_PKG_NAME"));
    println!("{: <30}: {}", "CARGO_PKG_REPOSITORY"   , env!("CARGO_PKG_REPOSITORY"));
    println!("{: <30}: {}", "CARGO_PKG_VERSION"      , env!("CARGO_PKG_VERSION"));
    println!("{: <30}: {}", "CARGO_PKG_VERSION_MAJOR", env!("CARGO_PKG_VERSION_MAJOR"));
    println!("{: <30}: {}", "CARGO_PKG_VERSION_MINOR", env!("CARGO_PKG_VERSION_MINOR"));
    println!("{: <30}: {}", "CARGO_PKG_VERSION_PATCH", env!("CARGO_PKG_VERSION_PATCH"));
    println!("{: <30}: {}", "CARGO_PKG_VERSION_PRE"  , env!("CARGO_PKG_VERSION_PRE"));

    return;
}
