package pong

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
)

type Rect struct {
	Width  int
	Height int
}

func NewRect(width, height int) *Rect {
	return &Rect{
		Width:  width,
		Height: height,
	}
}

func (r Rect) Draw(screen *ebiten.Image, posX, posY float32) {
	// create rect
	img := ebiten.NewImage(r.Width, r.Height)
	img.Fill(color.White)

	//set x, y position
	pOpts := &ebiten.DrawImageOptions{}
	pOpts.GeoM.Translate(float64(posX), float64(posY-float32(r.Height/2)))

	//draw
	screen.DrawImage(img, pOpts)
}
