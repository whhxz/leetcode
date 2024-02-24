package main

import (
	"fmt"
	"math"
	"sort"
)

func findAllPeople(n int, meetings [][]int, firstPerson int) []int {
	datas := make(map[int][][]int)
	peoples := make([]int, n)
	appendMeeting := func(p1, p2, time int) {
		if _, ok := datas[p1]; !ok {
			datas[p1] = [][]int{}
		}
		if _, ok := datas[p1]; !ok {
			datas[p1] = [][]int{}
		}
		datas[p1] = append(datas[p1], []int{p2, time})
		if _, ok := datas[p2]; !ok {
			datas[p2] = [][]int{}
		}
		datas[p2] = append(datas[p2], []int{p1, time})

	}
	for _, meeting := range meetings {
		p1, p2, time := meeting[0], meeting[1], meeting[2]
		appendMeeting(p1, p2, time)
	}
	appendMeeting(0, firstPerson, 0)

	for _, data := range datas {
		sort.Slice(data, func(i, j int) bool {
			return data[i][1] < data[j][1]
		})
	}
	for i := range peoples {
		peoples[i] = math.MaxInt
	}
	//print(datas)
	var find func(int, int)
	find = func(p, t int) {
		data, ok := datas[p]
		//not meeting
		if !ok {
			return
		}
		//exist
		if t >= peoples[p] {
			return
		}
		peoples[p] = t
		for _, d := range data {
			p2, time := d[0], d[1]
			//time end
			if t > time {
				continue
			}
			find(p2, time)
		}
	}
	find(0, 0)

	var res []int
	for i, p := range peoples {
		if p != math.MaxInt {
			res = append(res, i)
			//fmt.Printf("%d\t", i)
		}
	}
	return res
}
func print(datas map[int][][]int) {
	for p1, data := range datas {
		for _, d := range data {
			fmt.Printf("%d\t%d\t%d\n", p1, d[0], d[1])
		}
	}
}
func main() {
	meetings := [][]int{{3, 1, 3}, {1, 2, 2}, {0, 3, 3}}
	findAllPeople(4, meetings, 3)
}
