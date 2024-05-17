export interface Notice {
  date: number;
  heading: string;
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
