use crate::proxy::Proxy;

/// The Communicator is a basic object in ZeroC Ice. Currently
/// this is more a stub that does dummy initialization.
pub struct Communicator {}

impl Communicator {
    pub fn string_to_proxy(&self, proxy_string: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
        Proxy::new(proxy_string)
    }
}