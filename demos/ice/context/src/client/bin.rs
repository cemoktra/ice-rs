use termion;
use termion::input::TermRead;

mod gen;
use crate::gen::demo::{Context, ContextPrx};

fn menu() {
    println!("usage:");
    println!("1: call with no request context");
    println!("2: call with explicit request context");
    println!("3: call with per proxy request context");
    println!("4: call with implicit request context");
    println!("s: shutdown server");
    println!("x: exit");
    println!("?: help");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut comm = ice_rs::communicator::initialize("config.client").await?;
    let proxy = comm.property_to_proxy("Context.Proxy").await?;
    let mut context_prx = ContextPrx::unchecked_cast(proxy).await?;

    menu();
    let mut stdin = termion::async_stdin().keys();

    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('1') => {
                    context_prx.call(None).await?;
                },
                termion::event::Key::Char('2') => {
                    let mut context = std::collections::HashMap::new();
                    context.insert(String::from("type"), String::from("Explicit"));
                    context_prx.call(Some(context)).await?;
                },
                termion::event::Key::Char('3') => {
                    println!("Deactivated");
                    // let mut context = std::collections::HashMap::new();
                    // context.insert(String::from("type"), String::from("Per-Proxy"));
                    // let proxy2 = context_prx.proxy.ice_context(context);
                    // let mut context_prx2 = ContextPrx::unchecked_cast(proxy2)?;
                    // context_prx2.call(None).await?;
                },
                termion::event::Key::Char('4') => {
                    println!("No supported yet");
                },
                termion::event::Key::Char('s') => {
                    context_prx.shutdown(None).await?
                },
                termion::event::Key::Char('x') => return Ok(()),
                termion::event::Key::Char('?') => {
                    menu()
                },
                _ => {},
            }
        }
    }
}