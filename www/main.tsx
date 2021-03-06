/** @jsx h */
import { h } from "https://deno.land/x/sift@0.4.2/mod.ts";
import { serve } from "https://deno.land/x/sift@0.4.2/mod.ts";

serve({
  "/": () => (
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <title>Captain Hook 🪝</title>
        <meta
          name="description"
          content="Cross-platform, modern and native git hooks."
        />

        <link
          rel="stylesheet"
          href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/4.0.0/github-markdown.min.css"
          integrity="sha512-Oy18vBnbSJkXTndr2n6lDMO5NN31UljR8e/ICzVPrGpSud4Gkckb8yUpqhKuUNoE+o9gAb4O/rAxxw1ojyUVzg=="
          crossorigin="anonymous"
          referrerpolicy="no-referrer"
        />
        <style>
          {`
            .markdown-body {
                box-sizing: border-box;
                min-width: 200px;
                max-width: 980px;
                margin: 0 auto;
                padding: 45px;
            }
        
            @media (max-width: 767px) {
                .markdown-body {
                    padding: 15px;
                }
            }
        `}
        </style>
      </head>
      <body>
        <main class="markdown-body">
          <div align="center">
            <h1>Captain Hook 🪝</h1>
            <p>
              <b>
                Cross-platform, modern and native git hooks.
              </b>
            </p>
            <sub>
              Inspired by
              <a
                href="https://github.com/typicode/husky"
                rel="noreferrer"
                target="_blank"
              >
                husky
              </a>.
            </sub>
          </div>
          <div>
            <h2>Prerequisites</h2>
            <p>
              To run <code>captain-hook</code> you need to have{" "}
              <a href="https://git-scm.com/" rel="noreferrer" title="git">
                git
              </a>{" "}
              installed on your machine.
            </p>
            <h2>Quick install</h2>
            <p>
              It is possible to install <code>captain-hook</code>{" "}
              in two flavours:
            </p>
            <ul>
              <li>
                With Shell:
                <pre lang="sh">
                  <code>
                    sh -c "$(curl -fsSL https://captain-hook.sh/install)"
                  </code>
                </pre>
                To update the Captain Hook itself, rerun the above script. It
                will replace the current version with the latest one.
              </li>
              <li>
                With Cargo:
                <pre lang="sh">
                  <code>cargo install captain-hook</code>
                </pre>
                To update the Captain Hook with Cargo, remember to force
                re-installing the binary.
                <pre lang="sh">
                  <code>cargo install --force captain-hook</code>
                </pre>
              </li>
            </ul>
          </div>
        </main>
      </body>
    </html>
  ),
  "/install": async () => {
    const __dirname = new URL('.', import.meta.url).pathname;
    const file = await Deno.readFile(`${__dirname}install`);
    return new Response(file, {
      headers: {
        "content-type": "text/text",
      },
    });
  },
});
