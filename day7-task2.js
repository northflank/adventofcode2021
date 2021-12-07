const input = [1101, 1, 29, ...]

function getMedian(arr) {
    return arr.slice().sort((a, b) => a - b)[Math.floor(arr.length / 2)];
};

function crabCost(top, bottom) {
    let crabCost = 0
    for (let i = 0; i <= Math.abs(top - bottom); i++) crabCost += i
    return crabCost
}

function fuelCost(input, position) {
    let fuelCost = 0
    input.map(e => fuelCost += crabCost(e, position))
    return fuelCost
}

function getLowestResult(position, oldResult) {
    let result = fuelCost(input, position)
    if (!oldResult || result < oldResult) return getLowestResult(position + 1, result)
    else return oldResult
}

console.log("Task 2:", getLowestResult(getMedian(input)))
