// push local 0
  @LCL
  D=M
  @0
  A=D+A
  D=M
  @SP
  A=M
  M=D
  @SP
  M=M+1
// push static 0
  @Add.0
  D=M
  @SP
  A=M
  M=D
  @SP
  M=M+1
// add
  @SP
  M=M-1
  A=M
  D=M
  @SP
  M=M-1
  A=M
  D=D+M
  @SP
  A=M
  M=D
  @SP
  M=M+1
(END)
  @END
  0;JMP
