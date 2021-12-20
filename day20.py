import numpy as np
import scipy.ndimage as ndimage

with open("day20-input.txt") as f:
    windowingMap, _, *inputImage = [line.strip() for line in f.readlines()]
    windowingMap = list(windowingMap.replace('#', '1').replace('.', '0'))
    inputImage = [list(line.replace('#', '1').replace('.', '0')) for line in inputImage]

    windowingMap = np.array(list(windowingMap), dtype=int)
    inputImage = np.array(inputImage, dtype=int)


def enhanceImage(x):
    return windowingMap[int(''.join([str(num) for num in x.astype(int)]), 2)]


def runEnhancer(times):
    result = np.pad(inputImage, pad_width=times*2+1)
    i = 0
    while i < times:
        i += 1
        result = ndimage.generic_filter(result, enhanceImage, size=(3, 3), cval=0)
    return np.count_nonzero(result)


print('Task 1: lit up pixels', runEnhancer(2))
print('Task 2: lit up pixels', runEnhancer(50))
