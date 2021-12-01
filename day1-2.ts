import fs from 'fs';

const depths = fs
  .readFileSync('input', 'utf8')
  .split('\n')
  .map((a) => Number.parseInt(a, 10));

let previousDepth: number | undefined;
let increasedCount: number = 0;

function* slidingWindow(values: number[]) {
  const elements = [] as number[];
  for (let i = 0; i < values.length; i += 1) {
    elements.push(values[i]);
    if (elements.length > 3) {
      elements.shift();
    }
    if (elements.length === 3) {
      yield elements;
    }
  }
}

// eslint-disable-next-line no-restricted-syntax
for (const [a, b, c] of slidingWindow(depths)) {
  const depth = a + b + c;

  if (previousDepth !== undefined && previousDepth < depth) {
    increasedCount += 1;
  }

  previousDepth = depth;
}

console.log(increasedCount);
