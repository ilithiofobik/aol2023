import numpy as np
import matplotlib.pyplot as plt

def read_float(page_struct, migration, distr):
    filename = f"data/{distr}_{migration}_{page_struct}.txt"
    file = open(filename, 'r')
    line = file.readline()
    return float(line)

def plot_heatmap_pagestruct(page_struct):
    migrations = [ "move_to_min", "random_flip" ]
    distrs        = [ "uniform", "harmonic", "biharmonic" ]

    data = [ [ read_float(page_struct, migr, dist) for migr in migrations ] for dist in distrs ]
    data = np.array(data)

    fig, ax = plt.subplots()
    im = ax.imshow(data)

    # We want to show all ticks...
    ax.set_xticks(np.arange(len(migrations)))
    ax.set_yticks(np.arange(len(distrs)))
    # ... and label them with the respective list entries
    ax.set_xticklabels(migrations)
    ax.set_yticklabels(distrs)

    # Rotate the tick labels and set their alignment.
    plt.setp(ax.get_xticklabels(), rotation=45, ha="right",
            rotation_mode="anchor")

    # Loop over data dimensions and create text annotations.
    for i in range(len(distrs)):
        for j in range(len(migrations)):
            text = ax.text(j, i, data[i, j],
                        ha="center", va="center", color="w")

    ax.set_title(f"Average competitive ratio for {page_struct}")
    fig.tight_layout()
    #plt.show()
    plt.savefig(f'data/{page_struct}_heatmap.png')

def plot_heatmap():
    for page_struct in [ "torus", "hypercube" ]:
        plot_heatmap_pagestruct(page_struct)

if __name__ == "__main__":
    plot_heatmap()