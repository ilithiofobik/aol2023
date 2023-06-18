import numpy as np
import matplotlib.pyplot as plt

ds = [ 16, 32, 64, 128, 256 ]
ps = [ 0.01, 0.02, 0.05, 0.1, 0.2, 0.5 ]

def plot_linear():
    cost = np.zeros((len(ds),len(ps)))
    copies = np.zeros((len(ds),len(ps)))

    for i in range(len(ds)):
        for j in range(len(ps)):             
            filename = f"data/d_{ds[i]}_p_{ps[j]}_allocation.txt"
            file = open(filename, 'r')
            line = file.readline().split(",")
            cost[i][j]   = int(float(line[0]))
            copies[i][j] = float(line[1])
  
    plt.figure().set_figwidth(10)
    for i,d in enumerate(ds):
        plt.plot(ps, cost[i][:], label=d, marker='o')
    plt.xlabel('p')
    plt.ylabel('avg_cost')
    plt.legend()
    plt.savefig("data/cost_ds.png", dpi=300)
    plt.close()

    plt.figure().set_figwidth(10)
    for i,d in enumerate(ds):
        plt.plot(ps, copies[i][:], label=d, marker='o')
    plt.xlabel('p')
    plt.ylabel('max_copies')
    plt.legend()
    plt.savefig("data/copies_ds.png", dpi=300)
    plt.close()

def read_floats(d, p):
    filename = f"data/d_{d}_p_{p}_allocation.txt"
    file = open(filename, 'r')
    line = file.readline()
    return [ float (x) for x in line.split(",") ]

def plot_heatmap(parameter, idx):
    data = [ [ (read_floats(d, p))[idx] for d in ds ] for p in ps ]
    data = [ [int(value) for value in row] for row in data ] if parameter == "avg_cost" else data
    data = np.array(data)

    fig, ax = plt.subplots()
    im = ax.imshow(data)

    # We want to show all ticks...
    ax.set_xticks(np.arange(len(ds)))
    ax.set_yticks(np.arange(len(ps)))
    # ... and label them with the respective list entries
    ax.set_xticklabels(ds)
    ax.set_yticklabels(ps)

    # Rotate the tick labels and set their alignment.
    plt.setp(ax.get_xticklabels(), rotation=45, ha="right",
            rotation_mode="anchor")

    # Loop over data dimensions and create text annotations.
    for i in range(len(ps)):
        for j in range(len(ds)):
            text = ax.text(j, i, data[i, j],
                        ha="center", va="center", color="w")

    ax.set_title(parameter)
    fig.tight_layout()
    #plt.show()
    plt.savefig(f'data/{parameter}_heatmap.png')

if __name__ == "__main__":
    for (parameter, idx) in [ ("avg_cost", 0), ("avg_copies", 1) ]:
        plot_heatmap(parameter, idx)
    plot_linear()