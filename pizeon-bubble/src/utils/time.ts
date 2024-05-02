import { Ref, computed } from "vue";
interface Time {
  month: Ref<number>;
  day: Ref<number>;
}
export default function (second: number): Time {
  let millisecond = computed(() => second * 1000);
  return {
    month: computed(() => new Date(millisecond.value).getMonth() + 1),
    day: computed(() => new Date(millisecond.value).getDate()),
  };
}
