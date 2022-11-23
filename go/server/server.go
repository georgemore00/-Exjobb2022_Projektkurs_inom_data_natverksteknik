// Package main so that we can execute the program
package main

import (
	"fmt"
	"net"
	"os"
	"strconv"
)

const (
	SERVER_HOST = "192.168.0.35"
	SERVER_PORT = "9988"
	SERVER_TYPE = "tcp"
)

func main() {
	clients := [2]net.Conn{}
	nrOfClients := 0
	fmt.Println("Server Running...")
	server, err := net.Listen(SERVER_TYPE, SERVER_HOST+":"+SERVER_PORT)
	if err != nil {
		fmt.Println("Error listening:", err.Error())
		os.Exit(1)
	}
	defer server.Close()

	fmt.Println("Listening on " + SERVER_HOST + ":" + SERVER_PORT)
	//wait until 2 clients are connected
	for nrOfClients != 2 {
		fmt.Println("Waiting for clients...")
		connection, err := server.Accept()
		if err != nil {
			fmt.Println("Error accepting: ", err.Error())
			os.Exit(1)
		}

		fmt.Printf("client connected from %v\n", connection.RemoteAddr().String())
		clients[nrOfClients] = connection
		nrOfClients += 1

		connection.Write([]byte(strconv.FormatInt(int64(nrOfClients), 10)))
	}

	//loop
	//send tcp packets between the 2 clients
	for {
		processClient(clients[0], clients)
		processClient(clients[1], clients)
	}
}
func processClient(connection net.Conn, clients [2]net.Conn) error {
	buffer := make([]byte, 1024)
	mLen, err := connection.Read(buffer)
	if err != nil {
		fmt.Println("Error reading:", err.Error())
	}
	fmt.Println("Received: ", string(buffer[:mLen]), " from", connection.RemoteAddr().String())

	if connection.RemoteAddr().String() == clients[0].RemoteAddr().String() {
		_, err = clients[1].Write([]byte(string(buffer[:mLen])))
	} else {
		_, err = clients[0].Write([]byte(string(buffer[:mLen])))
	}
	return err
}
