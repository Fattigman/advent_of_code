biggest = 0
temp = 0
biggest_list = []
for line in  open('input.txt').readlines():
    if line.strip() == '':
        if len(biggest_list) < 3 or temp > min(biggest_list):
            if len(biggest_list) < 3:
                biggest_list.append(temp)
            else:
                biggest_list[biggest_list.index(min(biggest_list))] = temp
        biggest = max(biggest, temp)
        temp = 0
    else:
        temp += int(line)
print(biggest)
print(sum(biggest_list))