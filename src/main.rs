mod labyrinth;
use std::io;

fn main() -> io::Result<()>{
    labyrinth::main()?;
    Ok(())
}
