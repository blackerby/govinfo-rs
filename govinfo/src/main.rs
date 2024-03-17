use govinfo::{Collections, Interval};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = Collections::new("DEMO_KEY");
    let collections = client.all()?;

    println!("{:#?}", collections);

    let bills_since = client.since("bills", "2024-03-13T00:00:00Z");

    println!("{:#?}", bills_since);

    Ok(())
}
