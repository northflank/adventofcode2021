import fs from 'fs';

type SnailfishNumberJSValue = [SnailfishNumberJSValue, SnailfishNumberJSValue] | number;

class SnailfishNumber {
  private constructor(private value: [SnailfishNumber, SnailfishNumber] | number) {}

  get isPair() {
    return Array.isArray(this.value);
  }

  get isNumber() {
    return !this.isPair;
  }

  add(other: SnailfishNumber) {
    return new SnailfishNumber([this.clone(), other.clone()]).reduce();
  }

  reduce() {
    let result = this;
    while (true) {
      const reduced = result.reduceIteration();
      if (!reduced) {
        return result;
      }
      result = reduced;
    }
  }

  get magnitude() {
    if (Array.isArray(this.value)) {
      return 3 * this.value[0].magnitude + 2 * this.value[1].magnitude;
    }
    return this.value;
  }

  reduceIteration() {
    const result = this.clone();

    const order = [] as SnailfishNumber[];
    const parents = new Map<SnailfishNumber, SnailfishNumber>();

    let explode: SnailfishNumber | undefined;
    let split: SnailfishNumber | undefined;

    // Build list of pre-order sorted nodes, parent relations and find
    // numbers that need to be exploded or split.
    result.traverse((v, depth, p) => {
      order.push(v);
      parents.set(v, p);
      if (!explode && v.isPair && depth >= 4) {
        explode = v;
      }
      if (!split && v.isNumber && v.value >= 10) {
        split = v;
      }
    });

    if (explode) {
      const [a, b] = explode.value as [SnailfishNumber, SnailfishNumber];
      const index = order.findIndex((n) => n === explode);

      const left = order
        .slice(0, index)
        .reverse()
        .find((n) => n.isNumber);

      // Assumes exploding pair to consist of two regular numbers
      const right = order.slice(index + 3).find((n) => n.isNumber);

      if (left) left.value += a.value as any;
      if (right) right.value += b.value as any;

      const parent = parents.get(explode);

      if (parent) {
        const [c1, c2] = parent.value as any;
        if (c1 === explode) parent.value[0] = new SnailfishNumber(0);
        if (c2 === explode) parent.value[1] = new SnailfishNumber(0);
      }
    } else if (split) {
      split.value = [
        SnailfishNumber.fromJSValue(Math.floor((split.value as number) / 2)),
        SnailfishNumber.fromJSValue(Math.ceil((split.value as number) / 2)),
      ];
    } else {
      return undefined;
    }

    return result;
  }

  traverse(
    f: (num: SnailfishNumber, depth: number, parent?: SnailfishNumber) => void,
    _parent?: SnailfishNumber,
    _depth?: number
  ) {
    const depth = _depth ?? 0;

    f(this, depth, _parent);

    if (Array.isArray(this.value)) {
      const [a, b] = this.value;
      a.traverse(f, this, depth + 1);
      b.traverse(f, this, depth + 1);
    }
  }

  clone() {
    if (Array.isArray(this.value)) {
      const [a, b] = this.value;
      return new SnailfishNumber([a.clone(), b.clone()]);
    }
    return new SnailfishNumber(this.value);
  }

  toString() {
    if (Array.isArray(this.value)) {
      return `[${this.value[0].toString()}, ${this.value[1].toString()}]`;
    }

    return `${this.value}`;
  }

  static fromJSValue(value: SnailfishNumberJSValue) {
    if (Array.isArray(value)) {
      const a = SnailfishNumber.fromJSValue(value[0]);
      const b = SnailfishNumber.fromJSValue(value[1]);
      return new SnailfishNumber([a, b]);
    }

    return new SnailfishNumber(value);
  }

  static parse(str: string) {
    return this.fromJSValue(JSON.parse(str));
  }
}

console.log(`${SnailfishNumber.parse('[[[[[9,8],1],2],3],4]').reduceIteration()}`);
console.log(`${SnailfishNumber.parse('[7,[6,[5,[4,[3,2]]]]]').reduceIteration()}`);
console.log(`${SnailfishNumber.parse('[[6,[5,[4,[3,2]]]],1]').reduceIteration()}`);
console.log(`${SnailfishNumber.parse('[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]').reduceIteration()}`);
console.log(`${SnailfishNumber.parse('[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]').reduceIteration()}`);
console.log(`${SnailfishNumber.parse('[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]').reduce()}`);

// const input = `
//     [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
//     [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
//     [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
//     [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
//     [7,[5,[[3,8],[1,4]]]]
//     [[2,[2,2]],[8,[8,1]]]
//     [2,9]
//     [1,[[[9,3],9],[[9,0],[0,7]]]]
//     [[[5,[7,4]],7],1]
//     [[[[4,2],2],6],[8,7]]`

const input = fs.readFileSync('input-day-18.txt', 'utf-8');

const numbers = input
  .split('\n')
  .filter((a) => a.trim() !== '')
  .map((a) => SnailfishNumber.parse(a));

const result = numbers.reduce((a, b) => {
  const r = a.add(b);

  console.log(`   ${a.toString().replace(/ /g, '')}`);
  console.log(` + ${b.toString().replace(/ /g, '')}`);
  console.log(` = ${r.toString().replace(/ /g, '')}`);

  return r;
});

console.log('Task 1:', result.magnitude);

let largestMagnitude = 0;

for (const a of numbers) {
  for (const b of numbers) {
    const m = a.add(b).magnitude;
    if (largestMagnitude < m) {
      largestMagnitude = m;
    }
  }
}

console.log('Task 2:', largestMagnitude);
