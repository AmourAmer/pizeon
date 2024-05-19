import { Ref } from "vue";
import { useStorage } from "@vueuse/core";
import { stringMap } from "./type";

export function useData(id: string) {
  const data: Ref<stringMap[]> = useStorage(id, []);

  // FUCK U IDIOT STUPID BLIND DAMN WEAK ASSHOLE ABSURD RIDICULOUS MISERABLE SHAMEFUL ts
  let nonDeletedIter: (data: stringMap[]) => IterableIterator<stringMap> = (
    data: stringMap[],
  ) => {
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
    }[Symbol.iterator]() as IterableIterator<stringMap>;
  };

  return { data, nonDeletedIter };
}
