
import numpy as np


SIZE = 16


def make_test_tex():
    voxels = np.ndarray((SIZE, SIZE, SIZE, 3), float)

    for x in range(SIZE):
        for y in range(SIZE):
            for z in range(SIZE):
                r = y / SIZE
                g = y / SIZE
                b = y / SIZE
                
                voxels[x, y, z, 0] = r
                voxels[x, y, z, 1] = g
                voxels[x, y, z, 2] = b

    return voxels


def to_serialized(arr: np.ndarray):
    ret = bytearray()

    for x in range(arr.shape[0]):
        for y in range(arr.shape[1]):
            for z in range(arr.shape[2]):
                color = arr[z, y, x] # IMPORTANT: switches order of dimensions here
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
        f.write(to_serialized(make_test_tex()))


main()
