use ice_rs::communicator::Communicator;
use std::collections::HashMap;
use async_trait::async_trait;

mod gen;
use crate::gen::demo::{Contact, ContactDBServer, ContactDBI, NumberType};

struct ContactDBImpl {
    data: HashMap<String, Contact>
}

#[async_trait]
impl ContactDBI for ContactDBImpl {
    async fn add_contact(&mut self, name: &String, r#type: Option<NumberType>, number: Option<String>, dial_group: Option<i32>, _context: Option<HashMap<String, String>>) {        
        let contact = Contact {
            name: name.clone(),
            r#type: if let Some(_value) = r#type { r#type } else { Some(NumberType::HOME) },
            number,
            dial_group
        };
        self.data.insert(name.clone(), contact);
    }

    async fn update_contact(&mut self, name: &String, r#type: Option<NumberType>, number: Option<String>, dial_group: Option<i32>, _context: Option<HashMap<String, String>>) {
        match self.data.get_mut(name) {
            Some(contact) => {
                contact.r#type = if let Some(_value) = r#type { r#type } else { contact.r#type };
                contact.number = if let Some(_value) = number.clone() { number } else { contact.number.clone() };
                contact.dial_group = if let Some(_value) = dial_group { dial_group } else { contact.dial_group };
            }
            _ => {}
        }
    }

    async fn query(&mut self, name: &String, _context: Option<HashMap<String, String>>) -> Contact {
        self.data.get(name).unwrap().clone()
    }

    async fn query_number(&mut self, name: &String, _context: Option<HashMap<String, String>>) -> Option<String> {
        self.data.get(name).unwrap().number.clone()
    }

    async fn query_dialgroup(&mut self, name: &String, dial_group: &mut Option<i32>, _context: Option<HashMap<String, String>>) {
        *dial_group = self.data.get(name).unwrap().dial_group.clone();
    }

    async fn shutdown(&mut self, _context: Option<HashMap<String, String>>) {
        // todo!("impl shutdown")
        // current.adapter.getCommunicator().shutdown()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let comm = Communicator::new().await?;
    let mut adapter = comm.create_object_adapter_with_endpoint("contactdb", "tcp -h localhost -p 10000").await?;

    let server = ContactDBServer::new(Box::new(ContactDBImpl{data: HashMap::new()}));

    adapter.add("contactdb", Box::new(server));
    adapter.activate().await?;

    // comm.wait_for_shutdown().await?;

    Ok(())
}