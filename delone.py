import numpy as np
import matplotlib.pyplot as plt
from math import sqrt

# Function to check if a point is inside the circumcircle of a triangle
def circumcircle(p1, p2, p3, p):
    ax, ay = p1
    bx, by = p2
    cx, cy = p3
    dx, dy = p

    # Compute the determinant of the matrix of the triangle and the point
    matrix = np.array([[ax - dx, ay - dy, (ax - dx)**2 + (ay - dy)**2],
                       [bx - dx, by - dy, (bx - dx)**2 + (by - dy)**2],
                       [cx - dx, cy - dy, (cx - dx)**2 + (cy - dy)**2]])
    return np.linalg.det(matrix) > 0

# Function to implement the Delaunay Triangulation algorithm
def delaunay(points):
    # Supertriangle that will contain all the points in the set
    xmin, ymin = np.min(points, axis=0)
    xmax, ymax = np.max(points, axis=0)
    dx = xmax - xmin
    dy = ymax - ymin
    supertriangle = np.array([[xmin - 10 * dx, ymin - 10 * dy],
                              [xmax + 10 * dx, ymin - 10 * dy],
                              [xmin - 10 * dx, ymax + 10 * dy]])

    # List of triangles (initially contains the supertriangle)
    triangles = [supertriangle]

    # Iterate over each point and perform the triangulation
    for p in points:
        edges = []
        bad_triangles = []
        
        # Find all triangles that are no longer valid (share an edge with the point)
        for i, triangle in enumerate(triangles):
            if circumcircle(triangle[0], triangle[1], triangle[2], p):
                bad_triangles.append(i)
                edges.extend([(tuple(triangle[0]), tuple(triangle[1])),
                              (tuple(triangle[1]), tuple(triangle[2])),
                              (tuple(triangle[2]), tuple(triangle[0]))])

        # Remove the bad triangles from the list
        triangles = [t for i, t in enumerate(triangles) if i not in bad_triangles]

        # Remove duplicate edges (edges that are shared between two triangles)
        edges = list(set(edges))

        # Create new triangles by adding the point to each edge
        for edge in edges:
            triangles.append(np.array([edge[0], edge[1], p]))

    # Remove the supertriangle triangles
    triangles = [t for t in triangles if not any(np.array_equal(t[0], supertriangle[0]) or np.array_equal(t[1], supertriangle[1]) or np.array_equal(t[2], supertriangle[2]))]

    return triangles

# Example points
points = np.random.rand(10, 2)

# Perform Delaunay triangulation
triangles = delaunay(points)

# Plot the result
plt.figure(figsize=(8, 8))
for triangle in triangles:
    x = triangle[:, 0]
    y = triangle[:, 1]
    plt.plot(np.append(x, x[0]), np.append(y, y[0]), 'b-')

plt.plot(points[:, 0], points[:, 1], 'ro')  # plot the points
plt.title("Delaunay Triangulation (Bowyer-Watson)")
plt.show()
