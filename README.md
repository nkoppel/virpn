# Virpn

The Vi-Inspired Reverse Polish Notation calculator.

A calculator designed for efficiency inspired by [vim](https://www.vim.org/), [vim-vertigo](https://github.com/prendradjaja/vim-vertigo), and hp rpn calculators.

## Usage

### Operators

Operators are functions which can modify the stack and hidden state (i.e. variables). When you enter the binding for one, it will immediately run and display the result.

### The Stack and Lists

The stack is where the numbers you are working with go. It is displayed with the top of the stack at the bottom. It can contain numbers and lists also containing lists and numbers. The stack can also move "down" into lists within it and back "up" into containing lists.

### Line Edit Mode

Line edit mode allows you to chain operators together and define lists without changing the stack as you go. To insert operators and numbers, use the same bindings as you would in normal mode. Bindings for line edit mode are [here](#line-editing).

## Bindings

### Basics

    'u'                    = undo last action
    'R'                    = redo last action
    'cc' or 'C'            = clear the stack
    '<space>' or '<enter>' = push number/repeat last action
    '<delete>' or '<esc>'  = clear input line

    "asdfghjkl;"           = insert digit "1234567890", respectively
    'm' or '.'             = insert decimal point
    'n'                    = insert negative number

    'q' or '+'             = add
    'w' or '-'             = subtract
    'e' or '/'             = divide
    'r' or '*'             = multiply
    't' or '^'             = to the power of

Note: Numbers will be pushed onto the stack when entering any operator.

### Stack Manipulation

    'op' = pop top element off of stack
    'od' = duplicate first element on stack
    'ow' = swap first two elements of stack
    'ov' = reverse stack
    'oo' = rotate stack (move bottom element to top)
    'ou' = sum all numbers on the stack
    'om' = multiply all numbers on the stack together


### Functions

    'oq'           = square
    'or'           = square root
    'ob'           = cube root
    'on'           = nth root
    'oe'           = negate

    'oge'          = log base e
    'og1' or 'oga' = log base 10
    'og2' or 'ogs' = log base 2
    'ogg' or 'ogl' = log

    'os'           = sine
    'oc'           = cosine
    'ot'           = tangent
    'oas'          = arc sine
    'oac'          = arc cosine
    'oat'          = arc tangent


### Constants

    'cp'  = pi
    'ce'  = e
    'cq'  = square root of 2
    'cn'  = nan
    'cip' = poitive infinity
    'cin' = negative infinity


### History
    '<up>' / '<down>' = browse previously entered lines
    '<enter>'         = run viewed line


### Lists

    'K' or 'ok' = go "up" into containing list
    'J' or 'oj' = go "down" into contained list or make new list
    'iln'       = insert new list
    'ila'       = given two numbers, make new list containing all numbers from the first to the second, inclusive ("range" operator)
    'ilu'       = sum all numbers in the bottom list
    'ilm'       = multiply all numbers in the bottom list together


### Registers

    'y<letter>' = move the bottom element to register
    'p<letter>' = copy the contents of register to the stack

Note: \<letter\> can be any upper or lower case letter


### Line Editing

    'I'                    = begin line editing
    'ili' or '['           = If not in line edit mode, enter it. Begin writing a new list.
    '<left>'               = move left one operator/number
    '<right>'              = move right one operator/number
    '<space>' or '<enter>' = run current line and exit line edit mode
    '<esc>'                = exit line edit mode without running
    '<backspace>'          = delete previous operator/number
    '<delete>'             = delete current operator/number
