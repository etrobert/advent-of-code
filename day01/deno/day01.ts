function sum(arr: number[]) {
  return arr.reduce((a, b) => a + b, 0);
}

const text = (await Deno.readTextFile("../input")).trim();

const elfStrings = text.split("\n\n");

const elfSnacks = elfStrings.map((elfString) =>
  elfString
    .split("\n")
    .map((snackString) => parseInt(snackString))
);

const elfSnacksSums = elfSnacks.map(sum);

const sortedElfSnacksSums = elfSnacksSums.sort((a, b) => b - a);

const biggestSnack = Math.max(...elfSnacksSums);

const threeBiggest = sortedElfSnacksSums.slice(0, 3);

console.log(`Biggest elf: ${biggestSnack}`)
console.log(`Biggest three elves: ${sum(threeBiggest)}`)
