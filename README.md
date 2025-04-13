This repository shows a bug with eframe where the previous window remains (and cannot be closed) until the "run_native" method is called again, upon which it is replaced. Or the application exits, which is not always desirable.

## REPRODUCE

To reproduce this build and run this project with `cargo run` and click on the window close button to see the issue occur.
