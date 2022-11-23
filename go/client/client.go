package client

import (
	"fmt"
	"net"
)

const (
	SERVER_HOST = "192.168.0.35"
	SERVER_PORT = "9988"
	SERVER_TYPE = "tcp"
)

type Client struct {
	Connection net.Conn
	ClientNr   int64
}

func (c *Client) Connect() {
	//establish connection
	connection, err := net.Dial(SERVER_TYPE, SERVER_HOST+":"+SERVER_PORT)
	if err != nil {
		panic(err)
	}
	c.Connection = connection
}

func (c *Client) Write(message string) error {
	///send some data
	_, err := c.Connection.Write([]byte(message))
	if err != nil {
		fmt.Println("Error writing:", err.Error())
	}
	return err
}

func (c *Client) Read() string {
	buffer := make([]byte, 1024)
	//read from buffer
	mLen, err := c.Connection.Read(buffer)
	if err != nil {
		fmt.Println("Error reading:", err.Error())
		panic(err)
	}
	return string(buffer[:mLen])
}
