func rangeBitwiseAnd(left int, right int) int {
	shift := 0
	/*
		如果二进制长度不一样 0
		如果一样，寻找最长公共前缀
	*/
	//寻找最长公共前缀
	for left < right {
		left, right = left>>1, right>>1
		shift++
	}
	return left << shift
}