func missingNumber(nums []int) int {
	sum := 0
	numsLen := len(nums)
	for i := 0; i < numsLen; i++ {
		sum = sum + nums[i]
	}
	return (1+len(nums))*len(nums)/2 - sum
}