package main

type Card uint

type Player struct {
	initialStack uint
	stack uint
	name string
	cards []Card
}

type Table struct {
	small uint
	big uint
	dealer uint
	players []Player
	tableCards [][]Card
	deck []Card
}

type Game interface {
	addPlayer(name string)	

	doTableAction(action string)

	getEventService() <-chan Table
}
