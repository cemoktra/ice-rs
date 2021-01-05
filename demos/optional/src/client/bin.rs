use ice_rs::communicator::Communicator;

mod gen;
use crate::gen::demo::{ContactDB,ContactDBPrx,NumberType};


fn run(comm: &Communicator) -> Result<(), Box<dyn std::error::Error>> {
    let proxy = comm.property_to_proxy("ContactDB.Proxy")?;
    let mut contact_db = ContactDBPrx::checked_cast(proxy)?;

    let john_number = Some(String::from("123-456-7890"));
    contact_db.add_contact(&String::from("john"), Some(NumberType::HOME), john_number.clone(), Some(0))?;

    print!("Checking john ... ");
    let number = contact_db.query_number(&String::from("john"))?;    
    if !number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    if number != john_number {
        print!("number is incorrect ");
        return Ok(())
    }

    let mut dial_group = None;
    contact_db.query_dialgroup(&String::from("john"), &mut dial_group)?;

    if !dial_group.is_some() || dial_group.unwrap() != 0 {
        print!("dialgroup is incorrect ");
        return Ok(())
    }

    println!("ok");


    let steve_number = Some(String::from("234-567-8901"));
    contact_db.add_contact(&String::from("steve"), None, steve_number.clone(), Some(1))?;
    print!("Checking steve ... ");
    let number = contact_db.query_number(&String::from("steve"))?;    
    if !number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    if number != steve_number {
        print!("number is incorrect ");
        return Ok(())
    }
    let info = contact_db.query(&String::from("steve"))?;
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
    contact_db.add_contact(&String::from("frank"), Some(NumberType::CELL), frank_number.clone(), None)?;
    print!("Checking frank ... ");
    let number = contact_db.query_number(&String::from("frank"))?;    
    if !number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    if number != frank_number {
        print!("number is incorrect ");
        return Ok(())
    }
    let info = contact_db.query(&String::from("frank"))?;
    if info.dial_group.is_some() {
        print!("info is incorrect ");
        return Ok(())
    }
    if info.number != frank_number || info.r#type != Some(NumberType::CELL) {
        print!("info is incorrect ");
        return Ok(())
    }

    println!("ok");


    contact_db.add_contact(&String::from("anne"), Some(NumberType::OFFICE), None, Some(2))?;
    print!("Checking anne ... ");
    let number = contact_db.query_number(&String::from("anne"))?;    
    if number.is_some() {
        print!("number is incorrect ");
        return Ok(())
    }
    let info = contact_db.query(&String::from("anne"))?;
    if info.number.is_some() {
        print!("info is incorrect ");
        return Ok(())
    }
    if info.r#type != Some(NumberType::OFFICE) || info.dial_group != Some(2) {
        print!("info is incorrect ");
        return Ok(())
    }

    let anne_number = Some(String::from("456-789-0123"));
    contact_db.update_contact(&String::from("anne"), None, anne_number.clone(), None)?;
    let info = contact_db.query(&String::from("anne"))?;
    if info.number != anne_number || info.r#type != Some(NumberType::OFFICE) || info.dial_group != Some(2) {
        print!("info is incorrect ");
        return Ok(())
    }

    println!("ok");

    contact_db.shutdown()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let comm = ice_rs::communicator::initialize("config.client");
    run(&comm)
}