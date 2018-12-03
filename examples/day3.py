#!python3.7

# https://adventofcode.com/2018/day/3
import re
from dataclasses import dataclass

from data import data

REGEX = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')


@dataclass(frozen=True)
class Area:
    id: int
    x: int
    y: int
    width: int
    height: int

    @property
    def xw(self):
        return self.x + self.width

    @property
    def yh(self):
        return self.y + self.height

    @property
    def inches(self):
        for x in range(self.x, self.xw):
            for y in range(self.y, self.yh):
                yield (x, y)


if __name__ == '__main__':
    lines = REGEX.findall(data)

    areas = [Area(*map(int, line)) for line in lines]
    for area in areas[:10]:
        print(area)
    width = max((area.xw for area in areas))
    height = max((area.yh for area in areas))

    inches_count = {k: 0 for k in Area(0, 0, 0, width, height).inches}

    for area in areas:
        for inch in area.inches:
            inches_count[inch] += 1

    print(f'overlap is {len([v for v in inches_count.values() if v > 1 ])}')

    for area in areas:
        found = True
        for inch in area.inches:
            if inches_count[inch] > 1:
                found = False
        if found:
            print(area.id)
