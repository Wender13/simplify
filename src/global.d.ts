import type { Composer } from "vue-i18n";

declare global {
  var t: Composer["t"];
}

export {};
