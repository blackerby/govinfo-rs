use govinfo::GovInfo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = GovInfo::new(None);
    let collections = client.collections();
    print!("{:#?}", collections);
    Ok(())
}
