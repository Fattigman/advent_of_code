with open("../data/input.txt", "r") as file:
    lines = file.readlines()

    coordinate_1 = sorted([int(line.strip("\n").split("   ")[0]) for line in lines])
    coordinate_2 = sorted([int(line.strip("\n").split("   ")[1]) for line in lines])

    print(sum([abs(coordinate_1[i] - coordinate_2[i]) for i, coord in enumerate(coordinate_1)]))