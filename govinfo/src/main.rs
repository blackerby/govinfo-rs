use govinfo::GovInfo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = GovInfo::new(String::from("DEMO_KEY"));
    let collections = client.collections()?;

    println!("{:#?}", collections);

    Ok(())
}
