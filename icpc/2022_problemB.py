def main():
  while True:
    c = 0
    n = int(input())
    if n == 0:
      break
    v = drop_pairs([list(map(int, input().split())) for _ in range(n)])
    if check_all_is_empty(v):
      print(c)
      print('---')
      continue
    p = select_player(v)
    while check_all_is_empty(v) == False:
      print("p:" + str(p))
      pv = sorted(v[p])[0]
      v[p].remove(pv)
      c += 1
      v[next_player(v, p)].append(pv)
      v = drop_pairs(v)
      p = increment_player(v, p)
    print('---')
    print(c)


def next_player(v, p) -> int:
  new_p = p + 1
  if new_p == len(v):
    new_p = 0
  if len(v[new_p]) == 0:
    new_p = next_player(v, new_p)
  return new_p


def increment_player(v, p) -> int:
  new_p = p + 1
  if new_p >= len(v):
    select_player(v)
  if len(v[new_p]) == 0:
    new_p = increment_player(v, new_p)
  return new_p


def select_player(v) -> int:
  for i in range(len(v)):
    if len(v[i]) != 0:
      return i
  return -1


def check_all_is_empty(v):
  for i in range(len(v)):
    if len(v[i]) != 0:
      return False
  return True


def drop_pairs(v):
  new_v = []
  for i in range(len(v)):
    new_v.append(remove_pairs(v[i]))
  return new_v


def remove_pairs(lst):
  count_dict = {}
  for item in lst:
    if item in count_dict:
      count_dict[item] += 1
    else:
      count_dict[item] = 1
  return [item for item in lst if count_dict[item] != 2]


main()
