# Data directory

Language-agnostic local data at the repository root. Nothing in it is read by application code.

## Compose service data (gitignored)

Docker Compose bind-mounts service state here under `data/*_data/`: `data/postgres_data/`, and `data/victorialogs_data/` and `data/victoriametrics_data/` when the opt-in `observability` profile is running. These folders are created by `docker compose up` and are excluded by the root `.gitignore`. Delete them to reset a local service.

## Local sample data (optional)

Put media you want to import while trying FileSquid or a distribution in a subfolder here (for example `data/samples/`) and point an import at it from the UI or the API. Use UUID v4 file names ([ADR 021](../resources/architecture/021-uuid-v4-identifiers.md)) when a sample set is meant to be shared, so no file name carries descriptive content. Whether a sample set is tracked or gitignored is decided per set in the root `.gitignore`.

Application hosts keep their own operational store (activity journal, admin token) under `filesquid-data/` in their working directory, not here; that folder is also gitignored.
