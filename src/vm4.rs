//! Note: this does not manage `ip` the same way as the article, and the `ip` could really be
//! dropped entirely.

const REGISTER_NUM: usize = 16;

#[derive(Default)]
pub struct Vm {
    ip: usize,

    reg: [u64; REGISTER_NUM],

    result: u64,
}

#[derive(Debug, PartialEq, Eq, ToPrimitive, FromPrimitive)]
pub enum Op {
    Done = 0,
    LoadI = 1,
    Add = 2,
    Sub = 3,
    Div = 4,
    Mul = 5,
    MovRes = 6,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode,
    DivByZero,
}

struct Inst {
    c: [u8; 2],
}

impl Inst {
    fn from_slice(c: [u8; 2]) -> Inst {
        Self { c }
    }

    fn op(&self) -> u8 {
        (self.c[0] & 0xF0) >> 4
    }

    fn reg0(&self) -> usize {
        (self.c[0] & 0x0F) as usize
    }

    fn reg1(&self) -> usize {
        ((self.c[1] & 0xF0) >> 4) as usize
    }

    fn reg2(&self) -> usize {
        (self.c[1] & 0x0F) as usize
    }

    fn imm(&self) -> u64 {
        self.c[1] as u64
    }
}

#[macro_export]
macro_rules! vm4_asm {
    ( @a [ $($n:tt)* ] -> DONE ; $($e:tt)* ) => {
        vm4_asm!(@a
            [ $($n)*
                (Op::Done.to_u8().unwrap() << 4),
                0 ,
            ] -> $($e)*
        )
    };
    ( @a [$($n:tt)*] -> LOADI $r0:expr , $imm:expr ; $($e:tt)* ) => {
        vm4_asm!(@a
            [ $($n)*
                (Op::LoadI.to_u8().unwrap() << 4 | $r0),
                $imm,
            ] -> $($e)*
        )
    };
    ( @a [$($n:tt)*] -> ADD $r0:expr , $r1:expr , $r2:expr ; $($e:tt)* ) => {
        vm4_asm!(@a
            [ $($n)*
                (Op::Add.to_u8().unwrap() << 4 | $r0),
                ($r1 << 4 | $r2),
            ] -> $($e)*
        )
    };
    ( @a [$($n:tt)*] -> SUB $r0:expr , $r1:expr , $r2:expr ; $($e:tt)* ) => {
        vm4_asm!(@a
            [ $($n)*
                (Op::Sub.to_u8().unwrap() << 4 | $r0),
                ($r1 << 4 | $r2),
            ] -> $($e)*
        )
    };
    ( @a [$($n:tt)*] -> DIV $r0:expr , $r1:expr , $r2:expr ; $($e:tt)* ) => {
        vm4_asm!(@a
            [ $($n)*
                (Op::Div.to_u8().unwrap() << 4 | $r0),
                ($r1 << 4 | $r2),
            ] -> $($e)*
        )
    };
    ( @a [$($n:tt)*] -> MUL $r0:expr , $r1:expr , $r2:expr ; $($e:tt)* ) => {
        vm4_asm!(@a
            [ $($n)*
                (Op::Mul.to_u8().unwrap() << 4 | $r0),
                ($r1 << 4 | $r2),
            ] -> $($e)*
        )
    };
    ( @a [$($n:tt)*] -> MOVRES $r0:expr ; $($e:tt)* ) => {
        vm4_asm!(@a
            [ $($n)*
                (Op::MovRes.to_u8().unwrap() << 4 | $r0),
                0 ,
            ] -> $($e)*
        )
    };
    ( @a [$($n:tt)*] -> ) => {
        [ $($n)* ]
    };
    ( $($n:tt)* ) => {
        vm4_asm!(@a [] -> $($n)*)
    };
}

impl Vm {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    fn next_i(&mut self, bytecode: &mut &[u8]) -> Option<u8> {
        if bytecode.is_empty() {
            None
        } else {
            let a = bytecode[0];
            *bytecode = &bytecode[1..];
            self.ip += 1;
            Some(a)
        }
    }

    pub fn interp(&mut self, mut bytecode: &[u8]) -> Result<(), Error> {
        self.reset();

        loop {
            let ins1 = self.next_i(&mut bytecode).unwrap();
            let ins2 = self.next_i(&mut bytecode).unwrap();

            let insv = Inst::from_slice([ins1, ins2]);

            match num::FromPrimitive::from_u8(insv.op()) {
                Some(Op::LoadI) => {
                    self.reg[insv.reg0()] = insv.imm();
                }
                Some(Op::Add) => {
                    self.reg[insv.reg2()] = self.reg[insv.reg0()] + self.reg[insv.reg1()];
                }
                Some(Op::Sub) => {
                    self.reg[insv.reg2()] = self.reg[insv.reg0()] - self.reg[insv.reg1()];
                }
                Some(Op::Div) => {
                    let r1 = self.reg[insv.reg1()];
                    if r1 == 0 {
                        return Err(Error::DivByZero);
                    }

                    self.reg[insv.reg2()] = self.reg[insv.reg0()] / r1;
                }
                Some(Op::Mul) => {
                    self.reg[insv.reg2()] = self.reg[insv.reg0()] * self.reg[insv.reg1()];
                }
                Some(Op::MovRes) => {
                    self.result = self.reg[insv.reg0()];
                }
                Some(Op::Done) => break,
                None => return Err(Error::UnknownOpcode),
            }
        }

        Ok(())
    }
}

#[test]
fn t1() {
    use num::ToPrimitive;
    let mut vm = Vm::default();

    assert_eq!(
        vm.interp(
            &vm4_asm!(
                LOADI 0, 3;
                LOADI 1, 5;
                ADD 0, 1, 2;
                LOADI 3, 6;
                MUL 2, 3, 4;
                DIV 4, 1, 5;
                SUB 5, 0, 6;
                MOVRES 6;
                DONE;
            )[..]
        ),
        Ok(())
    );

    assert_eq!(vm.result, ((3 + 5) * 6) / 5 - 3);
}
