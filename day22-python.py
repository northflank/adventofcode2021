import dataclasses
import typing


@dataclasses.dataclass
class Cuboid:
    x: range
    y: range
    z: range

    def volume(self):
        return len(self.x) * len(self.y) * len(self.z)

    def __mul__(self, other) -> typing.Union['Cuboid', None]:
        r = Cuboid(
            range(max(self.x.start, other.x.start), min(self.x.stop, other.x.stop)),
            range(max(self.y.start, other.y.start), min(self.y.stop, other.y.stop)),
            range(max(self.z.start, other.z.start), min(self.z.stop, other.z.stop))
        )

        if r.x.start >= r.x.stop or r.y.start >= r.y.stop or r.z.start >= r.z.stop:
            return Cuboid(range(0, 0), range(0, 0), range(0, 0))

        return r

    def __sub__(self, other: 'Cuboid') -> 'Shape':
        if (self * other).volume() == 0:
            return Shape([self])

        cuboids = []

        for e in other.exterior():
            exterior_cuboid = self * e
            if exterior_cuboid.volume() > 0:
                cuboids.append(exterior_cuboid)

        return Shape(cuboids)

    def exterior(self):
        c = 10 ** 9

        yield Cuboid(
            range(-c, self.x.start),
            range(-c, c),
            range(-c, c),
        )

        yield Cuboid(
            range(self.x.stop, c),
            range(-c, c),
            range(-c, c),
        )

        yield Cuboid(
            range(self.x.start, self.x.stop),
            range(-c, self.y.start),
            range(-c, c),
        )

        yield Cuboid(
            range(self.x.start, self.x.stop),
            range(self.y.stop, c),
            range(-c, c),
        )

        yield Cuboid(
            range(self.x.start, self.x.stop),
            range(self.y.start, self.y.stop),
            range(-c, self.z.start),
        )

        yield Cuboid(
            range(self.x.start, self.x.stop),
            range(self.y.start, self.y.stop),
            range(self.z.stop, c),
        )


@dataclasses.dataclass
class Shape:
    cuboids: typing.List[Cuboid]

    def __add__(self, other: typing.Union['Cuboid', 'Shape']):
        if isinstance(other, Cuboid):
            other = Shape([other])

        return Shape([*self.cuboids, *(other - self).cuboids])

    def __sub__(self, other: typing.Union['Cuboid', 'Shape']) -> 'Shape':
        if isinstance(other, Cuboid):
            cuboids = []
            for c in self.cuboids:
                cuboids.extend((c - other).cuboids)
            return Shape(cuboids)

        if isinstance(other, Shape):
            new_shape = self
            for c in other.cuboids:
                new_shape = new_shape - c
            return new_shape

    def __mul__(self, other: 'Cuboid'):
        cuboids = [c * other for c in self.cuboids]
        return Shape([c for c in cuboids if c.volume() > 0])

    def volume(self):
        return sum(c.volume() for c in self.cuboids)


with open('input-day-22.txt') as f:
    lines = f.readlines()

    shape = Shape([])

    for i, line in enumerate(lines):
        if line.strip() == "":
            continue

        state, line = line.split(" ")
        ranges = line.split(",")
        ranges = [r.split("=")[1].split("..") for r in ranges]
        ranges = [range(int(f), int(t) + 1) for f, t in ranges]

        cuboid = Cuboid(*ranges)

        print(f"{i} / {len(lines)}", state, len(shape.cuboids))

        if state == "on":
            shape += cuboid

        if state == "off":
            shape -= cuboid

    shape_50 = shape * Cuboid(range(-50, 51), range(-50, 51), range(-50, 51))

    print("Task 1:", shape_50.volume())
    print("Task 2:", shape.volume())
