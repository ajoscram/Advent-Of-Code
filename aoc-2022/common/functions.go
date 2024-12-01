package common

func Max(values ...int) int {
	max := values[0]

	for _, value := range values {
		if max < value {
			max = value
		}
	}

	return max
}

func Add(values ...int) int {
	result := 0
	for _, value := range values {
		result += value
	}
	return result
}

func Keys[K comparable, V any](m map[K]V) []K {
	var keys []K

	for key := range m {
		keys = append(keys, key)
	}

	return keys
}
