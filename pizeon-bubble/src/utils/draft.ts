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

  function uniqueDataType(datum: Ref<stringMap>) {
    return (type: string) => {
      for (let nonDeletedDatum of nonDeletedIter(data.value))
        if (nonDeletedDatum.type == type && nonDeletedDatum != datum.value) {
          datum.value["type_change_warning"] =
            "One event should have only one " +
            type +
            ", right? If things go beyond my expectation, please email me(Amour<pizeon@tuta.io>)";
          // BUG: yes, you can add multiple "type"s by delete then recover. 2 reasons not to prevent this, 1st is respect the choice of user
          return false;
        }
      return true;
    };
  }

  return { data, nonDeletedIter, uniqueDataType };
}
