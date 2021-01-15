use ice_rs::{communicator::Communicator, locator::Locator, protocol::EndPointType};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let comm = Communicator::new();

    let mut loc = Locator::new("DemoIceGrid/Locator:default -h localhost -p 4061", &comm.init_data.properties)?;
    let mut result = loc.find_object_by_id("hello")?;
    loop {
        match result.endpoint {
            EndPointType::WellKnownObject(object) => {
                result = loc.find_adapter_by_id(&object)?;
            }
            EndPointType::TCP(tcp) => {
                let proxy_string = format!("{}:tcp -h {} -p {}", "hello", tcp.host, tcp.port);
                let proxy = comm.string_to_proxy(&proxy_string)?;
                let mut hello_prx = HelloPrx::checked_cast(proxy)?;

                menu();
                let mut stdin = termion::async_stdin().keys();

                loop {
                    let input = stdin.next();
                    if let Some(Ok(key)) = input {
                        match key {
                            termion::event::Key::Char('x') => return Ok(()),
                            termion::event::Key::Char('t') => {
                                hello_prx.say_hello()?
                            },
                            termion::event::Key::Char('s') => {
                                hello_prx.shutdown()?
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
            _ => panic!("Not supported in simple demo")
        }
    }
}