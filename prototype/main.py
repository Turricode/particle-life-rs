import numpy as np
import matplotlib.pyplot as plt
import random

def generate_random_vec2():
    return (random.uniform(-1, 1), random.uniform(-1, 1)) 

def generate_random_particles(size, n_types):
    return [(generate_random_vec2(), generate_random_vec2(), random.randrange(n_types)) for _ in range(size)] 



def main():
    partciles = generate_random_particles(20, 3)
    points, velocities, fams = zip(*partciles)
    # print(list(points))
    # print(fams) 
    print(list(zip(points)))
    # plt.scatter(x, y)

    # plt.xlim([-1, 1])
    # plt.ylim([-1, 1])
    # plt.show()

if __name__ == '__main__':
    main()
