use govinfo::GovInfo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = GovInfo::new(String::from("DEMO_KEY"), String::from("collections"));
    let collections = client.collections();
    print!("{:#?}", collections);
    Ok(())
}
