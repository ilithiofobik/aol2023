import matplotlib.pyplot as plt

dists = ["uniform", "harmonic", "biharmonic", "geometric"]
caches = [ "FIFO", "LFU", "LRU", "RMA", "RAND", "FWF" ]
colors = [ "r", "b", "g", "y", "m", "c" ]

def plot_by_k_for_dist_and_cache():
    n = 100
    ks = [ k for k in range(n // 10, (n // 5) + 1) ]
    
    for dist in dists:
        plt.clf()
        plt.xlabel('k')
        plt.ylabel('Avg cost')
        plt.title(f"Avg cost by k for {dist} distribution (n = 100)")
        for (cache, color) in zip(caches, colors):
            filename = f"data/n_100dist_{dist}cache_{cache}.txt"
            file = open(filename, 'r')
            lines = file.readlines()
            y = [ float((line.split(";"))[1]) for line in lines ]
            label = f"Cache = {cache}"
            plt.plot(ks, y, color, label=label, markersize=1)
        plt.legend()
        plt.savefig(f'data/{dist}.png')

def plot_by_n_for_dist_and_cache():
    ns = [ 20, 30, 40, 50, 60, 70, 80, 90, 100 ]
    
    for cache in caches:
        plt.clf()
        plt.xlabel('n')
        plt.ylabel('Avg cost')
        plt.title(f"Avg cost by n for {cache} cache (k = n / 5)")
        for (dist, color) in zip(dists, colors):
            y = [] 
            for n in ns:
                filename = f"data/n_{n}dist_{dist}cache_{cache}.txt"
                file = open(filename, 'r')
                lines = file.readlines()
                y.append( float((lines[-1].split(";"))[1]) )
            label = f"Distribution = {dist}"
            plt.plot(ns, y, color, label=label, markersize=1)
        plt.legend()
        plt.savefig(f'data/{cache}.png')

if __name__ == "__main__":
    plot_by_k_for_dist_and_cache()
    plot_by_n_for_dist_and_cache()
