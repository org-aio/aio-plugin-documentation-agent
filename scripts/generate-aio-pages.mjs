import { mkdir, readFile, readdir, rm, writeFile } from "node:fs/promises";

const root = new URL("../frontend/", import.meta.url);
const pages = JSON.parse(
  await readFile(
    new URL("src/features/navigation/aio-pages.json", root),
    "utf8",
  ),
);
const indexHtml = await readFile(new URL("index.html", root), "utf8");

const escapeHtml = (value) =>
  String(value).replace(
    /[&<>"']/g,
    (character) =>
      ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[
        character
      ],
  );

const pagesRoot = new URL("pages/", root);
await mkdir(pagesRoot, { recursive: true });
for (const entry of await readdir(pagesRoot, { withFileTypes: true })) {
  if (entry.isFile() && entry.name.endsWith(".html")) {
    await rm(new URL(entry.name, pagesRoot));
  }
}
for (const page of pages) {
  const html = indexHtml
    .replace(
      '<meta name="color-scheme" content="light dark" />',
      `<meta name="color-scheme" content="light dark" />\n    <meta name="aio-page-id" content="${escapeHtml(page.id)}" />\n    <meta name="aio-route" content="${escapeHtml(page.route)}" />`,
    )
    .replace(
      '<link rel="icon" href="./logo.svg" />',
      '<link rel="icon" href="../logo.svg" />',
    )
    .replace("<title></title>", `<title>${escapeHtml(page.label)}</title>`);
  await writeFile(new URL(`pages/${page.id}.html`, root), html);
}
