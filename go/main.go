package main

import (
	"labb/projektkurs/client"
	"labb/projektkurs/common"
	"labb/projektkurs/pong"
	"log"
	"strconv"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
)

const (
	windowWidth   = 800
	windowHeight  = 600
	ballStartX    = 400
	ballStartY    = 300
	player1StartX = padding
	player2StartX = windowWidth - 20
	playersStartY = windowHeight / 2
	playerSpeed   = 5.0

	//todo sätt till 10 i framtiden
	padding = 20
)

type Game struct {
	ball        *pong.Ball
	myPlayer    *pong.Player
	enemyPlayer *pong.Player
	client      *client.Client
}

// Called 60 times per second
// Game logic goes here
func (g *Game) Update() error {

	//goroutines for async sending and recieving

	//recieve
	go func() {
		recieved := g.client.Read(g.client.Connection)
		splitted := strings.Split(recieved, ",")
		enemyY, err := strconv.ParseFloat(splitted[1], 32)

		if err != nil {
			println("Corrupted data, could not parse into x y positions")
		}
		//move enemy
		g.enemyPlayer.Position.Y = float32(enemyY)
	}()

	// send
	go func() {
		message := common.NewMessage(g.myPlayer.Position.X, g.myPlayer.Position.Y)
		g.client.Write(g.client.Connection, message.Data)

	}()

	//move myplayer
	if ebiten.IsKeyPressed(ebiten.KeyS) {
		if g.myPlayer.Position.Y+float32(g.myPlayer.Rect.Height) < float32(windowHeight-padding) {
			g.myPlayer.Position.Y += playerSpeed
		}
	}
	if ebiten.IsKeyPressed(ebiten.KeyW) {
		if g.myPlayer.Position.Y+float32(g.myPlayer.Rect.Height) > float32(padding) {
			g.myPlayer.Position.Y -= playerSpeed
		}
	}

	//move ball
	if g.client.ClientNr == 1 {
		g.ball.Update(windowHeight, g.myPlayer, g.enemyPlayer)
	} else {
		g.ball.Update(windowHeight, g.enemyPlayer, g.myPlayer)
	}

	//check if anyone won round
	if g.ball.Position.X <= 0 {
		g.enemyPlayer.Score += 1
		g.ball.Reset()
		g.myPlayer.ResetPosition()
		println("enemyPlayer won round")
	}
	if g.ball.Position.X >= windowWidth {
		g.myPlayer.Score += 1
		g.ball.Reset()
		g.myPlayer.ResetPosition()
		println("myPlayer won round")
	}
	return nil
}

// Draws the game screen
func (g *Game) Draw(screen *ebiten.Image) {

	//Draw scores
	ebitenutil.DebugPrintAt(screen, "score "+strconv.FormatInt(int64(g.myPlayer.Score), 10), 200, 15)
	ebitenutil.DebugPrintAt(screen, "score "+strconv.FormatInt(int64(g.enemyPlayer.Score), 10), 600, 15)

	// Draw ball
	g.ball.Draw(screen, g.ball.Position.X, g.ball.Position.Y)

	// Draw players
	g.myPlayer.Draw(screen)
	g.enemyPlayer.Draw(screen)
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (screenWidth, screenHeight int) {
	return windowWidth, windowHeight
}

func main() {
	ebiten.SetWindowSize(windowWidth, windowHeight)
	ebiten.SetWindowTitle("Pong")

	//init client
	client := client.Client{}
	client.Connect(&client)
	clientNr, err := strconv.ParseInt(client.Read(client.Connection), 10, 32)
	if err != nil {
		log.Println(error.Error(err))
	}
	client.ClientNr = clientNr

	//will block game until server starts sending out game data
	//which will happen when both clients are connected
	//so that they start the game at the same time
	client.Read(client.Connection)
	/*if client.ClientNr == 1 {
		time.Sleep(time.Millisecond * 100)
	}*/

	//init starting positions, may vary, based on the order you connect to the server
	var myPlayer *pong.Player
	var enemyPlayer *pong.Player
	if clientNr == 1 {
		myPlayer = pong.NewPlayer(player1StartX, playersStartY)
		enemyPlayer = pong.NewPlayer(player2StartX, playersStartY)
	} else {
		myPlayer = pong.NewPlayer(player2StartX, playersStartY)
		enemyPlayer = pong.NewPlayer(player1StartX, playersStartY)
	}

	//Starts the game loop
	if err := ebiten.RunGame(&Game{
		ball:        pong.NewBall(ballStartX, ballStartY),
		myPlayer:    myPlayer,
		enemyPlayer: enemyPlayer,
		client:      &client,
	}); err != nil {
		log.Fatal(err)
	}
	defer client.Connection.Close()
}
