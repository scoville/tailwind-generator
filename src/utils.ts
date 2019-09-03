import fs from "fs";
import postcss from "postcss";

import { Class } from "./types";

export const camelCase = (name: string) =>
  name.replace(/[-_\s]+([a-z0-9])/g, (_, word) => word.toUpperCase());

export const isValidLang = (
  str: string,
): str is "elm" | "reasonml" | "typescript" =>
  ["elm", "reasonml", "typescript"].includes(str);

export const shutDownLog = async <T>(f: () => Promise<T>): Promise<T> => {
  // tslint:disable-next-line: no-console
  const oldConsoleLog = console.log;

  // tslint:disable-next-line: no-console
  console.log = () => undefined;

  const res = await f();

  // tslint:disable-next-line: no-console
  console.log = oldConsoleLog;

  return res;
};

export const readClasses = (filepath: string) => {
  const root = postcss.parse(fs.readFileSync(filepath, "utf8"));

  const classes: Class[] = [];

  root.walkRules(({ selector: className }) => {
    // Ignore anything that's not a class
    if (!className.startsWith(".")) {
      return;
    }

    // Ignore responsive variations
    if (
      className.startsWith(".sm\\:") ||
      className.startsWith(".md\\:") ||
      className.startsWith(".lg\\:") ||
      className.startsWith(".xl\\:")
    ) {
      return;
    }

    // Ignore pseudo selectors
    if (
      className.indexOf(":after") !== -1 ||
      className.indexOf(":before") !== -1
    ) {
      return;
    }

    let name = className
      .replace(/^\S*\./, "")
      .replace(":focus", "")
      .replace(":hover", "")
      .replace(/\\\//g, "Over")
      .replace(/\\/g, "")
      .split(":")
      .join("-");

    // replace -* to *-neg
    if (name.startsWith("-")) {
      name = name.replace(/^-/, "") + "-neg";
    }

    name = camelCase(name);

    className = className
      .replace(/^\S*\./, "")
      .replace(":focus", "")
      .replace(":hover", "")
      .replace(/\\\//g, "/")
      .replace(/\\/g, "");

    // Skip if it already exists
    if (classes.some(c => c.name === name)) {
      return;
    }

    classes.push({
      className,
      name,
    });
  });

  return classes;
};
