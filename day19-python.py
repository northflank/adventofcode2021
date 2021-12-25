from typing import List, Tuple, Union

import dataclasses
import functools
import itertools
import math

import numpy as np


@dataclasses.dataclass
class Scanner:
    name: str
    data: np.ndarray

    @functools.cached_property
    def fingerprints(self):
        result = []
        for a in self.data:
            for b in self.data:
                if tuple(a) != tuple(b):
                    result.append(a - b)

        return np.array(result)


@dataclasses.dataclass
class Orientation:
    flips: Tuple[int]
    perms: Tuple[int]

    def rotate(self, other: Union[np.ndarray, "Orientation"]):
        if isinstance(other, Orientation):
            def permute(perm, arr):
                return arr[perm[0]], arr[perm[1]], arr[perm[2]]

            # noinspection PyTypeChecker
            return Orientation(
                tuple(p * q for p, q in zip(self.flips, permute(self.perms, other.flips))),
                tuple(permute(self.perms, other.perms))
            )

        return np.column_stack([
            self.flips[0] * other[..., self.perms[0]],
            self.flips[1] * other[..., self.perms[1]],
            self.flips[2] * other[..., self.perms[2]]]
        )

    @staticmethod
    def all_with_vectors(arr: np.ndarray):
        for orientation in Orientation.all():
            yield orientation, orientation.rotate(arr)

    @staticmethod
    def all():
        def parity(flips, permutations):
            parity = math.prod(flips)
            seen = set()
            for i0, i1 in enumerate(permutations):
                if i0 != i1 and i0 not in seen:
                    parity *= -1
                    seen.add(i1)
            return parity

        # this lists all possible configurations of the axes and filters those
        # that correspond to proper rotations by checking the sign of the determinant
        # of the rotation matrix via the parity of
        for flips in itertools.product([-1, 1], repeat=3):
            for perms in itertools.permutations([0, 1, 2]):
                # determinant of rotation matrix must be positive 1
                # for proper rotation matrix
                if parity(flips, perms) == 1:
                    yield Orientation(flips, perms)


@dataclasses.dataclass
class Transform:
    offset: np.ndarray
    orientation: Orientation

    def apply(self, other: Union[np.ndarray, "Orientation"]):
        if isinstance(other, Transform):
            return Transform(
                self.orientation.rotate(other.offset) + self.offset,
                self.orientation.rotate(other.orientation))

        return self.orientation.rotate(other) + self.offset


def read_input(filepath):
    input_result = []

    with open(filepath) as f:
        lines = f.readlines()

    current_scanner = None

    def add_scanner():
        if current_scanner:
            input_result.append(
                Scanner(
                    name=current_scanner["name"],
                    data=np.array(current_scanner["data"])
                ))

    for line in lines:
        if "scanner" in line:
            add_scanner()
            scanner = line.replace("-", "").strip()
            current_scanner = {"name": scanner, "data": []}

        if "," in line:
            current_scanner["data"].append([int(a) for a in line.split(",")])

    add_scanner()

    return input_result


def vector_set(arr: np.ndarray):
    return set(map(tuple, arr))


def find_transform(scanner_1: Scanner, scanner_2: Scanner):
    fingerprints_1 = scanner_1.fingerprints
    fingerprints_2 = scanner_2.fingerprints

    for orientation, fingerprints_2 in Orientation.all_with_vectors(fingerprints_2):
        matches = vector_set(fingerprints_1) & vector_set(fingerprints_2)

        if not matches:
            continue

        points_1 = scanner_1.data
        points_1_set = vector_set(scanner_1.data)
        points_2 = orientation.rotate(scanner_2.data)

        for origin_1, origin_2 in itertools.product(points_1, points_2):
            offset = np.array(origin_1) - origin_2

            matches = points_1_set & vector_set(points_2 + offset)

            if len(matches) >= 12:
                return Transform(offset, orientation)


def stitch_beacons(scanners: List[Scanner]):
    graph = {}

    for scanner_1, scanner_2 in itertools.product(scanners, repeat=2):
        if scanner_1 == scanner_2:
            continue

        transform = find_transform(scanner_1, scanner_2)

        if transform:
            graph[scanner_1.name + "/" + scanner_2.name] = transform
            print(scanner_1.name, scanner_2.name, transform)

    while True:
        added = False
        for edge1, edge2 in itertools.product(graph, repeat=2):
            [a, b1] = edge1.split("/")
            [b2, c] = edge2.split("/")

            if b1 != b2:
                continue

            new_edge = f"{a}/{c}"

            if new_edge not in graph:
                graph[new_edge] = graph[edge1].apply(graph[edge2])
                added = True

        if not added:
            break

    full_map = []

    scanners_by_name = {scanner.name: scanner for scanner in scanners}

    for edge in graph:
        [a, b] = edge.split("/")

        if a != "scanner 0":
            continue

        transform = graph[edge]

        points = scanners_by_name[b].data
        points = transform.apply(points)

        full_map.extend(points)

    print("Task 1:")
    print(len(vector_set(full_map)))

    max_offset = 0
    for edge1, edge2 in itertools.product(graph, repeat=2):
        if not edge1.startswith("scanner 0") or not edge2.startswith("scanner 0"):
            continue

        transform1 = graph[edge1]
        transform2 = graph[edge2]

        offset = (transform1.offset - transform2.offset).sum()
        if offset > max_offset:
            max_offset = offset

    print()
    print("Task 2:")
    print(max_offset)


scanners = read_input("input-day-19.txt")

stitch_beacons(scanners)
