import numpy as np
import matplotlib.pyplot as plt
import csv 

ds = [ 16, 32, 64, 128, 256 ]
ps = [ 0.01, 0.02, 0.05, 0.1, 0.2, 0.5 ]

def read_data(parameters):
    data = { parameter: np.zeros((len(ds),len(ps))) for parameter in parameters }

    with open('data/allocation_experiment.csv') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            d = int(row['d'])
            p = float(row['p'])
            i = ds.index(d)
            j = ps.index(p)
            for parameter in parameters:
                data[parameter][i][j] = float(row[parameter])

    return data

def plot_linear(parameter, data):
    plt.figure().set_figwidth(10)
    for i, d in enumerate(ds):
        plt.plot(ps, data[i][:], label=f'd={d}', marker='o')
    plt.xlabel('p')
    plt.ylabel(parameter)
    plt.legend()
    plt.savefig(f'data/{parameter}_linear.png')
    plt.close()

if __name__ == "__main__":
    parameters = ["avg_cost", "avg_max_copies"]
    data = read_data(parameters)
    for parameter in parameters:
        plot_linear(parameter, data[parameter])
