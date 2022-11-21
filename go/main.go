package main

import (
	"labb/projektkurs/pong"
	"log"
	"strconv"

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
	ball    *pong.Ball
	player1 *pong.Player
	player2 *pong.Player
}

// Called 60 times per second
// Game logic goes here
func (g *Game) Update() error {

	//Kanske flytta ner dessa key-input metoder in i player classen

	//player 1 move
	if ebiten.IsKeyPressed(ebiten.KeyS) {
		if g.player1.Position.Y+float32(g.player1.Rect.Height) < windowHeight-padding {
			g.player1.Position.Y += playerSpeed
		}
	}
	if ebiten.IsKeyPressed(ebiten.KeyW) {
		if g.player1.Position.Y+float32(g.player1.Rect.Height) > padding {
			g.player1.Position.Y -= playerSpeed
		}
	}

	//player 2 move
	if ebiten.IsKeyPressed(ebiten.KeyArrowDown) {
		if g.player2.Position.Y+float32(g.player2.Rect.Height) < windowHeight-padding {
			g.player2.Position.Y += playerSpeed
		}
	}
	if ebiten.IsKeyPressed(ebiten.KeyArrowUp) {
		if g.player2.Position.Y+float32(g.player2.Rect.Height) > padding {
			g.player2.Position.Y -= playerSpeed
		}
	}

	//Move ball
	g.ball.Update(windowHeight, g.player1, g.player2)

	// check if anyone won round
	//Kanske bryta ut till egen metod eller class senare
	if g.ball.Position.X <= 0 {
		g.player2.Score += 1
		g.ball.Reset()
		g.player1.ResetPosition(player1StartX, playersStartY)
		g.player2.ResetPosition(player2StartX, playersStartY)
		println("player2 won round")
	}
	if g.ball.Position.X >= windowWidth {
		g.player1.Score += 1
		g.ball.Reset()
		g.player1.ResetPosition(player1StartX, playersStartY)
		g.player2.ResetPosition(player2StartX, playersStartY)
		println("player1 won round")
	}

	return nil
}

// Draws the game screen
func (g *Game) Draw(screen *ebiten.Image) {

	//Draw scores
	// denna är en liten fuling, metoden ska egentligen bara användas för att "testa" att saker funkar, enligt dokummentationen
	ebitenutil.DebugPrintAt(screen, "score "+strconv.FormatInt(int64(g.player1.Score), 10), 200, 15)
	ebitenutil.DebugPrintAt(screen, "score "+strconv.FormatInt(int64(g.player2.Score), 10), 600, 15)

	// Draw ball
	g.ball.Draw(screen, g.ball.Position.X, g.ball.Position.Y)

	// Draw players
	g.player1.Draw(screen)
	g.player2.Draw(screen)
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (screenWidth, screenHeight int) {
	return windowWidth, windowHeight
}

func main() {
	ebiten.SetWindowSize(windowWidth, windowHeight)
	ebiten.SetWindowTitle("Pong")

	//Starts the game loop
	if err := ebiten.RunGame(&Game{
		ball:    pong.NewBall(ballStartX, ballStartY),
		player1: pong.NewPlayer(player1StartX, playersStartY),
		player2: pong.NewPlayer(player2StartX, playersStartY),
	}); err != nil {
		log.Fatal(err)
	}
}
