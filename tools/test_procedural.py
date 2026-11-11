"""Reproducibility, continuity and baked placement/LOD checks. Standard library."""
import json
import math
from pathlib import Path
import unittest
import build_planets as planets
from noise3d import Perlin3

ROOT = Path(__file__).resolve().parents[1]

class ProceduralTests(unittest.TestCase):
    def test_noise_is_continuous_seeded_and_periodic(self):
        n=Perlin3(2026,4)
        self.assertEqual(n(.2,.4,.7),Perlin3(2026,4)(.2,.4,.7))
        self.assertNotAlmostEqual(n(.2,.4,.7),Perlin3(42,4)(.2,.4,.7))
        self.assertAlmostEqual(n(.2,.4,.7),n(4.2,-3.6,4.7))
        for axis in range(3):
            a=[.2,.4,.7];b=a[:];a[axis]=1-1e-6;b[axis]=1+1e-6
            self.assertLess(abs(n(*a)-n(*b)),1e-5)

    def test_manifest_matches_regeneration_and_both_lods(self):
        manifest=json.loads((ROOT/'assets/models/placements.json').read_text())
        planets.SEED=manifest['seed'];planets.placement_sites.cache_clear()
        for label,group in manifest['groups'].items():
            sites=planets.placement_sites(len(group['instances']),label)
            self.assertEqual([k for _,k,_ in sites],[i['candidate'] for i in group['instances']])
            for (score,k,n),saved in zip(sites,group['instances']):
                self.assertAlmostEqual(score,saved['noise'])
                self.assertEqual(list(n),saved['normal'])
                self.assertAlmostEqual(sum(v*v for v in n),1.)
            kind=label.removesuffix('_hill')
            prefix='green_hill_island' if label.endswith('_hill') else {'garden':'garden_detail','dojo':'city_tower','runner':'city','mario':'mushroom_detail'}[kind]
            for lod in ('','-low'):
                groups={line.strip() for line in (ROOT/f'assets/models/{kind}{lod}.obj').read_text().splitlines() if line.startswith('g ')}
                for _,k,_ in sites:self.assertIn(f'g {prefix}_{k}',groups)

    def test_seeds_change_positions_and_preserve_clearance(self):
        baseline=None
        for seed in (2026,42,7,1,2027):
            planets.SEED=seed;planets.placement_sites.cache_clear()
            for kind,count in (('garden',18),('dojo',22),('runner',10),('mario',14),('race_hill',10)):
                sites=planets.placement_sites(count,kind)
                self.assertEqual(len(sites),count)
                for i,(_,_,n) in enumerate(sites):
                    for _,_,b in sites[i+1:]:self.assertGreaterEqual(math.dist(n,b),.38)
                if kind=='garden':
                    ids=[k for _,k,_ in sites]
                    if baseline is None:baseline=ids
                    else:self.assertNotEqual(ids,baseline)

if __name__=='__main__':unittest.main()
