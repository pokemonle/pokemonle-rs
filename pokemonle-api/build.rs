use vergen::{BuildBuilder, Emitter, Git2Builder, RustcBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/config");
    println!("cargo:rerun-if-changed=build.rs");

    let build = BuildBuilder::all_build()?;
    let rustc = RustcBuilder::all_rustc()?;
    let git2 = Git2Builder::all_git()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&git2)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
