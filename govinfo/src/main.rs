use govinfo::Collections;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let collections = Collections::new("DEMO_KEY");
    let collections = collections.all()?;

    println!("{:#?}", collections);

    Ok(())
}
