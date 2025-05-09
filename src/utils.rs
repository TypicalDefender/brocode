use anyhow::Result;
use std::io::{self, Write};

/// Prompt the user for input with the given message
pub fn prompt(message: &str) -> Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
