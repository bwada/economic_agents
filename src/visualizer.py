import numpy as np
import matplotlib.pyplot as plt

data = np.loadtxt("sim_results/specialization_1.txt")
print(data)

fig = plt.figure()
ax = fig.add_subplot(projection = "3d")
ax.set_xlim(0,10)
ax.set_ylim(0,10)
ax.set_zlim(0,10)
ax.scatter(data[:,0], data[:,1], data[:,2])
ax.view_init(30,30)
plt.savefig("sim_results/pytest.png")