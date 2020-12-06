import { join } from "path";
import { readInputFile } from "../../../lib/input";

function part2(lines: string[]) {
  const groups: string[][] = [];
  let currentGroupIndex = 0;

  for (let line of lines) {
    if (line === "") {
      currentGroupIndex += 1;
      continue;
    }
    if (!groups[currentGroupIndex]) groups.push([]);
    groups[currentGroupIndex].push(line);
  }

  const groupsSplitByPerson = groups.map(g => g.map(sub => sub.split("")));
  const groupsIntersections = groupsSplitByPerson.map(group => group.reduce((inter, person) => inter.filter(e => person.includes(e))));
  const totalIntersecitonCharCount = groupsIntersections.reduce((total, group) => total + group.length, 0);
  console.log(`Part 2: ${totalIntersecitonCharCount}`);
}

async function run() {
  const input = await readInputFile(join(__dirname, "../data.txt"));
  const lines = input.split("\n");

  const groups: string[][] = []
  let currentGroupIndex = 0;
  for (let line of lines) {
    if (line === "") {
      currentGroupIndex += 1;
      continue;
    }
    if (!groups[currentGroupIndex]) groups.push([]);
    line.split("").forEach(ch => groups[currentGroupIndex].push(ch));
  }

  const groupsUnique = groups.map(g => [...new Set(g)]);
  const totalCharCount = groupsUnique.reduce((total, group) => total + group.length, 0);
  console.log(`Part 1: ${totalCharCount}`);

 part2(lines); 
}
run();