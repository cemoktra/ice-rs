use crate::properties::Properties;


pub struct InitializationData {
    pub properties: Properties,
}

impl InitializationData {
    pub fn new() -> InitializationData {
        InitializationData {
            properties: Properties::new()
        }
    }
}
