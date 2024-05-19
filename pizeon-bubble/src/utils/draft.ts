import { Ref } from "vue";
import { useStorage } from "@vueuse/core";
import { stringMap } from "./type";

export function useData(id: string) {
  const data: Ref<stringMap[]> = useStorage(id, []);

  let iter = (data: stringMap[]) => {
    return {
      current: 0,
      [Symbol.iterator]() {
        return this;
      },
      next() {
        while (this.current < data.length) {
          this.current++;
          if (!data[this.current - 1].deleted)
            return {
              value: data[this.current - 1],
              done: false,
            };
        }
        return { done: true };
      },
    }[Symbol.iterator];
  };
  return { data, iter };
}
