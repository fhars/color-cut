# color-cut

This utility is a line oriented filter that cuts the input lines to a
given length. On pure ASCII text, `color-cut 20` will behave the same
as the standard unix utility cut called with `cut -c 1-20`. Unlike
`cut`, `color-cut` does not assume that every byte in the input is a
character. Instead, it assumes that the input contains valid utf-8
text interspersed with ANSI control sequences. It will pass color
control seqences to the output, and count double width unicode
characters as two columns.

## Installation

This is a very simple rust programm. If you have `cargo` installed,
you can install it locally with
```
cargo install --path .
```
in the root of the cloned repository.

## Motivation

Some time ago, I gave a talk on the datamodel behind `git`, see
https://media.ccc.de/v/DiVOC-5-ein_kurzer_blick_in_git, and I wanted
to display the effects on the content of the `.git` subdirectory while
going through some common sequenences of git commands in a
repository. I was already running tmux in the shell window for the
presentation to be able to script all the shell commands, so I could
just split the window in a large pane für the shell session and a
smaller sidebar displaying something like
`watch -tc tree -C -I"logs|hooks|info" .git`
and I was almost done jusing just some standard tools. The only
problem was that some of the files in that directory have quite long
names and there was some ugly line wrapping. Of course, there is a
default tool for that, too, `cut`. But `cut` works on bytes and knows
nothing about multibyte characters or ANSI color control sequences,
both of which can thoroughly confuse your layout if cut off
improperly, for example it the sequence to reset the colors to default
is just cut off. For that talk I found a line length for the sidebar
where this didn't matter (36, in case you wonder), but such a line
length does not exist in general. So I decided to write a more layout
aware version of `cut`, in case I need it again.

For comparision:
<table>
  <tr>
    <th>cut</th>
    <th>color-cut</th>
  </tr>
  <tr>
    <td>
<pre>$ tree -C .git | cut -c 1-30
<font color="#12488B"><b>.git</b></font>
├── <font color="#12488B"><b>branches</b></font>
<font color="#12488B"><b>OMMIT_EDITMSG</b></font>
<font color="#12488B"><b>├── config</b></font>
<font color="#12488B"><b>├── description</b></font>
<font color="#12488B"><b>├── HEAD</b></font>
<font color="#12488B"><b>├── index</b></font>
<font color="#12488B"><b>├── objects</b></font>
│   ├── <font color="#12488B"><b>2c</b></font>
<font color="#12488B"><b>6</b></font>
<font color="#12488B"><b>│   ├── 33</b></font>
<font color="#12488B"><b>5f</b></font>
<font color="#12488B"><b>│   ├── 7f</b></font>
<font color="#12488B"><b>6</b></font>
<font color="#12488B"><b>│   ├── 8f</b></font>

<font color="#12488B"><b>bc</b></font>
<font color="#12488B"><b>4</b></font>
<font color="#12488B"><b>│   └── pack</b></font>
<font color="#12488B"><b>└── refs</b></font>
    ├── <font color="#12488B"><b>heads</b></font>
<font color="#12488B"><b>aster</b></font>
<font color="#12488B"><b>    └── tags</b></font>
<font color="#12488B"><b>elease-0.1</b></font>

<font color="#12488B"><b>11 directories, 12 files</b></font>
<font color="#12488B"><b>$ tput sgr0</b></font>
$
</pre>
    </td>
    <td>
<pre>$ tree -C .git | color-cut 30
<font color="#12488B"><b>.git</b></font>
├── <font color="#12488B"><b>branches</b></font>
├── COMMIT_EDITMSG
├── config
├── description
├── HEAD
├── index
├── <font color="#12488B"><b>objects</b></font>
│   ├── <font color="#12488B"><b>2c</b></font>
│   │   └── 92f6632693da1df414
│   ├── <font color="#12488B"><b>33</b></font>
│   │   └── 9b5f3603264ee532b6
│   ├── <font color="#12488B"><b>7f</b></font>
│   │   └── 80a63029a034dbe500
│   ├── <font color="#12488B"><b>8f</b></font>
│   │   └── 3761e066d5f3c0d3fa
│   ├── <font color="#12488B"><b>bc</b></font>
│   │   └── 56c4d89448a963d0b6
│   └── <font color="#12488B"><b>pack</b></font>
└── <font color="#12488B"><b>refs</b></font>
    ├── <font color="#12488B"><b>heads</b></font>
    │   └── master
    └── <font color="#12488B"><b>tags</b></font>
        └── release-0.1

11 directories, 12 files
$
</pre>

  </tr>
</table>
