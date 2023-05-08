
N, M = map(int, input().split())
balances = list(map(int, input().split()))
prices = list(map(int, input().split()))

for i in range(1, N+1):
  c = 0
  for j in range(1, M+1):
    if balances[i-1] >= prices[j-1]:
      c += 1
  print(c)
