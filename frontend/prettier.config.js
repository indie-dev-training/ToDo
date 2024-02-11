module.exports = {
  trailingComma: "none",
  tabWidth: 2,
  printWidth: 120,
  semi: true,
  // NOTE:
  // prettier-plugin-tailwindcssは最後に指定しないとソートが効かない。
  // https://github.com/tailwindlabs/prettier-plugin-tailwindcss#compatibility-with-other-prettier-plugins
  plugins: ["@trivago/prettier-plugin-sort-imports", "prettier-plugin-tailwindcss"],
  importOrderSeparation: false,
  importOrderSortSpecifiers: true,
  importOrder: [
    "^(react/(.*)$)|^(react$)",
    "^(next/(.*)$)|^(next$)",
    "<THIRD_PARTY_MODULES>",
    "^types$",
    "^@/common/lib/(.*)$",
    "^@/dev/lib/(.*)$",
    "^@/publish/lib/(.*)$",
    "^@/common/components/(.*)$",
    "^@/dev/components/(.*)$",
    "^@/publish/components/(.*)$",
    "^[./]"
  ]
};
