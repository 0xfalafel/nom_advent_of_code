use color_eyre::Result;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    let input_data = fs::read_to_string("input.txt")?;
    
    for line in input_data.lines() {
        println!("{line}");
    }

    Ok(())
}
