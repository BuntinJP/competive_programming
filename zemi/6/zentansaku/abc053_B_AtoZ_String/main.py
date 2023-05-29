x = list(input())
l = 0
f = 0

for i in range(len(x)):
  if (x[i] == 'A'):
    f = i
    break

for j in range(len(x)):
  if (x[j] == 'Z'):
    l = j

print(l-f+1)
