use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, str::from_utf8};

fn main() {
    let mut nr_of_clients : i32= 0;
    let mut connections: Vec<TcpStream> = Vec::new();

    //init server
    let listener = TcpListener::bind("192.168.0.35:9988").unwrap();
    println!("Server listening on port 9988");

    // wait for clients to connect
    while nr_of_clients != 2 {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    
                    nr_of_clients += 1;
                    //tell the recently connected client which clientId it has
                    stream.write(&nr_of_clients.to_be_bytes()).unwrap();
                    connections.push(stream);
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
    }

    println!("both clients connected");
    
    loop {
        proccess_clients(&mut connections);
    }

    // close the socket server
    //drop(listener);
}

//Reads from both clients and forwards to the other
pub fn proccess_clients(connections: &mut Vec<TcpStream>){
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    match connections[0].read(&mut data) {
        Ok(size) => {
            // echo everything!
            println!("received: {:?} from {}", from_utf8(&data[0..size]).unwrap(), connections[0].peer_addr().unwrap());

            connections[1].write(&data[0..size]).unwrap();
        },
        Err(_) => {
            println!("An error occurred, with connection {}", connections[0].peer_addr().unwrap());
        }
    }

    //reset buffer
    data = [0 as u8; 50];

    match connections[1].read(&mut data) {
        Ok(size) => {
            // echo everything!
            println!("received: {:?} from {}", from_utf8(&data[0..size]).unwrap(), connections[1].peer_addr().unwrap());

            connections[0].write(&data[0..size]).unwrap();
        },
        Err(_) => {
            println!("An error occurred, with connection {}", connections[1].peer_addr().unwrap());
        }
    }
}
