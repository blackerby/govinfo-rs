use govinfo::{Collections, MAX_PAGE_SIZE};
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let client = Collections::new("DEMO_KEY");
    let collections = client.all()?;

    println!("{:#?}", collections);

    let params = HashMap::from([("offsetMark", "*"), ("pageSize", MAX_PAGE_SIZE)]);
    let bills_since = client.since("bills", "2024-03-13T00:00:00Z", params);

    println!("{:#?}", bills_since);

    Ok(())
}
