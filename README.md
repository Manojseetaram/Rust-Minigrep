when you want to new file 



$ cargo run > output.txt

Let’s run the program again with arguments that don’t cause an error but still redirect standard output to a file, like so:




$ cargo run -- to poem.txt > output.txt




This chapter recapped some of the major concepts you’ve learned so far and covered how to perform common I/O operations in Rust.
By using command line arguments, files, environment variables, and the eprintln! macro for printing errors, 
you’re now prepared to write command line applications. Combined with the concepts in previous chapters,
your code will be well organized, store data effectively in the appropriate data structures, handle errors nicely, and be well tested.

