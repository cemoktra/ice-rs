use crate::properties::Properties;


#[derive(Clone)]
pub struct InitializationData {
    properties: Properties,
}

impl InitializationData {
    pub fn new() -> InitializationData {
        InitializationData {
            properties: Properties::new()
        }
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn properties_as_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
}
