# Kolibra

Kolibra uses Dioxus Fullstack. The server handles server functions and login sessions, while the web, desktop, and mobile apps act as clients that send requests to it.

## Local development

Install the stylesheet dependencies before running the app:

```bash
bun install
```

Run the web app with a local Fullstack server and CSS watch:

```bash
./scripts/dev.sh
```

The script builds CSS, starts its watcher, and launches `dx serve` directly so Bun's `.env` interpolation does not alter the server's environment.

## Deploy the Fullstack server

The server requires two environment variables:

- `APP_PASSWORD_HASH`: the Argon2 hash for the login password, not the plaintext password.
- `SESSION_SECRET`: a random secret used to sign session cookies.

Generate these values locally with the provided helpers:

```bash
cargo run --bin hash_password
cargo run --bin generate_secret
```

Set the hash and secret as environment variables on your deployment host (for example, Railway). Do not put the plaintext password or `SESSION_SECRET` in source code, client images, or the repository. For local development, you can store them in `.env`, which is already ignored by Git.

The repository's `Dockerfile` builds a Fullstack web bundle and runs the server binary on `0.0.0.0:8080`. Use this Docker image to deploy the server:

```bash
docker build -t kolibra .
docker run --rm -p 8080:8080 \
  -e APP_PASSWORD_HASH='<hash-argon2>' \
  -e SESSION_SECRET='<secret-acak>' \
  kolibra
```

On Railway, deploy from this repository using the `Dockerfile`, then add both variables under **Variables**. The platform must pass `PORT` and `IP=0.0.0.0` to the server process; the Dockerfile sets defaults of `8080` and `0.0.0.0`. After deployment, test the status endpoint, for example:

```bash
curl -i https://your-domain.example/api/auth/status
```

The server at your web domain can also serve the web client. Build the Fullstack web bundle with:

```bash
dx bundle --web --release
```

## Connect clients to the server

The server-function base URL is configured in `src/main.rs`, inside `main()`:

```rust
#[cfg(not(feature = "server"))]
set_server_url("https://kolibra-production.up.railway.app");
```

Replace this URL with the origin of your deployed server. **Do not add a trailing slash (`/`)**: Dioxus appends the server-function path directly, such as `/api/auth/login`. For example, use `https://example.up.railway.app`, not `https://example.up.railway.app/`.

This setting applies to web, desktop, and mobile clients. Set it once at startup, before calling the first server function; the URL cannot be changed after the app starts. The `cfg(not(feature = "server"))` attribute prevents the client URL from being set in the server binary.

Whenever you change the server address, rebuild the client you plan to distribute. The endpoint must be reachable from users' devices over HTTPS. Do not include server credentials in client builds.

## Build the desktop client

Build a Linux AppImage:

```bash
dx bundle --desktop --package-types appimage --release
```

For other distribution formats, use the appropriate package type, such as `deb` or `rpm` on Linux, `msi` on Windows, or `dmg` on macOS. In general, build desktop apps on the target OS. Desktop users still need a running Fullstack server; its URL comes from `set_server_url` above.

## Build the mobile client

Android APK:

```bash
dx bundle --android --package-types apk --release
```

Android App Bundle for store distribution:

```bash
dx bundle --android --package-types aab --release
```

iOS archive:

```bash
dx bundle --ios --package-types ipa --release
```

Mobile builds require the relevant platform toolchain: Android requires the Android SDK/NDK, while iOS requires macOS and Xcode. Set the server URL before building. The mobile app uses the same Fullstack server and does not need a copy of the server secrets.

## Styling

Tailwind CSS v4 and DaisyUI are managed with Bun. The stylesheet source is `assets/tailwind.input.css`, and the app uses the generated `assets/tailwind.css`. Run `bun run css:build` after editing the stylesheet.
