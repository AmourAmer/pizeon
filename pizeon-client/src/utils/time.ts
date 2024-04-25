import { Ref, computed } from "vue";
export default function (second: Ref<number>) {
  return {
    month: computed(() => new Date(second.value).getMonth() + 1),
    day: computed(() => new Date(second.value).getDate()),
  };
}
