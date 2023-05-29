N = int(input().strip())
inputs = list(map(int, input().strip().split()))

s = [0] * (N + 1)
for i, x in enumerate(inputs):
  s[i + 1] = s[i] + x

for i in range(1, N + 1):
  max_val = max(s[j] - s[j - i] for j in range(i, N + 1))
  print(max_val)
