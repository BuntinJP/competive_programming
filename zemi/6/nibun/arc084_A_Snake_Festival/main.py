import bisect
N = int(input())

A = sorted(list(map(int, input().split())))
B = sorted(list(map(int, input().split())))
C = sorted(list(map(int, input().split())))

sum = 0
i = 0

while (i < N):
  ax = bisect.bisect_left(A, B[i])
  cx = bisect.bisect_right(C, B[i])
  sum += ax * (N - cx)
  i += 1

print(sum)
