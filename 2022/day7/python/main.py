import networkx as nx
import matplotlib.pyplot as plt

G = nx.DiGraph()
G.add_node('/')
current_node = '/'
for counter, line in enumerate(open('../input.txt').readlines()):
    try:
        line = line.split()
        if line[0] == '$':
            if '..' in line:
                print('Current node: ', current_node, ' Next node: ', [x for x in G.predecessors(current_node)])
                current_node = [x for x in G.predecessors(current_node)][0]
            if 'cd' in line and '/' not in line and '..' not in line:
                current_node = line[2]
        elif line[0] == 'dir':
            G.add_edges_from([(current_node, line[1])])
        else:
            G.add_weighted_edges_from([(current_node, line[1], int(line[0]))])
    except IndexError:
        print (counter)

nx.draw(G, with_labels=True)
plt.draw()

while True:
    plt.pause(0.1)