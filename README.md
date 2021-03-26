## To do
* Request to pypi/simple:
  * Process version requirements to generate regex
  * Filter result of call to get list of matching requirements
  * Retrieve the corresponding wheels
  * Unzip and parse metadata file
  * Spawn threads to process everything
* Tree:
  * Create a complex tree to accomodate requirements

## Open questions
* Shall I use `crossbeam` to communicate between threads? How does it work with `tokio`?
* Probs need to use `crossterm` and `clap`
