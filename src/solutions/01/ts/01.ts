import path from "path";
import { readInputFile } from "../../../lib/input";
import { range } from "../../../lib/loop";

async function run() {
  const input = await readInputFile(path.join(__dirname, "../data.txt"));
  const data = input.split("\n").map(Number);

  range(data.length).forEach((x) => {
    range(x + 1, data.length).forEach((y) => {
      if (data[x] + data[y] === 2020) {
        console.log(`Part 1: ${data[x] * data[y]}`);
      }

      range(y + 1, data.length).forEach((z) => {
        if (data[x] + data[y] + data[z] === 2020) {
          console.log(`Part 2: ${data[x] * data[y] * data[z]}`);
        }
      });
    });
  });
}

run();
