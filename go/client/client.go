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

func (c *Client) Connect(client *Client) {
	//establish connection
	connection, err := net.Dial(SERVER_TYPE, SERVER_HOST+":"+SERVER_PORT)
	if err != nil {
		panic(err)
	}
	client.Connection = connection
}

func (c *Client) Write(connection net.Conn, message string) error {
	///send some data
	_, err := connection.Write([]byte(message))
	if err != nil {
		fmt.Println("Error writing:", err.Error())
	}
	return err
}

func (c *Client) Read(connection net.Conn) string {
	buffer := make([]byte, 1024)
	mLen, err := connection.Read(buffer)
	if err != nil {
		fmt.Println("Error reading:", err.Error())
		panic(err)
	}

	//fmt.Println("Received: ", string(buffer[:mLen]))
	return string(buffer[:mLen])
}
