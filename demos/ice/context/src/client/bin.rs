mod gen;
use crate::gen::demo::{Context, ContextPrx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let comm = ice_rs::communicator::initialize("config.client");
    let proxy = comm.property_to_proxy("Context.Proxy")?;
    let mut context_prx = ContextPrx::unchecked_cast(proxy)?;

    context_prx.call(None)?;

    let mut context = std::collections::HashMap::new();
    context.insert(String::from("type"), String::from("Explicit"));
    context_prx.call(Some(context))?;

    Ok(())
}