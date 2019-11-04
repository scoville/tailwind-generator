import langs from "./langs";

export interface Class {
  className: string;
  name: string;
}

export interface Adapter {
  save(dir: string, classes: Class[]): void;
}

export type Lang = typeof langs[number];
