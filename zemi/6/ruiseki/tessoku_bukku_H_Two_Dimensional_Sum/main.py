H, W = map(int, input().split())
X = [list(map(int, input().split())) for _ in range(H)]

s = [[0] * (W+1) for _ in range(H+1)]
for i in range(H):
  for j in range(W):
    s[i+1][j+1] = s[i][j+1] + s[i+1][j] - s[i][j] + X[i][j]

q = int(input())
for _ in range(q):
  a, b, c, d = map(int, input().split())
  ans = s[c][d] - s[c][b-1] - s[a-1][d] + s[a-1][b-1]
  print(ans)
