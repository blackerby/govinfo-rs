use govinfo::GovInfo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = GovInfo::new(String::from("DEMO_KEY"));
    let collections = client.collections()?;

    println!("{:#?}", collections);

    let packages = client.published_since("2024-03-13", "bills")?;

    println!("{:#?}", packages);

    let packages = client.published_between("2024-03-13", "2024-03-14", "bills")?;

    println!("{:#?}", packages);

    Ok(())
}
