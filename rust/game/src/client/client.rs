use std::{net::{TcpStream}, io::{Error, Write, Read}, str::from_utf8, };


pub struct Client{
    pub client_nr : i32,
    pub buf : [u8; 50],
    pub connection : TcpStream
}

impl Client {

    pub fn new() -> Client{
        Client {
            client_nr : 0,
            buf : [0 as u8; 50],
            connection: Client::connect().unwrap(),
        }
    }

    fn connect() -> Result<TcpStream, Error>{
        //establish connection
        match TcpStream::connect("192.168.0.35:9988") {
            Ok(stream) => {
                println!("Successfully connected to server in port 9988");
                Ok(stream)
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
                Err(e)
            }
        }
    }

    pub fn write(&mut self, message : &str) -> Result<usize, Error>{
        let msg = message.as_bytes();
        //write data on the stream, may return nr of bytes written or error
        return self.connection.write(msg);
    }


    pub fn read(&mut self) -> Result<&str, Error>{
        match self.connection.read(&mut self.buf){
            Ok(size) => {
                return Ok(from_utf8(&self.buf[0..size]).unwrap());
            }
            ,
            Err(e) => {
                println!("Failed to read from server {}", e);
                Err(e)
            }
        }
    }

    pub fn read_client_id(&mut self) -> Result<i32, Error>{
        //4 byte array buffer, to read i32
        let mut buf = [0 as u8; 4];
        match self.connection.read(&mut buf){
            Ok(_) => {
                return Ok(i32::from_be_bytes(buf));
            }
            ,
            Err(e) => {
                println!("Failed to read from server {}", e);
                Err(e)
            }
        }
    }



}