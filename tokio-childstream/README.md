Convert a [tokio::process::Child] into a [futures::Stream] which yields
[ChildEvent]s for stdout, stderr, and the exit status.
