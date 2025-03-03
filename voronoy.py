import math

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.tri as tri

# Generate random points
np.random.seed(42)
points = np.random.rand(15, 2)

# Compute Delaunay triangulation
triang = tri.Triangulation(points[:, 0], points[:, 1])

# Compute circumcenters
circumcenters = np.zeros((len(triang.triangles), 2))

for i, t in enumerate(triang.triangles):
    pts = points[t]  # Triangle vertices
    A = np.column_stack((pts, np.ones(3)))  # Augmented matrix
    b = np.sum(pts**2, axis=1)
    circumcenters[i] = np.linalg.solve(A, b)[:2] / 2  # Compute circumcenter

# Create Voronoi edges from Delaunay neighbors
voronoi_edges = []
infinite_edges = []

def vector_length(v):
    diff = v[1] - v[0]  # Разница координат
    return np.linalg.norm(diff)

for i, t in enumerate(triang.triangles):
    for j in range(3):
        neighbor = triang.neighbors[i][j]
        if neighbor != -1:
            # Finite Voronoi edges
            c1, c2 = circumcenters[i], circumcenters[neighbor]
            voronoi_edges.append((c1, c2))
        else:
            # Infinite Voronoi edge
            p1, p2 = points[t[j]], points[t[(j+1) % 3]]
            midpoint = (p1 + p2) / 2
            direction = midpoint - circumcenters[i]
            alt_direction = direction * -1
            direction /= np.linalg.norm(direction)
            alt_direction /= np.linalg.norm(alt_direction)
            far_point = circumcenters[i] + direction * 2  # Extend outward
            alt_far_point = circumcenters[i] + alt_direction * 2  # Extend outward
            infinite_edges.append((circumcenters[i], far_point))
            infinite_edges.append((circumcenters[i], alt_far_point))
            len_1 = vector_length((circumcenters[i], far_point))
            len_2 = vector_length(circumcenters[i] + alt_direction * 2)
            pass

# Plot Voronoi diagram
fig, ax = plt.subplots(figsize=(6, 6))

# Plot Delaunay triangulation
ax.triplot(points[:, 0], points[:, 1], triang.triangles, color='gray', linestyle='dotted')

# Draw finite Voronoi edges
for c1, c2 in voronoi_edges:
    ax.plot([c1[0], c2[0]], [c1[1], c2[1]], 'b-', linewidth=1.5)

# Draw infinite Voronoi edges as dashed lines
for c1, c2 in infinite_edges:
    ax.plot([c1[0], c2[0]], [c1[1], c2[1]], 'b--', linewidth=1.5)

# Plot original points
ax.plot(points[:, 0], points[:, 1], 'ro', markersize=5)

ax.set_xlim(0, 1)
ax.set_ylim(0, 1)
ax.set_title("Voronoi Diagram with Infinite Edges (without scipy.spatial)")
plt.show()
