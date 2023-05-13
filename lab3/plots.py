def read_float(bin_pack, distr):
    filename = f"data/dist_{distr}bp_{bin_pack}.txt"
    file = open(filename, 'r')
    line = file.readline()
    return round(float(line), 3)

import numpy as np
import matplotlib.pyplot as plt

def plot_heatmap():
    bin_packs = [ "BEST", "FIRST", "RANDOM",  "WORST", "NEXT" ]
    distrs    = [ "uniform", "harmonic", "biharmonic", "geometric" ]

    data = [ [ read_float(bp, dist) for bp in bin_packs ] for dist in distrs ]
    data = np.array(data)

    fig, ax = plt.subplots()
    im = ax.imshow(data)

    # We want to show all ticks...
    ax.set_xticks(np.arange(len(bin_packs)))
    ax.set_yticks(np.arange(len(distrs)))
    # ... and label them with the respective list entries
    ax.set_xticklabels(bin_packs)
    ax.set_yticklabels(distrs)

    # Rotate the tick labels and set their alignment.
    plt.setp(ax.get_xticklabels(), rotation=45, ha="right",
            rotation_mode="anchor")

    # Loop over data dimensions and create text annotations.
    for i in range(len(distrs)):
        for j in range(len(bin_packs)):
            ax.text(j, i, data[i, j], ha="center", va="center", color="w")

    ax.set_title("Average competitive ratio")
    fig.tight_layout()
    plt.show()

if __name__ == "__main__":
    plot_heatmap()