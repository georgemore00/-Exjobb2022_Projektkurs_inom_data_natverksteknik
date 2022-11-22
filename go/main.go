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
	windowWidth   = 600
	windowHeight  = 500
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

	// send
	message := common.NewMessage(g.myPlayer.Position.X, g.myPlayer.Position.Y)
	g.client.Write(g.client.Connection, message.Data)

	//might use goroutine here

	//recieve
	recieved := g.client.Read(g.client.Connection)
	splitted := strings.Split(recieved, ",")
	enemyY, err := strconv.ParseFloat(splitted[1], 32)

	if err != nil {
		println("Corrupted data, could not parse into positions")
	}

	g.enemyPlayer.Position.Y = float32(enemyY)

	//Kanske flytta ner dessa key-input metoder in i player classen
	//player 1 move
	if ebiten.IsKeyPressed(ebiten.KeyS) {
		if g.myPlayer.Position.Y+float32(g.myPlayer.Rect.Height) < windowHeight-padding {
			g.myPlayer.Position.Y += playerSpeed
		}
	}
	if ebiten.IsKeyPressed(ebiten.KeyW) {
		if g.myPlayer.Position.Y+float32(g.myPlayer.Rect.Height) > padding {
			g.myPlayer.Position.Y -= playerSpeed
		}
	}

	//Move ball
	g.ball.Update(windowHeight, g.myPlayer, g.enemyPlayer)

	// check if anyone won round
	//Kanske bryta ut till egen metod eller class senare
	if g.ball.Position.X <= 0 {
		g.enemyPlayer.Score += 1
		g.ball.Reset()
		g.myPlayer.ResetPosition(player1StartX, playersStartY)
		g.enemyPlayer.ResetPosition(player2StartX, playersStartY)
		println("enemyPlayer won round")
	}
	if g.ball.Position.X >= windowWidth {
		g.myPlayer.Score += 1
		g.ball.Reset()
		g.myPlayer.ResetPosition(player1StartX, playersStartY)
		g.enemyPlayer.ResetPosition(player2StartX, playersStartY)
		println("myPlayer won round")
	}

	return nil
}

// Draws the game screen
func (g *Game) Draw(screen *ebiten.Image) {

	//Draw scores
	// denna är en liten fuling, metoden ska egentligen bara användas för att "testa" att saker funkar, enligt dokummentationen
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
		log.Fatal(err)
	}

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
