export interface Notice {
  date: number;
  title: string;
  bare_body: string;
}

export enum Repo {
  Fresh = "Fresh",
  Blocked = "Blocked",
  Fridge = "Fridge",
  Junk = "Junk",
}

export interface stringMap {
  [key: string]: any;
}
