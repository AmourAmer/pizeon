import ingredientTitle from "ingredient/ingredientTitle.vue";
import ingredientTime from "ingredient/ingredientTime.vue";
import ingredientHost from "ingredient/ingredientHost.vue";
import ingredientPlace from "ingredient/ingredientPlace.vue";
import ingredientText from "ingredient/ingredientText.vue";

export function useIngredientType(type: string) {
  switch (type) {
    case "title":
      return ingredientTitle;
    case "host":
      return ingredientHost;
    case "time":
      return ingredientTime;
    case "place":
      return ingredientPlace;
    default:
      return ingredientText;
  }
}
