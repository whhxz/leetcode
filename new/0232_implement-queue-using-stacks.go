package main

type MyQueue struct {
	data []int
}

func Constructor() MyQueue {
	return MyQueue{
		data: []int{},
	}
}

func (this *MyQueue) Push(x int) {
	this.data = append(this.data, x)
}

func (this *MyQueue) Pop() int {
	if len(this.data) == 0 {
		return 0
	}
	res := this.data[0]
	this.data = this.data[1:]
	return res
}

func (this *MyQueue) Peek() int {
	if len(this.data) == 0 {
		return 0
	}
	return this.data[0]
}

func (this *MyQueue) Empty() bool {
	return len(this.data) == 0
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(x);
 * param_2 := obj.Pop();
 * param_3 := obj.Peek();
 * param_4 := obj.Empty();
 */

