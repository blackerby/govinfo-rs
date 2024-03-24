#[allow(unused_imports)]
use govinfo::{GovInfo, Packages, Params, Published, Related};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let client = GovInfo::new(None);
    // let collections = client.collections().get();
    // println!("{:#?}", collections);

    // let bills = GovInfo::new(None)
    //     .collections()
    //     .collection(String::from("bills"))
    //     .start_date(String::from("2024-03-21T22:00:00Z"))
    //     .end_date(String::from("2024-03-21T23:00:00Z"))
    //     .get();
    // println!("{:#?}", bills);
    let bills = GovInfo::new(None)
        .published()
        .collection(String::from("bills"))
        .start_date(String::from("2024-03-22"))
        .page_size(10)
        .get()?;
    println!("{:#?}", bills);
    Ok(())
}
