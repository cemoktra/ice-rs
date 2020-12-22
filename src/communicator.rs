use crate::proxy::Proxy;

pub struct Communicator {

}

impl Communicator {
    pub fn string_to_proxy(&self, proxy_string: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
        Proxy::new(proxy_string)
    }
}