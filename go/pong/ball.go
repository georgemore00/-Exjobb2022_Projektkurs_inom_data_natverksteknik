package pong

import (
	"github.com/hajimehoshi/ebiten/v2"
)

type Ball struct {
	Position  Position
	XVelocity float32
	YVelocity float32
	Rect      Rect
}

func NewBall(x, y float32) *Ball {
	return &Ball{
		Position: Position{
			X: x,
			Y: y,
		},
		XVelocity: 3.0,
		YVelocity: -5.0,
		Rect:      *NewRect(10, 10),
	}
}

func (b *Ball) Update(windowHeight int, player1 *Player, player2 *Player) {

	//make ball move
	b.Position.X += b.XVelocity
	b.Position.Y += b.YVelocity

	/*bounce off bottom and top edge
	Uses the rect height for some padding,
	otherwise some of the ball will go into the wall before bouncing*/
	if b.Position.Y > float32(windowHeight) {
		b.YVelocity = -b.YVelocity
		//padding
		b.Position.Y = float32(windowHeight) - float32(b.Rect.Height)
	} else if b.Position.Y < 0 {
		b.YVelocity = -b.YVelocity
		//padding
		b.Position.Y = float32(b.Rect.Height)
	}

	// bounce off player rects
	//något weird med denna, man måste sikta lite i mitten av väggarna
	if b.Position.X-10 < player1.Position.X+float32(player1.Rect.Width/2) &&
		b.Position.Y > player1.Position.Y-float32(player1.Rect.Height/2) {
		b.XVelocity = -b.XVelocity
		b.Position.X = player1.Position.X + float32(player1.Rect.Width/2) + 10

	} else if b.Position.X+10 > player2.Position.X+float32(player2.Rect.Width/2) &&
		b.Position.Y > player2.Position.Y-float32(player2.Rect.Height/2) {
		b.XVelocity = -b.XVelocity
		b.Position.X = player2.Position.X + float32(player2.Rect.Width/2) - 10
	}

}

func (b *Ball) Reset() {
	b.XVelocity = 3.0
	b.YVelocity = -5.0

	b.Position.X = 0
	b.Position.Y = 0
}

func (b Ball) Draw(screen *ebiten.Image, PositionX, PositionY float32) {
	b.Rect.Draw(screen, PositionX, PositionY)
}
