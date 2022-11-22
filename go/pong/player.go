package pong

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
)

type Player struct {
	Position Position
	Score    int
	Rect     Rect
}

func NewPlayer(x, y float32) *Player {
	return &Player{
		Position: Position{
			X: x,
			Y: y,
		},
		Score: 0,
		Rect:  *NewRect(10, 30),
	}
}

func (p *Player) ResetPosition(x, y float32) {
	p.Position.X = x
	p.Position.Y = y
}

func (p *Player) Draw(screen *ebiten.Image) {
	// create rect
	img := ebiten.NewImage(p.Rect.Width, p.Rect.Height)
	img.Fill(color.White)

	//set x, y position
	pOpts := &ebiten.DrawImageOptions{}
	pOpts.GeoM.Translate(float64(p.Position.X), float64(p.Position.Y-float32(p.Rect.Height/2)))

	//draw
	screen.DrawImage(img, pOpts)
}
