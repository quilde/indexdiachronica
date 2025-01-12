package main

import (
	"fmt"
	"os"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

type model struct {
	data []FamilySet
	list list.Model
}

type item struct {
	title, desc string
}

func (i item) Title() string       { return i.title }
func (i item) Description() string { return i.desc }
func (i item) FilterValue() string { return i.title }

var baseStyle = lipgloss.NewStyle().
	BorderStyle(lipgloss.NormalBorder()).
	BorderForeground(lipgloss.Color("240"))

var docStyle = lipgloss.NewStyle().Margin(2, 1).Padding(3).Border(lipgloss.InnerHalfBlockBorder())

func initialModel() model {
	data := get_data()

	var items []list.Item

	for i := range data {
		for j := range data[i].Cs {
			/*if len(data[0].Cs[lang].Ch[i].Be) > 0 && len(data[0].Cs[lang].Ch[i].Af) > 0 {
				row := table.Row{

					data[0].Cs[lang].Ch[i].Be[0],
					data[0].Cs[lang].Ch[i].Af[0],
					data[0].Cs[lang].Ch[i].En,
				}
				rows = append(rows, row)
			} */
			item := item{
				title: data[i].Cs[j].To,
				desc:  data[i].Cs[j].Fr,
			}
			items = append(items, item)

		}
	}

	m := model{list: list.New(items, list.NewDefaultDelegate(), 0, 0)}
	m.list.Title = "My Fave Things"

	return model{
		data: data,
		list: m.list,
	}
}

func (m model) Init() tea.Cmd {
	// Just return `nil`, which means "no I/O right now, please."
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		/* case "esc":
		if m.t.Focused() {
			m.t.Blur()
		} else {
			m.t.Focus()
		} */
		case "q", "ctrl+c":
			return m, tea.Quit
		case "enter":
			return m, tea.Batch(
				tea.Printf("Let's go to %s!", m.list.SelectedItem()),
			)

		}
	case tea.WindowSizeMsg:
		h, v := docStyle.GetFrameSize()
		m.list.SetSize(msg.Width-h, msg.Height-v)
	}
	m.list, cmd = m.list.Update(msg)
	return m, cmd
}

func (m model) View() string {
	// The header
	s := "What should we buy at the market?\n\n"

	s += baseStyle.Render("\n" + m.list.View() + "\n")

	// The footer
	s += "\nPress q to quit.\n"

	// Send the UI for rendering
	return s
}

func main() {
	p := tea.NewProgram(initialModel())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
