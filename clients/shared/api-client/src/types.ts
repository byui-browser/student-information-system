// TypeScript mirrors of the backend's public types.
// When a backend type changes in Rust, update the matching type here.

export interface Health {
  service: string;
  status: string;
  version: string;
}

export interface ApiErrorBody {
  error: { code: string; message: string };
}
