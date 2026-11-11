"""Seeded gradient Perlin noise with quintic interpolation (no dependencies).

Algorithm reference: Ken Perlin, Improved Noise (2002),
https://cs.nyu.edu/~perlin/noise/ . The seeded lattice hash is project-specific.
"""
import math

GRADIENTS = ((1,1,0),(-1,1,0),(1,-1,0),(-1,-1,0),
             (1,0,1),(-1,0,1),(1,0,-1),(-1,0,-1),
             (0,1,1),(0,-1,1),(0,1,-1),(0,-1,-1))

def hash32(x):
    x &= 0xffffffff
    x = ((x ^ (x >> 16)) * 0x7feb352d) & 0xffffffff
    x = ((x ^ (x >> 15)) * 0x846ca68b) & 0xffffffff
    return x ^ (x >> 16)

class Perlin3:
    def __init__(self, seed=2026, period=256):
        self.seed, self.period = seed, period

    def __call__(self, x, y, z):
        cell = (math.floor(x), math.floor(y), math.floor(z))
        frac = (x-cell[0], y-cell[1], z-cell[2])
        fade = tuple(t*t*t*(t*(t*6-15)+10) for t in frac)
        result = 0.
        for dz in (0, 1):
            for dy in (0, 1):
                for dx in (0, 1):
                    corner = (dx,dy,dz)
                    a,b,c = ((cell[i]+corner[i]) % self.period for i in range(3))
                    h = hash32(self.seed ^ hash32(a) ^ hash32(b+0x9e3779b9) ^ hash32(c+0x85ebca6b))
                    grad = GRADIENTS[h % len(GRADIENTS)]
                    dot = sum(grad[i]*(frac[i]-corner[i]) for i in range(3)) / math.sqrt(2)
                    weight = math.prod(fade[i] if corner[i] else 1-fade[i] for i in range(3))
                    result += dot*weight
        return result

    def fbm(self, x, y, z, octaves=3):
        value = total = 0.
        amplitude = 1.
        for _ in range(octaves):
            value += amplitude*self(x,y,z)
            total += amplitude
            x,y,z = x*2,y*2,z*2
            amplitude *= .5
        return value/total

def scatter(count, seed, allowed=lambda n: True, exclusions=()):
    """Rank spherical candidates by coherent noise, then enforce spacing.

    Candidate sites cover the sphere; Perlin determines which survive. Seed
    changes the actual positions, not just instance colors or their ordering.
    """
    noise = Perlin3(seed)
    candidates = []
    for k in range(1024):
        y = 1-2*(k+.5)/1024
        angle = k*math.pi*(3-math.sqrt(5))
        radius = math.sqrt(1-y*y)
        n = (radius*math.cos(angle), y, radius*math.sin(angle))
        if not allowed(n): continue
        if any(sum((n[i]-e[i])**2 for i in range(3)) < .48**2 for e in exclusions): continue
        score = noise.fbm(n[0]*2.4+17.3,n[1]*2.4+9.1,n[2]*2.4+5.7)
        candidates.append((score,k,n))
    selected = []
    for score,k,n in sorted(candidates, reverse=True):
        if all(sum((n[i]-s[2][i])**2 for i in range(3)) >= .38**2 for s in selected):
            selected.append((score,k,n))
            if len(selected) == count: break
    if len(selected) != count:
        raise ValueError(f'Only {len(selected)} sites for {count} instances')
    return selected
