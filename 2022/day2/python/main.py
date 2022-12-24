# Shitty solution, but it works
solving_dict = {'X':'Rock', 'Y':'Paper', 'Z':'Scissors',
                'A':'Rock', 'B':'Paper', 'C':'Scissors'}
wins = [['Rock', 'Paper'], ['Paper', 'Scissors'], ['Scissors', 'Rock']]
point_dict = {'Rock': 1, 'Paper': 2, 'Scissors': 3}
win, draw, loss, score = 6, 3, 0, 0

for line in open('../input.txt').readlines():
    line = [solving_dict[x] for x in line.split()]
    if line[0] == line[1]:
        score += draw + point_dict[line[1]]
    elif line in wins:
        score += win + point_dict[line[1]]
    else:
        score += point_dict[line[1]]
print(score)