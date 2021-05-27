const concurrently = require("concurrently");
const esbuild = require("esbuild");
const fs = require("fs");
const path = require("path");

process.stdout.write("Transpile source code...");

esbuild.buildSync({
  entryPoints: [path.resolve(__dirname, "generate-networks.ts")],
  outfile: path.resolve(__dirname, "generate-networks.js"),
  bundle: true,
  platform: "node",
  external: ["@tensorflow/tfjs-node"],
});

esbuild.buildSync({
  entryPoints: [path.resolve(__dirname, "play.ts")],
  outfile: path.resolve(__dirname, "play.js"),
  bundle: true,
  platform: "node",
  external: ["@tensorflow/tfjs-node"],
});

console.log("done");

process.stdout.write("Finding current generation...");

const generations = fs
  .readdirSync(__dirname)
  .filter((dir) => dir.startsWith("generation"))
  .map((dir) => parseInt(dir.replace("generation", "")));

let generation = Math.max(0, ...generations);

if (generation === 0) {
  console.log("no generation yet");
  generation++;
} else {
  console.log(`${generation}`);
}

async function main() {
  while (true) {
    console.log("");

    const dir = `generation${generation}`;

    if (!fs.readdirSync(__dirname).includes(dir)) {
      await concurrently([
        `node ${__dirname}/generate-networks.js ${generation}`,
      ]);
    }

    console.log(`Self-playing generation ${generation}`);

    let hasError = true;
    while (hasError) {
      hasError = false;
      try {
        await concurrently([
          `node ${__dirname}/play.js ${generation} 0`,
          `node ${__dirname}/play.js ${generation} 1`,
          `node ${__dirname}/play.js ${generation} 2`,
          `node ${__dirname}/play.js ${generation} 3`,
        ]);
      } catch {
        hasError = true;
      }
    }

    generation += 1;
  }
}

main();
