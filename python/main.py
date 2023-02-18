import matplotlib.pyplot as plt


x = []
y = []

with open('../data/max_steps.txt', 'r') as f:
    for line in f:
        x.append(int(line))
        
with open('../data/max_step_gap.txt', 'r') as f:
    for line in f:
        y.append(int(line))
        

xmin, xmax = xlim = 0, max(x) + max(x) * 0.1
ymin, ymax = ylim = 0, max(y) + max(y) * 0.1

fig, ax = plt.subplots()
ax.set(xlim=xlim, ylim=ylim, autoscale_on=False)

    

# plot the data into a bar chart

ax.bar(x, y, width=0.01, align='center', color='blue', edgecolor='black')

ax.set_aspect('auto')
plt.show()