#[allow(unused_imports)]
use govinfo::{Element, Error, GovInfo, Packages, Params, Published, Related};

fn main() -> Result<(), Error> {
    // let relationships: Result<Vec<Element>, Error> = GovInfo::new(None)
    //     .related()
    //     .access_id(String::from("BILLS-116hr748enr"))
    //     .get()?
    //     .into_iter()
    //     .collect();
    //
    // println!("{:#?}", relationships);
    let results: Result<Vec<Element>, Error> = GovInfo::new(None)
        .related()
        .access_id(String::from("BILLS-116hr748enr"))
        .collection(String::from("bills"))
        .get()?
        .into_iter()
        .collect();

    println!("{:#?}", results?);

    // let client = GovInfo::new(None);
    // let collections: Result<Vec<Element>, Error> =
    //     client.collections().get()?.into_iter().collect();
    // println!("{:#?}", collections);

    // let bills: Result<Vec<Element>, Error> = GovInfo::new(None)
    //     .collections()
    //     .collection(String::from("bills"))
    //     .start_date(String::from("2024-03-21T22:00:00Z"))
    //     .end_date(String::from("2024-03-21T23:00:00Z"))
    //     .get()?
    //     .into_iter()
    //     .collect();
    //
    // println!("{:#?}", bills);
    //
    // let bills: Result<Vec<Element>, Error> = GovInfo::new(None)
    //     .published()
    //     .collection(String::from("bills"))
    //     .start_date(String::from("2024-03-21"))
    //     .get()?
    //     .into_iter()
    //     .collect();
    //
    // println!("{:#?}", bills);

    Ok(())
}
