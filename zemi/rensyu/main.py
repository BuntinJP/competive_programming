from collections import deque

K = int(input().strip())

queue = deque(range(1, 10))

for _ in range(K - 1):
  x = queue.popleft()
  mod = x % 10
  if mod != 0:
    queue.append(10 * x + mod - 1)
  queue.append(10 * x + mod)
  if mod != 9:
    queue.append(10 * x + mod + 1)

print(queue[0])
