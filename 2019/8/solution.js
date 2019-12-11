const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
//const testFile = new TestFile(__dirname + '/test.txt');
//const testFile2 = new TestFile(__dirname + '/test2.txt');
//const { IntCodeMachine } = require('../IntCodeMachine');


function parseImage(pixelData, width = 25, height = 6) {
    const image = []
    for (let l = 0; l * width * height < pixelData.length; l++) {
        const layer = [];
        for (let h = 0; h < height; h++) {
            const horizontal = [];
            for (let w = 0; w < width; w++) {
                horizontal.push(pixelData[l * width * height + h * width + w]);
            }
            layer.push(horizontal);
        }
        image.push(layer);
    }
    return image;
}

function countNumber(layer, num) {
    let count = 0;
    for (const horizontal of layer) {
        for (const px of horizontal) {
            count += px === num ? 1 : 0;
        }
    }
    return count;
}

const rawPixelData = input.linesAsStrings()[0]
    .split('')
    .map(x => parseInt(x));
const layers = parseImage(rawPixelData)
    .sort((a, b) => countNumber(a, 0) - countNumber(b, 0));
const result1 = countNumber(layers[0], 1) * countNumber(layers[0], 2);

console.log(`Result 1: ${result1}`);

// part 2
function flattenLayers(layers) { return layers.map(horizontal => horizontal.join('')).join('\n'); }

function renderLayers(layers, width = 25, height = 6) {
    const finalLayer = [];
    for (let h = 0; h < height; h++) {
        finalLayer[h] = [];
        for (let w = 0; w < width; w++) {
            finalLayer[h][w] = 2;
        }
    }

    for (const layer of layers) {
        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                if (finalLayer[y][x] === 2) {
                    finalLayer[y][x] = layer[y][x];
                }
            }
        }
    }
    return finalLayer;
}

const result2 = flattenLayers(renderLayers(parseImage(rawPixelData)));
console.log(`Result2:\n${result2}`);

module.exports = { result1, result2 };