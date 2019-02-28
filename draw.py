#!/bin/python3
import numpy as np
import matplotlib.pyplot as plt

data = np.genfromtxt(r'./data.txt', delimiter=',')
x, y = data.T
plt.scatter(x, y)
plt.show()
