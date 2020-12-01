import path from "path";
import { readInputFile } from "../../../lib/input";

async function part01() {
  const input = await readInputFile(path.join(__dirname, "../data.txt"));
  const data = input.split("\n").map(Number);

  for (let x = 0; x < data.length; x++) {
    for (let y = x + 1; y < data.length; y++) {
      if (data[x] + data[y] === 2020) {
        console.log(`Part 1: ${data[x] * data[y]}`);
      }
    }
  }
}

async function part02() {
  const input = await readInputFile(path.join(__dirname, "../data.txt"));
  const data = input.split("\n").map(Number);

  for (let x = 0; x < data.length; x++) {
    for (let y = x + 1; y < data.length; y++) {
      for (let z = y + 1; z < data.length; z++) {
        if (data[x] + data[y] + data[z] === 2020) {
          console.log(`Part 2: ${data[x] * data[y] * data[z]}`);
        }
      }
    }
  }
}

part01();
part02();
