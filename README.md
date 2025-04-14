This repository shows a bug with eframe where the previous window remains (and cannot be closed) until the "run_native" method is called again, upon which it is replaced. Or the application exits, which is not always desirable.

## Reproduce

To reproduce this build and run this project with `cargo run` and click on the window close button to see the issue occur.

### Observed Behaviour

The window remains open until a new eframe instance is launched.

Video Example:
![video](./video_example.webm)

### Expected Behaviour
There should be no eframe window open after it is closed.


### Addition Info
It is not seen in this demonstration, but I know that the application update calls stop after the "x" button is pressed.
