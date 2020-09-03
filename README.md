<h1>Towers of Hanoi: Auto</h1>
<h2>Automatic Solver with No Recursive Functions</h2>
<p>Towers of Hanoi is a classic puzzle that's fairly easy for a person to solve, but harder to teach to a computer.</p>
<p>With some cleverness, a program can be written in about five lines of code to solve it recursively, but those five lines expand to a ludicrous amount of effective code for larger numbers of discs and I don't like the stack or processing overhead that entails.</p>
<p>As the puzzle is inherently recursive, I don't believe an algorithm can be written to solve it without using some form of recursion (other than simply defining the rules of disc movement and then moving the discs randomly until the puzzle is solved). This solution encapsulates the recursion in a binary counter, rather than interweaving it with the structure of the program, and maps the counter's activity to disc movements.</p>
<p>I did not invent the binary counter solution, but I think I still get partial credit for implementing it. This project also taught me a lot about Rust, which by and large is probably more rewarding than the finished program itself.</p>
