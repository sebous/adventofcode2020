import { promises as fs } from "fs";

export async function readInputFile(path: string): Promise<string> {
  try {
    const data = await fs.readFile(path, { encoding: "utf8" });
    return data;
  } catch (err) {
    throw Error(err);
  }
}
