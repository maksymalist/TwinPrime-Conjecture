import matplotlib.pyplot as plt
from lib import get_x_coords, get_y_coords, draw_vec

x = get_x_coords()
y = get_y_coords()
        

xmin, xmax = xlim = 0, max(x) + max(x) * 0.1
ymin, ymax = ylim = 0, max(y) + max(y) * 0.1

fig, ax = plt.subplots()

ax.set(xlim=xlim, ylim=ylim, autoscale_on=False)
ax.bar(x, y, width=0.01, align='center', color='blue', edgecolor='black')
ax.set_aspect('auto')

draw_vec()

plt.show()