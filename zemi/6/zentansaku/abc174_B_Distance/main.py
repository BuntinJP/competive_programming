import math
n, d = list(map(int, input().split()))
l_x = []
l_y = []
c = 0
for _ in range(n):
  x, y = list(map(int, input().split()))
  l_x.append(x)
  l_y.append(y)
for i in range(n):
  now_d = math.sqrt(l_x[i] ** 2 + l_y[i] ** 2)
  if now_d <= d:
    c += 1
print(c)
