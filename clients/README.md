# Clients

Every client talks to the backend only over HTTP, through
[`@sis/api-client`](shared/api-client). Install dependencies once from the
repo root with `npm install`.

| Client | Stack | Start (from repo root) |
| --- | --- | --- |
| [web](web) | React + Vite | `npm run dev:web` |
| [desktop](desktop) | Tauri v2 (React UI, Rust shell in `src-tauri/`) | `npm run dev:desktop` |
| [mobile](mobile) | Expo / React Native (iOS and Android) | `npm run dev:mobile` |
| [mcp](mcp) | Model Context Protocol server (stdio) | `npm run dev:mcp` |

Conventions:

- `src/App.tsx` is the app shell (layout and navigation). Keep domain screens out of it.
- Domain screens go in `src/features/<domain>/`, matching the backend domain names.
- Add new API calls to `@sis/api-client` first, then use them from the client.

Platform notes:

- **Desktop** needs the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.
  `src-tauri/` has its own Cargo workspace so backend builds stay fast.
- **Mobile** runs in Expo Go on a phone or in a simulator. On a physical device
  or Android emulator, set `EXPO_PUBLIC_SIS_API_URL` to an address the device can reach.
