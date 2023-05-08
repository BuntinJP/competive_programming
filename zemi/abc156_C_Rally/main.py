

def calc_cost(X, p):
  """
  X: 住人の座標を収納した整数値'リスト'
  p: 集会場所の座標
  """
  cost = 0
  for x in X:
    cost += (x - p) ** 2
  return cost


X = list(map(int, input().split()))
n = int(input())

start, end = min(X), max(X)
# start, end = 1, 100
ans = 10 ** 10

for i in range(start, end+1):
  ans = min(ans, calc_cost(X, i))
  pass

print(ans)
