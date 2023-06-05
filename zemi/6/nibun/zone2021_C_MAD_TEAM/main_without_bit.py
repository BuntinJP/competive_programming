left, right = 0, 10**9+1

N = int(input().strip())
A = [list(map(int, input().strip().split())) for _ in range(N)]


def check(x: int) -> bool:
  S = set()
  for a in A:
    bit_values = []
    for i in range(5):
      if a[i] >= x:
        bit_values.append(i)
    S.add(tuple(bit_values))
  for a in S:
    for b in S:
      for c in S:
        if len(set(a + b + c)) == 5:
          return True
  return False


while right - left > 1:
  mid = (left + right) // 2
  if check(mid):
    left = mid
  else:
    right = mid

print(left)
