
import numpy as np
import opensimplex


SIZE = 32 # IMPORTANT: This must match in the Rust code that reads the binary file.


def make_test_tex():
    voxels = np.ndarray((SIZE, SIZE, SIZE, 3), float)

    opensimplex.seed(294)
    scale = 20.0 / SIZE

    print("Sampling simplex noise")

    for x in range(SIZE):
        for y in range(SIZE):
            for z in range(SIZE):
                r = opensimplex.noise3(x * scale, y * scale, z * scale)
                g = opensimplex.noise3(x * scale, y * scale, z * scale + 100_000_000.0)
                b = opensimplex.noise3(x * scale, y * scale, z * scale + 200_000_000.0)

                r = 0.5 + 0.5 * r
                g = 0.5 + 0.5 * g
                b = 0.5 + 0.5 * b
                
                voxels[x, y, z, 0] = r
                voxels[x, y, z, 1] = g
                voxels[x, y, z, 2] = b

    print("Doing other stuff")

    for _ in range(10):
        old_voxels = voxels.copy()
        for x in range(SIZE):
            for y in range(SIZE):
                for z in range(SIZE):
                    d = old_voxels[x, y, z].copy()
                    d *= 2.0
                    
                    smp = sample(old_voxels, x + d[0], y + d[1], z + d[2])

                    voxels[x, y, z, 0] = smp[0]
                    voxels[x, y, z, 1] = smp[1]
                    voxels[x, y, z, 2] = smp[2]

    return voxels


# Samples it bilinearly in 3D (trilinear interpolation)
def sample(voxels: np.ndarray, x, y, z):
    x = float(x)
    y = float(y)
    z = float(z)

    max_x = voxels.shape[0] - 1
    max_y = voxels.shape[1] - 1
    max_z = voxels.shape[2] - 1

    x = np.clip(x, 0.0, max_x)
    y = np.clip(y, 0.0, max_y)
    z = np.clip(z, 0.0, max_z)

    x0 = int(np.floor(x))
    y0 = int(np.floor(y))
    z0 = int(np.floor(z))

    x1 = min(x0 + 1, max_x)
    y1 = min(y0 + 1, max_y)
    z1 = min(z0 + 1, max_z)

    xd = x - x0
    yd = y - y0
    zd = z - z0

    c000 = voxels[x0, y0, z0]
    c100 = voxels[x1, y0, z0]
    c010 = voxels[x0, y1, z0]
    c110 = voxels[x1, y1, z0]
    c001 = voxels[x0, y0, z1]
    c101 = voxels[x1, y0, z1]
    c011 = voxels[x0, y1, z1]
    c111 = voxels[x1, y1, z1]

    c00 = c000 * (1.0 - xd) + c100 * xd
    c10 = c010 * (1.0 - xd) + c110 * xd
    c01 = c001 * (1.0 - xd) + c101 * xd
    c11 = c011 * (1.0 - xd) + c111 * xd

    c0 = c00 * (1.0 - yd) + c10 * yd
    c1 = c01 * (1.0 - yd) + c11 * yd

    return c0 * (1.0 - zd) + c1 * zd


def to_serialized(voxels: np.ndarray):
    print("Serializing")
    
    ret = bytearray()

    for x in range(voxels.shape[0]):
        for y in range(voxels.shape[1]):
            for z in range(voxels.shape[2]):
                color = voxels[z, y, x] # IMPORTANT: switches order of dimensions here
                r = np.float16(color[0]).view(np.uint16).tobytes()
                g = np.float16(color[1]).view(np.uint16).tobytes()
                b = np.float16(color[2]).view(np.uint16).tobytes()
                a = np.float16(1.0).view(np.uint16).tobytes()

                ret.extend([r[0], r[1]])
                ret.extend([g[0], g[1]])
                ret.extend([b[0], b[1]])
                ret.extend([a[0], a[1]])

    return ret


def main():
    with open("./assets/generated/textures/test_tex.bin", "wb") as f:
        data = to_serialized(make_test_tex())
        print("Writing to file")
        f.write(data)


main()
