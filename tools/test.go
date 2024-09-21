package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/pelletier/go-toml/v2"
)

// Hello returns a greeting for the named person.
func Hello(name string) string {
	// Return a greeting that embeds the name in a message.
	message := fmt.Sprintf("Hi, %v. Welcome!", name)
	return message
}

func main() {
	fmt.Println(Hello("Index"))

	absPath, _ := filepath.Abs(`..\INDEX\CANON\afroasiatic.toml`)
	dat, fileerr := os.ReadFile(absPath)
	if fileerr != nil {
		panic(fileerr)
	}

	//source := string(dat)

	var cfg FamilySet
	//err := toml.Unmarshal([]byte(doc), &cfg)
	err := toml.Unmarshal(dat, &cfg)
	if err != nil {
		panic(err)
	}
	fmt.Println("version:", cfg.Fa)
	fmt.Println("name:", cfg.Cs[0].Fr)

	// Output:
	// version: 2
	// name: go-toml
	// tags: [go toml]
}

type FamilySet struct {
	Fa string
	Cs []ChangeSet
}

type ChangeSet struct {
	Fr string
	To string
	Or string
	Os string
	So []string
	No []string
	Ch []Rule
}

type Rule struct {
	Be []string
	Af []string
	en string
	No []string
	Ta []string
	Ty []string
}
