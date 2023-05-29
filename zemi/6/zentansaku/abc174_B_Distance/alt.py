N, D = map(int, input().split())
points = [list(map(int, input().split())) for _ in range(N)]

count = 0
for x, y in points:
  if x*x + y*y <= D*D:
    count += 1

print(count)
