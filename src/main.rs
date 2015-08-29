/*
    Copyright © 2015 Zetok Zalbavar <zetok@openmailbox.org>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/



/*
    Binding to toxcore
*/
extern crate rstox;
use rstox::core::*;


/*
    For loading and writing Tox data
*/
extern crate chrono;
use chrono::UTC;

/*
    For working with text files and writing to stdout
*/
//use std::io::{Read,Stdout,Write};
use std::io::{Read,Write};
use std::fs::File;



#[derive(Debug)]
struct Bootstrap {
    ip: String,
    port: u16,
    pubkey: PublicKey,
}

/*
    Function to read file and return vector of strings, each of them
    corresponding to a line from a file.

    In a case where there is no file, return early.
*/
fn vec_strings(file: &str) -> Result<Vec<String>, ()> {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening {}: {}", file, e);
            return Err(())
        },
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Ok(content.lines().map(|l| l.to_string()).collect())
}


fn parse_to_bootstrap(string: &str) -> Option<Bootstrap> {
    let input: Vec<&str> = string.split_whitespace().collect();
    if input.len() >= 3 {
        let bootstrap = Bootstrap {
            ip: input[0].to_string(),
            port: match input[1].parse::<u16>() {
                Ok(p) => p,
                Err(_) => return None,
            },
            pubkey: match input[2].parse::<PublicKey>() {
                Ok(pk) => pk,
                Err(_) => return None,
            },
        };
        Some(bootstrap)
    } else { None }
}



fn main() {

    let nodes = match vec_strings("nodes_list") {
        Ok(nds) => nds,
        Err(_) => panic!("No \"nodes_list\" file!"),
    };

    let mut to_return: Vec<String> = vec![];

    for n in nodes {
        //let current time
        if let Some(node) = parse_to_bootstrap(&n) {

            // for breaking out of loop after timeout
            let current_time = UTC::now().timestamp();

            // new Tox instance for bootstrapping
            let mut tox = Tox::new(ToxOptions::new(), None).unwrap();
            print!("Boostrapping from {} ...  ", &n);
            // workaround, since printing line without \n often doesn't want
            // to work correctly
            drop(std::io::stdout().flush());

            if let Err(e) = tox.bootstrap(&node.ip, node.port, node.pubkey) {
                // break if something supplied won't work
                println!("BAD BOOTSTRAP - FAILED ✗");
                println!("ERROR: {:?}", e);
                continue;
            };

            let mut pushed = false;
            while !pushed {
                for ev in tox.iter() {
                    match ev {
                        ConnectionStatus(c) => {
                            // check whether connection was established,
                            // and if that's the case, push it to_return
                            match c {
                                Connection::Tcp => {
                                    to_return.push((&*n).to_string());
                                    println!("SUCCESSFUL ✔");
                                    // break out of the loop
                                    pushed = true;
                                    break;
                                },
                                Connection::Udp => {
                                    to_return.push((&*n).to_string());
                                    println!("SUCCESSFUL ✔");
                                    // break out of the loop
                                    pushed = true;
                                    break;
                                },
                                _ => {},
                            }
                        },
                        _ => {},
                    }
                }

                if current_time + 20 < UTC::now().timestamp() {
                    // if unable to bootstrap for 30s, break loop
                    println!("FAILED ✗");
                    break;
                }
                tox.wait();
            }
        } else {
            // break if there was no node
            break;
        }
    }

    println!("\n\n\n");

    // print whichever nodes worked
    for n in to_return {
        println!("{}", &n);
    }
}
