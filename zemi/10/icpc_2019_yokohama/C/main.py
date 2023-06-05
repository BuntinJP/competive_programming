def enumerateWeight(idx, _sum, comb, w):
  if idx < 0:
    comb.add(_sum)
    return
  for select in range(-1, 2):
    enumerateWeight(idx - 1, _sum + select * w[idx], comb, w)


def solve(n, m):
  # Input
  a = list(map(int, input().split()))
  w = list(map(int, input().split()))
  comb = set()
  enumerateWeight(m - 1, 0, comb, w)
  can_measure = all(a_i in comb for a_i in a)
  if can_measure:
    return 0
  min_add_w = float('inf')
  idx = 0
  while idx < n and a[idx] in comb:
    idx += 1
  for _sum in comb:
    add_w = abs(_sum - a[idx])
    can_measure = all(a_i in comb or a_i - add_w in comb or a_i + add_w in comb for a_i in a)
    if can_measure:
      min_add_w = min(min_add_w, add_w)
  return min_add_w if min_add_w < float('inf') else -1


while True:
  n, m = map(int, input().split())
  if n == 0:
    break
  print(solve(n, m))
