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

export interface Abstract {
  title: string;
  body?: string; // FIXME: seems notice.rs doesn't want Abstract to know body? Should send back the very first few letters.
  date: number;
  class?: string; // TODO: remove ? ? Or no need
}
