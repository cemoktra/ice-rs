use std::thread;
use std::time;

use termion;
use termion::input::TermRead;

mod gen;
use crate::gen::demo::{Hello,HelloPrx};

fn menu() {
    println!("usage:");
    println!("t: send greeting");
    println!("s: shutdown server");
    println!("x: exit");
    println!("?: help");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut comm = ice_rs::communicator::initialize("config.client").await?;
    let proxy = comm.string_to_proxy("hello").await?;
    let mut hello_prx = HelloPrx::checked_cast(proxy).await?;

    menu();
    let mut stdin = termion::async_stdin().keys();

    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('x') => return Ok(()),
                termion::event::Key::Char('t') => {
                    hello_prx.say_hello(None).await?
                },
                termion::event::Key::Char('s') => {
                    hello_prx.shutdown(None).await?
                },
                termion::event::Key::Char('?') => {
                    menu()
                },
                _ => {},
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}