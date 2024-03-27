- March 26th, 2024
    - Brief [tutorial series](https://www.youtube.com/watch?v=qvZGUFHWChY&list=PL9xmBV_5YoZNqDI8qfOZgzbqahCUmUEin&index=1) on RB-tree
    - Red-black tree requirements:
        1. A node is either red or black
        2. Root and leaves (NIL) are black
        3. If a node is red, then its children are black
        4. All paths from a node to its NIL descendants contain the same number of black nodes
    - Nodes:
        - should encode some information about the **key** as well as its **color**
        - longest path is no more than twice the length of the shortest path
            - shortest path have all black nodes
            - longest path alternate between red and black
    - Rotation — larger subtrees up, smaller subtrees down
    - Insertion
        - insert the node and color it red
        - rotate the trees to fix the color violations. 4 different violation scenarios:
            - Z = root. Color it back
            - Z’s uncle = red. Recolor Z’s parent, grandparent, and uncle
            - Z’s uncle = black with triangle (left-right/right-left imbalance) rotate Z’s parent
            - Z’s uncle = black with line (left/right imbalance) rotate Z’s grandparent and recolor them
- March 25th, 2024
    - Data structures for storing text:
        - General collection of different data structures: https://www.cs.unm.edu/~crowley/papers/sds.pdf
        - rope: [https://en.wikipedia.org/wiki/Rope_(data_structure)](https://en.wikipedia.org/wiki/Rope_(data_structure))
        - piece tree: https://code.visualstudio.com/blogs/2018/03/23/text-buffer-reimplementation
        - piece table: [https://en.wikipedia.org/wiki/Piece_table](https://darrenburns.net/posts/piece-table/)
            - contain `original` and `add` buffer
            - use piece descriptors to tell
        - Gap buffer:
            - https://nullprogram.com/blog/2017/09/07/
    
- March 22nd, 2024
    - Control sequence (always start with byte 27)
    `0x1B + "[" + <zero or more numbers, separated by ";"> + <a letter>`
    - might need to implement `editor.resize()` to match the rows and cols with the terminal
- March 21st, 2024
    - Replaced Event with Key and Action
        - `Key` are what the front-end (terminal currently) send back to the editor
        - `Action` are the results of interpreting the `Key`
        
        —> Hopefully this will make it easier to refactor to other fronend later (e.g., website)
        
    - able to detect escape, delete and enter keys
- March 20th, 2024
    - Of course printing control/escape characters to the terminal would not see anything. Should definitely create a **parser** to interpret them and perform specific actions
        - https://notes.burke.libbey.me/ansi-escape-codes/
    - BufRead just has an internal buffer
        - it reads as much as it can with one `syscall`
        - then on subsequent call, it returns the bytes in its internal buffer
        
        —> Avoid making multiple syscalls
        
    - `File::Read` blocks the running thread
        - ideally, spawn another thread to handle input and let the background process run (well…) in the background
        - For now, let’s focusing on parsing and formating the output first
    - Added some enums describing the state/event of the editor:
        - EVENT
        - MODE — similar to Vim
            - NORMAL
            - VISUAL
            - EDIT
        - STATE — the internal state of the editor. Different from MODE in that users can interact with MODE
            - START — currently not being used; should perform some start up task
            - IN_SESSION
            - EXIT — currently not being used; should perform some clean up task
