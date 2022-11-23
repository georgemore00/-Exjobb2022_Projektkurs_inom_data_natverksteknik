package common

import "strconv"

type Message struct {
	Data string
}

func NewMessage(x, y float32) Message {
	return Message{
		Data: strconv.FormatFloat(float64(x), 'f', 2, 32) + "," + strconv.FormatFloat(float64(y), 'f', 2, 32),
	}
}
