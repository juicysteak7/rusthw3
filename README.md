# Rust HW3

Pedro Gonzalez

Professor Bart Massey

CS410 - Rust Programming

# Background

Chomp is a very simple game played with a chocolate bar. This [Wikipedia article](https://en.wikipedia.org/wiki/Chomp)

Links to an external site. describes the rules of the game.

Chomp is a good game to build a computer player for. Chomp is a two-player perfect-information terminating game, so a straightforward implementation of the Negamax algorithm can play perfectly on small boards.

The basic idea is that in a given position a best move can be found by exploring all possible moves, all possible responses, etc. The resulting move selected is one that gives the opponent the worst possible end-of-game result.

In this assignment, you will write a library crate that implements a Chomp AI (src/lib.rs), and a binary crate that allows the game to be played on the command line (src/main.rs).

# Assignment - The Library

The game state should be represented by a struct Board you define. The most important component of the state is the representation of the board itself:

One way: use an array

&nbsp;&nbsp;&nbsp;&nbsp;Use a fixed-size five-by-four (five columns, four rows) array of bools to represent the game state. The element at index i, j should be false if the square at i, j has been eaten, and true otherwise.

&nbsp;&nbsp;&nbsp;&nbsp;Your program should support multiple board sizes. Since the size of a Rust array is fixed at compile time, your board type needs to have fields representing its logical width and height.

Another way: use a set

&nbsp;&nbsp;&nbsp;&nbsp;You can make a HashSet of tuples, where tuple (i, j) being in the set means that position is not yet eaten. You can remove elements from the set as they are eaten.

Your Board type should support the following operations via impl:

&nbsp;&nbsp;&nbsp;&nbsp;Create a board with a given width and height.

&nbsp;&nbsp;&nbsp;&nbsp;Print a graphical representation of a board.

&nbsp;&nbsp;&nbsp;&nbsp;Chomp a given square, removing all squares below it and to the right of it.

&nbsp;&nbsp;&nbsp;&nbsp;Return a winning move for the board, if any (see the next section).

&nbsp;&nbsp;&nbsp;&nbsp;Clone a board (the Clone trait).

# The A.I.

The AI

The negamax algorithm solves any zero-sum perfect-information two-player game (like Chomp). It takes as input a board state and outputs a winning move, if one exists.

    winning-move(posn):
        for each remaining row r
            for each remaining column c in r
                if r = 0 and c = 0
                    continue
                p ← copy of posn
                chomp r, c from p
                m ← winning-move(p)
                if no winning move is returned
                    return the move r, c
       return no winning move

Understand this pseudocode as follows:

&nbsp;&nbsp;&nbsp;&nbsp;Check whether the board state is already lost. If so, then there is no winning move.

&nbsp;&nbsp;&nbsp;&nbsp;Otherwise, for each possible move m:

&nbsp;&nbsp;&nbsp;&nbsp;Create a new board p.

&nbsp;&nbsp;&nbsp;&nbsp;Perform the move m on p.

&nbsp;&nbsp;&nbsp;&nbsp;Call winning_move recursively at p. (Since one player has just made a move, we are now trying to find a winning move for the other player.)

&nbsp;&nbsp;&nbsp;&nbsp;If winning_move outputs a winning move for p, then m is not a winning move for the current player. (Why?) Continue on to the next move.

&nbsp;&nbsp;&nbsp;&nbsp;Otherwise, m is a winning move. Return it.

For Chomp:

&nbsp;&nbsp;&nbsp;&nbsp;The board state is lost if the upper-left square is the only one left.

&nbsp;&nbsp;&nbsp;&nbsp;You can represent the move “chomp at position i, j” by a tuple (i, j). There is one such possible move for each uneaten square (other than the top left one).

# The Program
Your program will be a simple terminal interface for playing Chomp against your AI. It should perform the following sequence:

&nbsp;&nbsp;&nbsp;&nbsp;Ask the user for a board size.

&nbsp;&nbsp;&nbsp;&nbsp;Repeatedly:

&nbsp;&nbsp;&nbsp;&nbsp;Print the board.

&nbsp;&nbsp;&nbsp;&nbsp;Ask the user to input a move and perform it.

&nbsp;&nbsp;&nbsp;&nbsp;Try to find a winning move. If there is one, perform it. 

&nbsp;&nbsp;&nbsp;&nbsp;Otherwise, stall by chomping as little as possible. (You can implement this by chomping the furthest-right piece in the lowermost nonempty row.)

If the user ever gives invalid input, simply ask again until they give valid input.

# Projest Notes

This assignment was similar to one in Bart's A.I. class so understanding the ask wasn't difficult.

The game Chomp was new to me, so learning the rules was where I started.

I elected to use a Hashset of tuples to represent the board, it seemed like an elegant solution and proved to be easy to work with.

I will likely try to use this approach of board representation again.

The wining_move function, and the game loop took the longest to nail down.

You can easily do testing on the library functions but testing user input functions is more challenging.

It tooks some repitition to get the exact logic right and catch some edge cases.

What if the human player chooses to eat the poisened square? - Something I missed initially

The biggest takeaway was the hashset representation of a 2d board. 

I had done arrays or vectors in the past which work well enough to represent the board but can easily become nested loop hell.