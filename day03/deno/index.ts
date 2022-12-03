import Lodash from "npm:@types/lodash";
import lodash from "npm:lodash";

const _ = lodash as typeof Lodash;

const text = (await Deno.readTextFile("../input")).trim();

function sum(arr: number[]) {
  return arr.reduce((a, b) => a + b, 0);
}

const priorityMap = "abcdefghijklmonpqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
  .split("")
  .reduce((acc, cur, i) => {
    acc[cur] = i + 1;
    return acc;
  }, {} as Record<string, number>);

// console.log(Object.keys(priorityMap).length)
// console.log(priorityMap["a"]);
// console.log(priorityMap)

const getPriority = (item: string) => {
  if (!(item in priorityMap)) {
    throw new Error(`No priority for ${item}`);
  }
  return priorityMap[item];
};

const findCommon = (a: string[], b: string[]) => {
  for (const item of a) {
    if (b.includes(item)) {
      return item;
    }
  }
  throw new Error("No common item found");
};

const getBagPriority = (bag: string) => {
  const items = bag.split("");
  const [compA, compB] = _.chunk(items, bag.length / 2);

  const lengths = [compA.length, compB.length];
  if (lengths[0] !== lengths[1]) {
    throw new Error(`Invalid bag: ${bag}`);
  }

  const common = findCommon(compA, compB);
  console.log(`Common: ${common}, priority: ${getPriority(common)}`);
  return getPriority(common);
};

const bags = text.split("\n");

const totalPriority = sum(bags.map(getBagPriority));

console.log(totalPriority);
