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
