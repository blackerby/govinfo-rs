use govinfo::GovInfo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = GovInfo::new(None);
    let collections = client.collections().all();
    print!("{:#?}", collections);
    let bills = client.collection(String::from("BILLS"));
    println!("{:#?}", bills);
    Ok(())
}
