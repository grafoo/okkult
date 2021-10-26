package main

import (
	"context"
	"fmt"

	"github.com/anaseto/gruid"
	gruidDriver "github.com/anaseto/gruid-tcell"
	"github.com/gdamore/tcell/v2"
)

type model struct{}

func (m *model) Update(msg gruid.Msg) gruid.Effect {
	switch msg := msg.(type) {
	case gruid.MsgKeyDown:
		if msg.Key == "q" {
			return gruid.End()
		}
	}
	return nil
}

func (m *model) Draw() gruid.Grid {
	grid := gruid.NewGrid(80, 50)
	grid.Fill(gruid.Cell{Rune: '.'})
	return grid
}

type styleManager struct{}

func (s *styleManager) GetStyle(style gruid.Style) tcell.Style {
	return tcell.StyleDefault.Background(tcell.ColorBlack).Foreground(tcell.ColorWhite)
}

func main() {
	model := &model{}
	config := gruidDriver.Config{StyleManager: &styleManager{}}
	driver := gruidDriver.NewDriver(config)
	app := gruid.NewApp(gruid.AppConfig{Model: model, Driver: driver})
	if err := app.Start(context.Background()); err != nil {
		fmt.Println(err)
	}
}
