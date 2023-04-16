Convert a [tokio::process::Child] into a [futures::Stream] which yields
[ChildItem]s for stdout, stderr, and the exit status.
