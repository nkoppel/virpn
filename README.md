# Virpn

The Vi-Inspired Reverse Polish Notation calculator.

A calculator designed for efficiency inspired by [vim](https://www.vim.org/), [vim-vertigo](https://github.com/prendradjaja/vim-vertigo), and hp rpn calculators.

A web build of virpn can be found [here](https://nkoppel.github.io/virpn).

## Features

- Rpn calculations on an infinite stack
- [Vertigo-like](https://github.com/prendradjaja/vim-vertigo) number input
- Lists
- 52 variables
- Undo and redo
- Polynomial and Calculus operations

## Concepts

### Operators

Operators are functions which can modify the stack and hidden state (i.e. variables). When you enter the binding for one, it will immediately run and display the result.

### The Stack and Lists

The stack is where the numbers you are working with go. It is displayed with the top of the stack at the bottom. It can contain numbers and lists also containing lists and numbers. The stack can also move "down" into lists within it and back "up" into containing lists.

#### Operators and Lists
When an operator which takes one number is applied to a list, it applies to each number recursively. i.e.

    [2 [3 4]] square  = [4 [9 16]]

When an operator which takes two numbers is applied to a list and a number, the number is used recursively. i.e.

    [1 [2 3]]  1 +  = [2 [3 4]]

When an operator which takes two numbers is applied to two lists, the elements from each list are paired up until one list runs out of elements. i.e.

    [1 2 3]  [1 2] +  = [2 4]

The previous two rules are also applied recursively. i.e.

    [[1 2] [3 4] 5]  [1 [2]] +  = [[2 3] [5]]

### Line Edit Mode

Line edit mode allows you to chain operators together and define lists without changing the stack as you go. To insert operators and numbers, use the same bindings as you would in normal mode. Bindings for line edit mode are [here](#line-editing).

## Bindings

### Basics

    'u'                    = undo last action
    'R'                    = redo last action
    'C', 'cc', or 'isc'    = clear the stack
    '<space>' or '<enter>' = push number/repeat last action
    '<delete>' or '<esc>'  = clear input line
    'Q'                    = quit

### Numbers

    "asdfghjkl;" = insert digit "1234567890", respectively
    'm' or '.'   = insert decimal point
    'n'          = insert negative number

Note: Numbers will be pushed onto the stack when entering any operator.

### Basic Operators

    'q' or '+'       = add
    'w' or '-'       = subtract
    'e' or '/'       = divide
    'r' or '*'       = multiply
    't' or '^'       = to the power of
    'D', 'E', or '%' = modulus

Note: The locations of the letters for addition, subtraction, and division are based off of the keys below them.

### Stack Manipulation

    'op' or 'isp' = pop top element off of stack
    'od' or 'isd' = duplicate first element on stack
    'ow' or 'isw' = swap first two elements of stack
    'ov' or 'isv' = reverse stack
    'oo' or 'iso' = rotate stack (move bottom element to top)

    'ou' or 'isu' = sum all numbers on the stack
    'om' or 'ism' = multiply all numbers on the stack together

    'y' = copy the bottom element to the aux stack
    'Y' = move the bottom element to the aux stack
    'p' = copy the bottom element of the aux stack to the stack
    'P' = move the bottom element of the aux stack to the stack


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

    'ior'          = round a number to the nearest integer
    'iof'          = floor a number
    'ioc'          = ceil a number
    'iodr'         = round the first number to the second number of digits after the decimal point
    'ioe' or 'ol'  = fix small floating point errors


### Constants

    'cp'  = pi
    'ce'  = e
    'cq'  = square root of 2
    'cs'  = distance between 1 and the next representable number
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
    'ilc'       = add each item in the bottom list to all items before it
    'ilv'       = reverse the items in the bottom list
    'ill'       = get the length of the bottom list
    'ilt'       = mirror a 2d list diagonally


### Polynomials

    'ipp' or 'ilp' = plug numbers in the first argument into the polynomial in the second argument
    'ips'          = synthetic divide the polynomial in the second argument by the first argument
    'ipd' or 'ipe' = divide the first polynomial by the second
    'ipm' or 'ipr' = multiply the first polynomial with the second
    'ipq'          = square a polynomial


### Registers

    'z<letter>' = copy the bottom element to register
    'v<letter>' = copy the contents of register to the stack

Note: \<letter\> can be any upper or lower case letter


### Functions

    'ifi' or '(' = write a new function in line edit mode

    'ifr'        = pop a function and run it on the current stack
    'iftr'       = pop a number and a function and run the function that number of times
    'ifq'        = run a function on the second argument the third argument number of times, recording each result in a list

    'ifm'        = apply the function to all items in the current stack
    'ifnm'       = apply the function to all numbers in a nested list

    'iff'        = fold the function across all items in the current stack
    'ifnf'       = fold the function across all numbers in a nested list

    'irx'        = find the maxumum near the second argument, with a starting interval size of the third argument
    'irn'        = find the minimum near the second argument, with a starting interval size of the third argument
    'irz'        = find a zero between the bounds given in the second and third arguments


### Line Editing

    'I'                    = begin line editing
    'u'                    = undo the previous action
    'ili' or '['           = If not in line edit mode, enter it. Begin writing a new list.
    ']'                    = Insert ']'
    'ifi' or '('           = If not in line edit mode, enter it. Begin writing a new function.
    ')'                    = Insert ')'
    '<left>'               = move left one operator/number
    '<right>'              = move right one operator/number
    '<space>' or '<enter>' = run current line and exit line edit mode
    '<esc>'                = exit line edit mode without running
    '<backspace>'          = delete previous operator/number
    '<delete>'             = delete current operator/number
