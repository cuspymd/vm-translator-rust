// lt
  @SP
  M=M-1
  A=M
  D=M
  @SP
  M=M-1
  A=M
  D=M-D
  @ltgt_THEN1
  D;JLT
  D=0
  @ltgt_END1
  0;JMP
(ltgt_THEN1)
  D=-1
(ltgt_END1)
  @SP
  A=M
  M=D
  @SP
  M=M+1
// gt
  @SP
  M=M-1
  A=M
  D=M
  @SP
  M=M-1
  A=M
  D=M-D
  @ltgt_THEN2
  D;JGT
  D=0
  @ltgt_END2
  0;JMP
(ltgt_THEN2)
  D=-1
(ltgt_END2)
  @SP
  A=M
  M=D
  @SP
  M=M+1
