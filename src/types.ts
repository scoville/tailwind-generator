export interface Class {
  className: string;
  name: string;
}

export interface Adapter {
  save(dir: string, classes: Class[]): void;
}
