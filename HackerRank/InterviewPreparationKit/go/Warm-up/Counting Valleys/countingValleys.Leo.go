/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

func countingValleys(steps int32, path string) int32 {
	var count int32 = 0
	level := 0
	for _, p := range path {
		// al momento de cruzar por 0, viniendo de un
		// valor negativo, incremento el contador
		if level == -1 && p == 'U' {
			count += 1
		}
		if p == 'U' {
			level += 1
		}
		if p == 'D' {
			level -= 1
		}
	}
	return count
}
