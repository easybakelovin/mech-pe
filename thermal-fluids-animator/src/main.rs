use std::{env, fs, io, path::PathBuf};

use thermal_fluids_animator::render_lessons;

fn main() -> io::Result<()> {
    let output_dir = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("dist"));

    fs::create_dir_all(&output_dir)?;

    for (file_name, html) in render_lessons() {
        let path = output_dir.join(&file_name);
        fs::write(&path, html)?;
        println!("wrote {}", path.display());
    }

    Ok(())
}
