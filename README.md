# PoTsai
A DoS attacker inspired by Hulk.

# Usage
Just run `cargo run` and it'll start.

Enter a URL to attack and the amount of requests to make.

There is a good chance you'll get tons of errors with threads (I'm working on it)

# How this works
It runs a loop the amount you specify and runs a new thread for each iteration. Each thread goes up and makes a request, closes the thread, and effectively crashes small servers.
