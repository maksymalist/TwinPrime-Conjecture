import numpy as np
import matplotlib.pyplot as plt

def main():
    fig, ax = plt.subplots()  # Create a figure containing a single axes.

    y_data = [0, 4, 6, 6, 12, 6, 36, 78, 18, 42, 72, 90, 126, 132, 294, 6, 78, 444, 60, 18, 192]
    x_data = [i for i in range(len(y_data))]

    ax.plot(x_data, y_data)

    plt.show()


if __name__ == '__main__':
    main()