import { mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises'

const root = new URL('../frontend/', import.meta.url)
const pages = JSON.parse(
  await readFile(new URL('src/features/navigation/aio-pages.json', root), 'utf8')
)

const pagesRoot = new URL('pages/', root)
await mkdir(pagesRoot, { recursive: true })
for (const entry of await readdir(pagesRoot, { withFileTypes: true })) {
  if (entry.isFile() && entry.name.endsWith('.html')) {
    await rm(new URL(entry.name, pagesRoot))
  }
}
for (const page of pages) {
  const html = `<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="color-scheme" content="light dark" />
    <base href="/" />
    <meta name="aio-page-id" content="${page.id}" />
    <meta name="aio-route" content="${page.route}" />
    <link rel="icon" href="/logo.png" />
    <title>${page.label}</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
`
  await writeFile(new URL(`pages/${page.id}.html`, root), html)
}
