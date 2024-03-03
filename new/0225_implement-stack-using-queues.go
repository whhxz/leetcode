package main

type MyStack struct {
	data []int
}

func Constructor() MyStack {
	return MyStack{
		data: []int{},
	}
}

func (this *MyStack) Push(x int) {
	this.data = append(this.data, x)
}

func (this *MyStack) Pop() int {
	if len(this.data) == 0 {
		return 0
	}
	res := this.data[len(this.data)-1]
	this.data = this.data[:len(this.data)-1]
	return res
}

func (this *MyStack) Top() int {
	if len(this.data) == 0 {
		return 0
	}
	return this.data[len(this.data)-1]
}

func (this *MyStack) Empty() bool {
	return len(this.data) == 0
}

