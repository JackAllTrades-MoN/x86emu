/// # JMP - Jump instructions
/// | name      | bin   | mnemonic | clocks    | description       |
/// | -:-       | ---   | ---      | -:-       | ---               |
/// | ShortJump | E8 cb | JMP cb   | 7         |                   |
/// | JumpTG    | EA cd | JMP cd   | 180       | Jump to task gate |
/// | NearJump  | E9 cw | JMP cw   | 7         |                   |
/// | FarJump   | EA cd | JMP cd   | 11, pm=23 | 4byte immd addr   |
/// |

EA cd, JMP cd 38 Jump to call gate, same privilege
EA cd JMP cd 175 Jump via Task State Segment
FF /4 JMP ew 7,mem=11, Jump ne,ar to EA word (absolute offset)
FF /5 JMP ad 15,pm=26 . Jump far (4-byte effective address in memory
doubleword)
FF /5 JMP ad 41 Jump to call gate, same privilege
FF /5 JMP ad 178 Jump via Task State Segment
FF /5 JMP ad 183 Jump to task gate 
