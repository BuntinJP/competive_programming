from collections import defaultdict, deque


def solve():
  N = int(input())
  edges = [list(map(int, input().split())) for _ in range(N - 1)]

  # Create an adjacency list representation of the graph
  graph = defaultdict(list)
  for i, (a, b) in enumerate(edges):
    a -= 1
    b -= 1
    graph[a].append((b, i))
    graph[b].append((a, i))

  # Initialize the color of each edge to be 0
  edge_colors = [0] * (N - 1)

  # Depth-first search to color the edges
  stack = deque([(0, -1, -1)])  # (node, parent, parent_color)
  while stack:
    node, parent, parent_color = stack.pop()
    color = 1
    for neighbor, i in graph[node]:
      if neighbor == parent:
        continue
      if color == parent_color:
        color += 1
      edge_colors[i] = color
      stack.append((neighbor, node, color))  # type: ignore
      color += 1

  print(max(edge_colors))
  for color in edge_colors:
    print(color)


solve()
