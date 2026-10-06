# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app. It also defines the routes for the app.
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page
│  │  ├─ echo.rs # The echo component uses server functions to communicate with the server
│  ├─ views/ # The views each route will render in the app.
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Styling and Development

Tailwind CSS v4, DaisyUI, and `tw-animate-css` are managed with Bun. Install the dependencies once:

```bash
bun install
```

Run the CSS watcher and Dioxus development server together in one terminal:

```bash
bun run dev
```

The development command builds the stylesheet once, then watches `assets/tailwind.input.css` and Rust source files for Tailwind classes while `dx serve` runs. CSS changes update the asset without recompiling the Rust app. To run only the Tailwind watcher, use `bun run css:watch`; to build the stylesheet once, use `bun run css:build`.

The generated stylesheet is `assets/tailwind.css`, which is linked by the app. Add DaisyUI components with their standard classes, such as `btn btn-primary`.

