use std::net::UdpSocket;
use crate::UserInput;

static mut THREAD: Option<std::thread::JoinHandle<()>> = None;

pub fn start(tx: std::sync::mpsc::Sender<UserInput>) -> () {
    unsafe {
        THREAD = Some(std::thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:15666").expect("Could not bind socket");
            let mut buffer = [0u8, 255];
            loop {
                if let Ok(len) = socket.recv_from(&mut buffer) {
                    if len.0 == 2 && buffer[0] == 0x33 {
                        let res = match buffer[1] {
                            0 => tx.send(UserInput::NextScreen),
                            1 => tx.send(UserInput::PrevScreen),
                            2 => tx.send(UserInput::ScreensaverOn),
                            3 => tx.send(UserInput::ScreensaverOff),
                            4 => tx.send(UserInput::Quit),
                            _ => Ok(()),
                        };
                        if let Err(e) = res {
                            println!("Got error sending user input: {:?}", e);
                        }
                    }
                }
            }
        }));
    }
}