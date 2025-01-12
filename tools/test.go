package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/pelletier/go-toml/v2"
)

// Hello returns a greeting for the named person.
func Hello(name string) string {
	// Return a greeting that embeds the name in a message.
	message := fmt.Sprintf("Hi, %v. Welcome!", name)
	return message
}

func get_data() []FamilySet {
	fmt.Println(Hello("Index"))

	/*
		absPath, _ := filepath.Abs(`..\INDEX\CANON\afroasiatic.toml`)
		dat, fileerr := os.ReadFile(absPath)
		if fileerr != nil {
			panic(fileerr)
		}
	*/

	entries, err1 := os.ReadDir(`..\INDEX\CANON\`)
	if err1 != nil {
		fmt.Println("bad" + err1.Error())
		panic(err1)
	}

	var filenames []string
	var files [][]byte
	for _, e := range entries {
		if !e.IsDir() {
			filenames = append(filenames, e.Name())
		}
	}
	for _, n := range filenames {
		if !strings.HasSuffix(n, "_sources.toml") && !strings.HasSuffix(n, "_sources.yaml") {
			fmt.Println(n)
			absPath, _ := filepath.Abs(`..\INDEX\CANON\` + n)
			var f, fileerr = os.ReadFile(absPath)
			if fileerr != nil {
				panic(fileerr)
			}
			files = append(files, f)
		}

	}

	var cfgs []FamilySet
	for _, f := range files {

		var cfg FamilySet
		err := toml.Unmarshal(f, &cfg)
		if err != nil {
			panic(err)
		}

		//fmt.Println("Family:", cfg.Fa)

		//fmt.Println("name:", cfg.Cs[0].Fr)
		cfgs = append(cfgs, cfg)
	}
	return cfgs
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
	En string
	No []string
	Ta []string
	Ty []string
}
