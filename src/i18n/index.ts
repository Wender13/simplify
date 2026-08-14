// i18n/index.ts
import { nextTick } from "vue";
import { createI18n, type I18n, type Composer } from "vue-i18n";

export const SUPPORT_LOCALES = ["pt-BR", "en-US"] as const;
export type SupportedLocale = (typeof SUPPORT_LOCALES)[number];

export function setupI18n(options = { locale: "pt-BR" as SupportedLocale }) {
  const i18n = createI18n({
    legacy: false,
    locale: options.locale,
    fallbackLocale: "en-US",
    messages: {},
  });
  setI18nLanguage(i18n, options.locale);

  globalThis.t = i18n.global.t;

  return i18n;
}

export function setI18nLanguage(i18n: I18n, locale: SupportedLocale) {
  if (i18n.mode === "legacy") {
    i18n.global.locale = locale;
  } else {
    (i18n.global.locale as Composer["locale"]).value = locale;
  }
  document.querySelector("html")?.setAttribute("lang", locale);
}

export async function loadLocaleMessages(i18n: I18n, locale: SupportedLocale) {
  const messages = await import(`./locales/${locale}.json`);
  i18n.global.setLocaleMessage(locale, messages.default);
  return nextTick();
}
