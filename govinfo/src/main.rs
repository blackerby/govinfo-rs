use govinfo::{Collections, Published, MAX_PAGE_SIZE};
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let client = Collections::new("DEMO_KEY");
    let collections = client.all()?;

    println!("{:#?}", collections);

    let params = HashMap::from([
        ("offsetMark".to_string(), "*".to_string()),
        ("pageSize".to_string(), MAX_PAGE_SIZE.to_string()),
    ]);
    let bills_since = client.since("bills", "2024-03-13T00:00:00Z", params);

    println!("{:#?}", bills_since);

    let params = HashMap::from([
        ("offsetMark".to_string(), "*".to_string()),
        ("pageSize".to_string(), MAX_PAGE_SIZE.to_string()),
    ]);
    let client = Published::new("DEMO_KEY");
    let published_since = client.since(vec!["bills"], "2024-03-13", params);

    println!("{:#?}", published_since);

    Ok(())
}
