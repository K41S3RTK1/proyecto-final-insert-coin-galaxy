"""Create a seamless tangent-space RGB normal map; Python standard library only."""
from pathlib import Path
import math, struct, zlib
from noise3d import Perlin3

def build(path, size=128):
    noise = Perlin3(2026, period=4)
    height = [[noise.fbm(x/size*4,y/size*4,.37) for x in range(size)] for y in range(size)]
    rows = bytearray()
    for y in range(size):
        rows.append(0)
        for x in range(size):
            dx = (height[y][(x+1)%size]-height[y][(x-1)%size])*size*.16
            dy = (height[(y+1)%size][x]-height[(y-1)%size][x])*size*.16
            length = math.sqrt(dx*dx+dy*dy+1)
            rows.extend(round((v/length*.5+.5)*255) for v in (-dx,-dy,1))
    def chunk(kind, data):
        return struct.pack('>I',len(data))+kind+data+struct.pack('>I',zlib.crc32(kind+data)&0xffffffff)
    png = b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>2I5B',size,size,8,2,0,0,0))+chunk(b'IDAT',zlib.compress(rows))+chunk(b'IEND',b'')
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_bytes(png)
    print(path.name, size, 'x', size, 'RGB tangent-space normal map')

if __name__ == '__main__':
    build(Path(__file__).resolve().parents[1]/'assets/textures/terrain-normal.png')
