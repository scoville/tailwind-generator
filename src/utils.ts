import fs from "fs";
import postcss from "postcss";

import langs from "./langs";
import { Class, Lang } from "./types";

export const camelCase = (name: string) => {
  const [head = "", tail = ""] = [
    name.substr(0, 1),
    name.substr(1, name.length - 1),
  ];

  return `${head.toLowerCase()}${tail.replace(/[-_\s]+([a-z0-9])/g, (_, word) =>
    word.toUpperCase(),
  )}`;
};

export const isValidLang = (str: string): str is Lang =>
  langs.includes(str as any);

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

const classNameSimpleRegExp = /\.-?[_a-zA-Z]+[\:\\_a-zA-Z0-9-]*/;

const pseudoClasses = `(
  \\:after
  |\\:before
  |\\:focus
  |\\:hover
  |\\:active
  |\\:disabled
  |\\:visited
  |\\:first-child
  |\\:last-child
  |\\:\\:placeholder
  |\\:\\:\\-ms\\-input\\-placeholder
  |\\:\\-ms\\-input\\-placeholder
  |\\:\\:\\-moz\\-placeholder
  |\\:\\:\\-webkit\\-input\\-placeholder)$
`;

const pseudoClassesRegExp = new RegExp(
  pseudoClasses
    .split("\n")
    .map((s) => s.trim())
    .join("")
    .trim(),
);

const removePseudoClasses = (className: string): string => {
  while (pseudoClassesRegExp.test(className)) {
    className = className.replace(pseudoClassesRegExp, "");
  }

  return className;
};

const extractClassNameFromSelector = (selector: string) => {
  const matches = selector.match(classNameSimpleRegExp);

  if (!matches) {
    return;
  }

  const className = matches[0];

  if (!className) {
    return;
  }

  return removePseudoClasses(
    className
      .trim()
      .replace(/^\./, "")
      .replace(/\\\//g, "/")
      .replace(/\\/g, ""),
  );
};

export const readClasses = (filepath: string) => {
  const root = postcss.parse(fs.readFileSync(filepath, "utf8"));

  const classes: Class[] = [];

  root.walkRules(({ selector }) => {
    // Ignore anything that's not a class
    let className = extractClassNameFromSelector(selector);

    // Skip if it already exists, or if it's not a class selector
    if (!className || classes.some((c) => c.className === className)) {
      return;
    }

    const name = camelCase(
      className
        .replace(/\\\//g, "Over")
        .replace(/\\/g, "")
        .replace(/^\-/, "neg-")
        .replace(/\:\-/, "-neg-")
        .replace(/:/g, "-"),
    );

    classes.push({
      className,
      name,
    });
  });

  return classes;
};
