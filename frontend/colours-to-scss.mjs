import coloursJson from "./colours.json" with { type: "json" };
import { openSync, writeSync } from "node:fs";

const colours = coloursJson.colors;

let css = "";

for (const colourName in colours) {
  for (const colourStrength in colours[colourName]) {
    const cssLine = `$${colourName}${colourStrength}: ${colours[colourName][colourStrength]};\n`;
    css += cssLine;
  }
}

const cssFile = openSync("./assets/styles/variables/colours.scss", "w");
writeSync(cssFile, css);
