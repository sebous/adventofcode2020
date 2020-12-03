import { join } from "path";
import { cloneDeep } from "lodash";
import { readInputFile } from "../../../lib/input";

interface Coord {
  x: number;
  y: number;
}

const coordToKey = ({ x, y }: Coord) => `${x},${y}`;
const keyToCoord = (k: string): Coord => {
  const [x, y] = k.split(",").map(Number);
  return { x, y };
};

const TARGET_X = 3;
const TARGET_Y = 1;
let gridWidth = 0;
let gridHeight = 0;
const sourceGrid: { [key: string]: boolean } = {};

let currentGrid: { [key: string]: boolean } = {};
let currentGridWidth = 0;
let currentPos: Coord = { x: 0, y: 0 };

let treesEncountered = 0;

function calculateNext(): boolean | undefined {
  const nextPos: Coord = { x: currentPos.x + TARGET_X, y: currentPos.y + TARGET_Y };
  if (nextPos.y + 1 > gridHeight) return;
  if (nextPos.x + 1 > currentGridWidth) {
    // copy grid right
    Object.entries(sourceGrid).forEach(([pos, val]) => {
      const coordPos = keyToCoord(pos);
      coordPos.x = coordPos.x + currentGridWidth;
      currentGrid[coordToKey(coordPos)] = val;
    });
    currentGridWidth += gridWidth;
  }
  // console.log(nextPos, currentGrid[coordToKey(nextPos)]);
  if (currentGrid[coordToKey(nextPos)]) {
    treesEncountered += 1;
  }
  currentPos = nextPos;
  return true;
}

async function run() {
  const input = await readInputFile(join(__dirname, "../data.txt"));

  input.split("\n").forEach((line, y) => {
    if (y + 1 > gridHeight) {
      gridHeight = y + 1;
    }
    line.split("").forEach((spot, x) => {
      if (x + 1 > gridWidth) {
        gridWidth = x + 1;
      }
      sourceGrid[coordToKey({ x, y })] = spot === "#";
    });
  });

  currentGrid = cloneDeep(sourceGrid);
  currentGridWidth = gridWidth;

  while (calculateNext()) {}
  console.log(treesEncountered);
}

run();
