# Dioxus demo

This demo shows how to use [Dioxus](https://github.com/DioxusLabs/dioxus) to build a static web application and deploy it to Vercel.

## Local development

To run the demo locally, you would need the Rust toolchain and `trunk` installed.

Then you can simply run:

```bash
trunk serve
```

And you can visit the demo at `http://localhost:8080` (or aother port if you have specified some using `trunk serve --port <port>`).

## Build for production

To build for production, you can simply run:

```bash
trunk build --release
```

The default output path is `./dist`.

## Deploy to Vercel

To deploy the application to vercel, you would need to first run `vercel` locally to get the `PROJECT_ID` and `ORG_ID` from the generated `./vercel/project.json` file.

Then you need to set up the following secrets in your GitHub repo:

- `PROJECT_ID`
- `ORG_ID`
- `VERCEL_TOKEN`

You can modify `vercel.json` to specify your alias.

Then you can deploy the application to Vercel via your GitHub workflow.
