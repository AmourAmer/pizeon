import IngredientTitle from "ingredient/IngredientTitle.vue";
import IngredientTime from "ingredient/IngredientTime.vue";
import IngredientHost from "ingredient/IngredientHost.vue";
import IngredientPlace from "ingredient/IngredientPlace.vue";
import IngredientLink from "ingredient/IngredientLink.vue";
import IngredientTickbox from "ingredient/IngredientTickbox.vue";
import IngredientText from "ingredient/IngredientText.vue";

export function useIngredientType(type: string) {
  switch (type) {
    case "title":
      return IngredientTitle;
    case "db":
      return null;
    case "host":
      return IngredientHost;
    case "time":
      return IngredientTime;
    case "place":
      return IngredientPlace;
    case "link":
      return IngredientLink;
    case "tickbox":
      return IngredientTickbox;
    case "text":
      return IngredientText;
    default:
      return IngredientText;
  }
}
