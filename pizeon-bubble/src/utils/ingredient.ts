import ingredientTitle from "ingredient/ingredientTitle.vue";
import ingredientTime from "ingredient/ingredientTime.vue";
import ingredientHost from "ingredient/ingredientHost.vue";
import ingredientPlace from "ingredient/ingredientPlace.vue";
import ingredientLink from "ingredient/ingredientLink.vue";
import ingredientTickbox from "ingredient/ingredientTickbox.vue";
import ingredientText from "ingredient/ingredientText.vue";

export function useIngredientType(type: string) {
  switch (type) {
    case "title":
      return ingredientTitle;
    case "db":
      return null;
    case "host":
      return ingredientHost;
    case "time":
      return ingredientTime;
    case "place":
      return ingredientPlace;
    case "link":
      return ingredientLink;
    case "tickbox":
      return ingredientTickbox;
    case "text":
      return ingredientText;
    default:
      return ingredientText;
  }
}
