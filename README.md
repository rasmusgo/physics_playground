# Physics playground

1. Install rerun and cargo watch

    cargo install cargo-watch
    cargo install -f rerun-cli

2. Start viewer in one terminal:

    rerun

3. Watch for changes and make a new recording when saving the source code:

    cargo watch -x 'run --release --bin xpbd_particles'

or do this to run clippy first:

    cargo watch -x 'clippy --release' -x 'run --release --bin xpbd_particles'


4. Click play or loop in the viewer to see the simulation.
