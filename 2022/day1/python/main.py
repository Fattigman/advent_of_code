biggest = 0
temp = 0
for line in  open('input.txt').readlines():
    if line.strip() == '':
        biggest = max(biggest, temp)
        temp = 0
    else:
        temp += int(line)
print(biggest)