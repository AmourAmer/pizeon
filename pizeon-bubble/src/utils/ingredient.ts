import ingredientHost from "ingredient/ingredientHost.vue";
import ingredientText from "ingredient/ingredientText.vue";
import ingredientTitle from "ingredient/ingredientTitle.vue";
export function useIngredientType(type: string) {
  switch (type) {
    case "title":
      return ingredientTitle;
    case "host":
      return ingredientHost;
    default:
      return ingredientText;
  }
}
