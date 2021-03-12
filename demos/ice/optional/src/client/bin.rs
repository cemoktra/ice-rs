#[macro_use]
extern crate ice_derive;

use ice_rs::communicator::Communicator;

mod gen;
use crate::gen::demo::{ContactDB,ContactDBPrx,NumberType};


async fn run(comm: &mut Communicator) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let proxy = comm.property_to_proxy("ContactDB.Proxy").await?;
    let mut contact_db = ContactDBPrx::checked_cast(proxy).await?;

    let john_number = Some(String::from("123-456-7890"));
    contact_db.add_contact(&String::from("john"), Some(NumberType::HOME), john_number.clone(), Some(0), None).await?;

    print!("Checking john ... ");
    let number = contact_db.query_number(&String::from("john"), None).await?;
    if !number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    if number != john_number {
        print!("number is incorrect ");
        return Ok(())
    }

    let mut dial_group = None;
    contact_db.query_dialgroup(&String::from("john"), &mut dial_group, None).await?;

    if !dial_group.is_some() || dial_group.unwrap() != 0 {
        print!("dialgroup is incorrect ");
        return Ok(())
    }

    println!("ok");


    let steve_number = Some(String::from("234-567-8901"));
    contact_db.add_contact(&String::from("steve"), None, steve_number.clone(), Some(1), None).await?;
    print!("Checking steve ... ");
    let number = contact_db.query_number(&String::from("steve"), None).await?;
    if !number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    if number != steve_number {
        print!("number is incorrect ");
        return Ok(())
    }
    let info = contact_db.query(&String::from("steve"), None).await?;
    if info.r#type != Some(NumberType::HOME) {
        print!("info is incorrect ");
        return Ok(())
    }
    if info.number != steve_number || info.dial_group != Some(1) {
        print!("info is incorrect ");
        return Ok(())
    }

    println!("ok");


    let frank_number = Some(String::from("345-678-9012"));
    contact_db.add_contact(&String::from("frank"), Some(NumberType::CELL), frank_number.clone(), None, None).await?;
    print!("Checking frank ... ");
    let number = contact_db.query_number(&String::from("frank"), None).await?;
    if !number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    if number != frank_number {
        print!("number is incorrect ");
        return Ok(())
    }
    let info = contact_db.query(&String::from("frank"), None).await?;
    if info.dial_group.is_some() {
        print!("info is incorrect ");
        return Ok(())
    }
    if info.number != frank_number || info.r#type != Some(NumberType::CELL) {
        print!("info is incorrect ");
        return Ok(())
    }

    println!("ok");


    contact_db.add_contact(&String::from("anne"), Some(NumberType::OFFICE), None, Some(2), None).await?;
    print!("Checking anne ... ");
    let number = contact_db.query_number(&String::from("anne"), None).await?;
    if number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    let info = contact_db.query(&String::from("anne"), None).await?;
    if info.number.is_some() {
        print!("info is incorrect ");
        return Ok(())
    }
    if info.r#type != Some(NumberType::OFFICE) || info.dial_group != Some(2) {
        print!("info is incorrect ");
        return Ok(())
    }

    let anne_number = Some(String::from("456-789-0123"));
    contact_db.update_contact(&String::from("anne"), None, anne_number.clone(), None, None).await?;
    let info = contact_db.query(&String::from("anne"), None).await?;
    if info.number != anne_number || info.r#type != Some(NumberType::OFFICE) || info.dial_group != Some(2) {
        print!("info is incorrect ");
        return Ok(())
    }

    println!("ok");

    contact_db.shutdown(None).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut comm = ice_rs::communicator::initialize("config.client").await?;
    run(&mut comm).await
}