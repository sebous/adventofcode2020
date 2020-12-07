import { join } from "path";
import { readInputFile } from "../../../lib/input";

type BagMap = Map<string, { [key: string]: number }>;
const map: BagMap = new Map();
const SHINY_GOLD = "shiny gold";

function parseInput(input: string) {
  input.split("\n").forEach((line) => {
    const [parent, children] = line.split("contain");
    const parentKey = parent.replace("bags", "").trim();
    const childrenPairs = children
      .replace(".", "")
      .split(",")
      .map((child) => {
        if (child.includes("no other bags")) {
          return {};
        }
        const parts = child.trim().split(" ");
        return {
          [`${parts[1]} ${parts[2]}`]: Number(parts[0]),
        };
      });
    map.set(
      parentKey,
      childrenPairs.reduce((acc, pair) => Object.assign(acc, pair), {})
    );
  });
}

function doesColorContainsGold(color: string) {
  const relationship = map.get(color);
  if (!relationship) throw Error("color not found");

  if (relationship[SHINY_GOLD]) return true;
  let goldFound = false;
  Object.keys(relationship).forEach((c) => {
    if (doesColorContainsGold(c)) {
      goldFound = true;
    }
  });
  return goldFound;
}

function part1() {
  let counter = 0;
  map.forEach((val, color) => {
    if (doesColorContainsGold(color)) counter += 1;
  });
  console.log(`Part 1: ${counter}`);
}

function countIncludedBags(color: string) {
  let bagsCount = 0;
  const relationship = map.get(color);
  if (!relationship) throw Error("color not found");

  Object.entries(relationship).forEach(([c, count]) => {
    bagsCount += count + count * countIncludedBags(c);
  });
  return bagsCount;
}

async function run() {
  const input = await readInputFile(join(__dirname, "../data.txt"));
  parseInput(input);
  part1();
  console.log(`Part 2: ${countIncludedBags(SHINY_GOLD)}`);
}

run();
