const input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]

function getMedian(arr) {
    return arr.slice().sort((a, b) => a - b)
    [Math.floor(arr.length / 2)];
};

function crabFuel(start, end) {
    let crabFuel = 0
    for (let i = 0; i <= Math.abs(start - end); i++)
        crabFuel += i
    return crabFuel
}

function totalFuel(input, position) {
    let totalFuel = 0
    input.map(e => totalFuel += crabFuel(e, position))
    return totalFuel
}

function getLowestFuel(position, oldResult) {
    let result = totalFuel(input, position)
    if (!oldResult || result < oldResult)
        return getLowestFuel(position + 1, result)
    else return oldResult
}

console.log("Task 2:", getLowestFuel(getMedian(input)))
