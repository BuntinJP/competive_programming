while (True):
  n, m = map(int, input().split())
  if (n == 0 and m == 0):
    break
  a = [list(map(int, input().split())) for _ in range(m)]
  scores = [0] * n
  for i in range(m):
    for j in range(n):
      scores[j] += a[i][j]
  print(max(scores))
