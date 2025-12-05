with open('../data/input.txt', 'r') as file:
          position = 50
          ans = 0
          for line in file.readlines():
            if line[0] == "L":
                position -= (int(line[1:]) % 100)
            else:
                position += (int(line[1:]) % 100)
            if position % 100 == 0:
                ans +=1
print(ans)
