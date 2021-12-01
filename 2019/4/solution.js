//part 1
function isValid(num) {
    const digits = String(num).split('').map(x => parseInt(x));
    let hasDoubles = false;
    for (let i = 1; i < digits.length; i++) {
        if (digits[i - 1] > digits[i]) {
            return false;
        }
        if (digits[i - 1] === digits[i]) {
            hasDoubles = true;
        }
    }
    return hasDoubles;
}

const list = [];
for (let n = 382345; n < 843167; n++) {
    if (isValid(n)) {
        list.push(n);
    }
}

const result1 = list.length;
console.log(`Result 1: ${result1}`);

//part 2
function isValid2(num) {
    const digits = String(num).split('').map(x => parseInt(x));
    let hasDoubles = false;
    for (let i = 0; i < digits.length - 1; i++) {
        if (digits[i] > digits[i + 1]) {
            return false;
        }
        if (digits[i] === digits[i + 1]) {
            const start = i;
            while (i < digits.length && digits[i] === digits[start]) {
                i++;
            }
            if (i - start === 2) {
                hasDoubles = true;
            }
            i -= 2; //back up to make sure we continue checking across boundary
        }
    }
    return hasDoubles;
}

const list2 = [];
for (let n = 382345; n < 843167; n++) {
    if (isValid2(n)) {
        list2.push(n);
    }
}

const result2 = list2.length;
console.log(`Result 2: ${result2}`);

module.exports = { result1, result2 };
