import pluginVue from "eslint-plugin-vue";
import { withVueTs, vueTsConfigs } from "@vue/eslint-config-typescript";
import prettier from "eslint-config-prettier";

export default withVueTs(
  pluginVue.configs["flat/recommended"],
  vueTsConfigs.recommended,
  prettier,
  {
    files: ["**/*.vue", "**/*.ts"],
    rules: {
      // suas customizações aqui
    },
  },
  {
    ignores: ["dist/**", "src-tauri/target/**", "node_modules/**"],
  },
);
