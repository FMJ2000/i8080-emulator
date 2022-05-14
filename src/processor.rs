#![allow(arithmetic_overflow)]

const RAM: usize = 0x2000;

// SZ0A0P1C
#[derive(Debug, Clone, Copy)]
struct Flags {
  s: bool,
  z: bool,
  ac: bool,
  p: bool,
  cy: bool,
}

impl Flags {
  fn new() -> Flags {
    Flags { s: false, z: false, ac: false, p: false, cy: false }
  }
}

#[derive(Debug)]
pub struct State {
  ic: u128,
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  sp: usize,
  pc: usize,
  cc: Flags,
  ie: bool,
  mem: Vec<u8>,
}

impl State {
  pub fn new(mem: Vec<u8>) -> State {
    State { ic: 0, a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0, cc: Flags::new(), ie: false, mem }
  }
  
  /// execute single command
  pub fn exec(&mut self) {
    self.ic += 1;
    let opcode = self.mem[self.pc];
    let d = (opcode >> 3) & 0b111;
    let s = opcode & 0b111;
    let rp = (opcode >> 4) & 0b11;
    let hblb = if self.pc + 2 < self.mem.len() {
      (self.mem[self.pc+2] as u16) << 8 | self.mem[self.pc+1] as u16
    } else {
      0
    };
    
    println!("op: {:04X}", opcode);

    match opcode {
      0x01 | 0x11 | 0x21 | 0x31 => {
        self.set_reg_pair(rp, hblb, false);
        self.pc += 2;
      },
      0x02 | 0x12 => {
        let db = self.get_reg_pair(rp, false) as usize;
        self.mem[db] = self.a;
      },
      0x03 | 0x13 | 0x23 | 0x33 => self.set_reg_pair(rp, self.get_reg_pair(rp, false) + 1, false),
      0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
        self.set_reg(d, self.get_reg(d) + 1);
        self.cc(self.get_reg(d));
      },
      0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
        self.set_reg(d, self.get_reg(d) + (!1 + 1));
        self.cc(self.get_reg(d));
      },
      0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
        self.set_reg(d, self.mem[self.pc+1]);
        self.pc += 1;
      }
      0x07 | 0x17 => self.rl(opcode == 0x17),
      0x09 | 0x19 | 0x29 | 0x39 => self.dad(rp),
      0x0A | 0x1A => self.a = self.mem[self.get_reg_pair(rp, false) as usize],
      0x0B | 0x1B | 0x2B | 0x3B => self.set_reg_pair(rp, self.get_reg_pair(rp, false) - 1, false),
      0x0F | 0x1F => self.rr(opcode == 0x1F),
      0x2F => self.a = !self.a,
      0x32 => {
        self.mem[hblb as usize] = self.a;
        self.pc += 2;
      },
      0x37 => self.cc.cy = true,
      0x3A => {
        self.a = self.mem[hblb as usize];
        self.pc += 2;
      },
      0x3F => self.cc.cy = !self.cc.cy,
      0x41..=0x75 | 0x77..=0x7F => self.set_reg(d, self.get_reg(s)),
      0x76 => return,
      0x80..=0x8F => self.add(self.get_reg(s), opcode >= 0x88),
      0x90..=0x9F => self.sub(self.get_reg(s), opcode >= 0x98),
      0xA0..=0xA7 => self.and(self.get_reg(s)),
      0xA8..=0xAF => self.xor(self.get_reg(s)),
      0xB0..=0xB7 => self.or(self.get_reg(s)),
      0xB8..=0xBF => self.cmp(self.get_reg(s)),
      0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => {
        if self.get_ccc(d) {
          self.pc = (self.pop() as usize) - 1;
        }
      },
      0xC1 | 0xD1 | 0xE1 | 0xF1 => {
        let db = self.pop();
        self.set_reg_pair(rp, db, true)
      },
      0xC2 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => {
        if self.get_ccc(d) {
          self.pc = (hblb as usize) - 1;
        } else {
          self.pc += 2;
        }
      },
      0xC3 => self.pc = (hblb as usize) - 1,
      0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => {
        if self.get_ccc(d) {
          self.push((self.pc + 2) as u16);
          self.pc = (hblb as usize) - 1;
        } else {
          self.pc += 2;
        }
      },
      0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push(self.get_reg_pair(rp, true)),
      0xC6 | 0xCE => {
        self.add(self.mem[self.pc+1], opcode == 0xCE);
        self.pc += 1;
      },
      0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
        self.push((self.pc + 2) as u16);
        self.pc = (d as usize) - 1;
      },
      0xC9 => self.pc = (self.pop() as usize) - 1,
      0xCD => {
        self.push((self.pc + 2) as u16);
        self.pc = (hblb as usize) - 1;
      },
      0xD3 | 0xDB => self.pc += 1,
      0xD6 => {
        self.sub(self.mem[self.pc+1], opcode == 0xDE);
        self.pc += 1;
      },
      0xE3 => {
        let (hb, lb) = (self.mem[self.sp+1], self.mem[self.sp]);
        self.mem[self.sp + 1] = self.h;
        self.mem[self.sp] = self.l;
        self.h = hb;
        self.l = lb;
      },
      0xE6 => {
        self.and(self.mem[self.pc+1]);
        self.pc += 1;
      },
      0xE9 => self.pc = (self.get_reg_pair(0x2, false) as usize) - 1,
      0xEB => {
        let (hb, lb) = (self.d, self.e);
        self.d = self.h;
        self.e = self.l;
        self.h = hb;
        self.l = lb;
      }
      0xEE => {
        self.xor(self.mem[self.pc+1]);
        self.pc += 1;
      },
      0xF3 | 0xFB => self.ie = opcode == 0xFB,
      0xF6 => {
        self.or(self.mem[self.pc+1]);
        self.pc += 1;
      },
      0xF9 => self.sp = self.get_reg_pair(0x02, false) as usize,
      0xFE => {
        self.cmp(self.mem[self.pc+1]);
        self.pc += 1;
      },
      _ => (),
    }
    self.pc += 1;
  }

  fn get_reg(&self, s: u8) -> u8 {
    match s & 0b111 {
      0x0 => self.b,
      0x1 => self.c,
      0x2 => self.d,
      0x3 => self.e,
      0x4 => self.h,
      0x5 => self.l,
      0x6 => self.mem[((self.h as usize) << 8) | self.l as usize],
      _ => self.a,
    }
  }

  fn set_reg(&mut self, d: u8, db: u8) {
    match d & 0b111 {
      0x0 => self.b = db,
      0x1 => self.c = db,
      0x2 => self.d = db,
      0x3 => self.e = db,
      0x4 => self.h = db,
      0x5 => self.l = db,
      0x6 => self.mem[((self.h as usize) << 8) | self.l as usize] = db,
      _ => self.a = db,
    }
  }

  fn get_reg_pair(&self, rp: u8, psw: bool) -> u16 {
    match rp & 0b11 {
      0x0 => (self.b as u16) << 8 | self.c as u16,
      0x1 => (self.d as u16) << 8 | self.e as u16,
      0x2 => (self.h as u16) << 8 | self.l as u16,
      _ => if psw {
        (self.a as u16) << 8 |
          (self.cc.ac as u16) << 4 |
          (self.cc.cy as u16) << 3 |
          (self.cc.p as u16) << 2 |
          (self.cc.s as u16) << 1 |
          self.cc.z as u16
      } else {
        self.sp as u16
      },
    }
  }

  fn set_reg_pair(&mut self, rp: u8, db: u16, psw: bool) {
    match rp & 0b11 {
      0x0 => {
        self.b = (db >> 8) as u8;
        self.c = db as u8;
      },
      0x1 => {
        self.d = (db >> 8) as u8;
        self.e = db as u8;
      },
      0x2 => {
        self.h = (db >> 8) as u8;
        self.l = db as u8;
      },
      _ => if psw {
        self.a = (db >> 8) as u8;
        self.cc.ac = (db & 0x10) != 0;
        self.cc.cy = (db & 0x08) != 0;
        self.cc.p = (db & 0x04) != 0;
        self.cc.s = (db & 0x02) != 0;
        self.cc.z = (db & 0x01) != 0;
      } else {  
        self.sp = db as usize
      },
    }
  }

  fn get_ccc(&self, d: u8) -> bool {
    match d & 0b111 {
      0x0 => !self.cc.z,
      0x1 => self.cc.z,
      0x2 => !self.cc.cy,
      0x3 => self.cc.cy,
      0x4 => !self.cc.p,
      0x5 => self.cc.p,
      0x6 => !self.cc.s,
      _ => self.cc.s,
    }
  }

  fn get_opcode(&self, index: usize) -> &'static str {
    match self.mem[index] {
      0x01	=> "LXI B,D16",
      0x05	=> "DCR B",
      0x06	=> "MVI B,D8",
      0x09	=> "DAD B",
      0x0d	=> "DCR C",
      0x0e	=> "MVI C,D8",
      0x0f	=> "RRC",
      0x11	=> "LXI D,D16",
      0x13	=> "INX D",
      0x19	=> "DAD D",
      0x1a	=> "LDAX D",
      0x21	=> "LXI H,D16",
      0x23	=> "INX H",
      0x26	=> "MVI H,D8",
      0x29	=> "DAD H",
      0x31	=> "LXI SP,D16",
      0x32	=> "STA adr",
      0x36	=> "MVI M,D8",
      0x3a	=> "LDA adr",
      0x3e	=> "MVI A,D8",
      0x56	=> "MOV D,M",
      0x5e	=> "MOV E,M",
      0x66	=> "MOV H,M",
      0x6f	=> "MOV L,A",
      0x77	=> "MOV M,A",
      0x7a	=> "MOV A,D",
      0x7b	=> "MOV A,E",
      0x7c	=> "MOV A,H",
      0x7e	=> "MOV A,M",
      0xa7	=> "ANA A",
      0xaf	=> "XRA A",
      0xc1	=> "POP B",
      0xc2	=> "JNZ adr",
      0xc3	=> "JMP adr",
      0xc5	=> "PUSH B",
      0xc6	=> "ADI D8",
      0xc9	=> "RET",
      0xcd	=> "CALL adr",
      0xd1	=> "POP D",
      0xd3	=> "OUT D8",
      0xd5	=> "PUSH D",
      0xe1	=> "POP H",
      0xe5	=> "PUSH H",
      0xe6	=> "ANI D8",
      0xeb	=> "XCHG",
      0xf1	=> "POP PSW",
      0xf5	=> "PUSH PSW",
      0xfb	=> "EI",
      0xfe	=> "CPI D8",
      _ => "NOP",
    }
  }

  /// And value with A
  fn and(&mut self, db: u8) {
    self.a &= db;
    self.cc.cy = false;
    self.cc(self.a);
  }

  /// Add value to A
  fn add(&mut self, db: u8, c: bool) {
    let mut ans = self.a as u16 + db as u16;
    if c {
      ans += self.cc.cy as u16;
    }
    self.cc.cy = ans > 0xFF;
    self.a = ans as u8;
    self.cc(self.a);
  }

  /// Change control flags
  fn cc(&mut self, ans: u8) {
    self.cc.s = (ans & 0x80) != 0;
    self.cc.z = ans == 0;
    self.cc.p = ans % 2 == 0;
  }

  /// Compare value with A
  fn cmp(&mut self, db: u8) {
    let ans = self.a + (!db + 1);
    self.cc(ans);
    self.cc.cy = self.a < db;
  }

  /// Double add
  fn dad(&mut self, rp: u8) {
    let ans = self.get_reg_pair(0x2, false) as u32 + self.get_reg_pair(rp, false) as u32;
    self.cc.cy = ans > 0xFFFF;
    self.set_reg_pair(0x2, ans as u16, false);
  }

  /// Or value with A
  fn or(&mut self, db: u8) {
    self.a |= db;
    self.cc.cy = false;
    self.cc(self.a);
  }

  fn pop(&mut self) -> u16 {
    let ret = (self.mem[self.sp + 1] as u16) << 8 | self.mem[self.sp] as u16;
    self.sp += 2;
    ret
  }

  pub fn print(&self) {
    let print_state = [
      format!("A:\t{:02X}\t\tS:\t{}\tMEM:\t", self.a, self.cc.s as u8),
      format!("BC:\t{:02X} {:02X}\t\tZ:\t{}\t\t", self.b, self.c, self.cc.z as u8),
      format!("DE:\t{:02X} {:02X}\t\tAC:\t{}\t\t", self.d, self.e, self.cc.ac as u8),
      format!("HL:\t{:02X} {:02X}\t\tP:\t{}\t\t", self.h, self.l, self.cc.p as u8),
      format!("SP:\t{:04X}\t\tCY:\t{}\t\t", self.sp, self.cc.cy as u8),
      format!("\t{:02X} {:02X}\t\tIC:\t{}\t\t", self.mem[self.sp + 1], self.mem[self.sp], self.ic),
    ];

    for i in 0..print_state.len() {
      let anno = if i == 0 { "PC:\t" } else if i == 2 { "->\t" } else { "\t" };
      let opcode = if (self.pc + i >= 2) && (self.pc + i + print_state.len() < self.mem.len() - 2) {
        let index = self.pc + i - 2;
        format!("{:04X} | {:02X} {}", index, self.mem[index], self.get_opcode(index))
      } else {
        String::new()
      };
      println!("{}{:04X} | {:02X}\t{}{}", print_state[i], RAM+i, self.mem[RAM+i], anno, opcode);
    }
  }

  /// Push 16-bit data onto stack
  fn push(&mut self, hblb: u16) {
    self.mem[self.sp-1] = (hblb >> 8) as u8;
    self.mem[self.sp-2] = hblb as u8;
    self.sp -= 2;
  }

  fn rl(&mut self, c: bool) {
    let hb = self.a >> 7 == 1;
    self.a = if c {
      self.a << 1 | self.cc.cy as u8
    } else {
      self.a << 1 | self.a >> 7
    };
    self.cc.cy = hb;
  }

  fn rr(&mut self, c: bool) {
    let lb = (self.a & 0x1) == 1;
    self.a = if c {
      (self.cc.cy as u8) << 7 | self.a >> 1
    } else {
      self.a << 7 | self.a >> 1
    };
    self.cc.cy = lb;
  }

  /// Subtract value from A
  fn sub(&mut self, mut db: u8, c: bool) {
    if c {
      db += self.cc.cy as u8;
    }
    let ans = self.a + (!db + 1);
    self.cc(ans);
    self.cc.cy = self.a < db;
    self.a = ans;
  }

  /// Xor value with A
  fn xor(&mut self, db: u8) {
    self.a ^= db;
    self.cc.cy = false;
    self.cc(self.a);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let a = 200;
    let b = 88;
    let c = (a as u16 + b as u16) as u8;
    let mut state = State::new(0);
    state.a = a;
    state.add(b, false);
    //println!("{:#?}", state);
    assert_eq!(state.a, c);
  }

  #[test]
  fn test_adc() {
    let a = 200;
    let b = 88;
    let c = (a as u16 + b as u16 + 1) as u8;
    let mut state = State::new(vec![]);
    state.a = a;
    state.cc.cy = true;
    state.add(b, true);
    //println!("{:#?}", state);
    assert_eq!(state.a, c);
  }

  #[test]
  fn test_sub() {
    let a: i8 = 20;
    let b: i8 = 88;
    let c = a - b;
    let mut state = State::new(vec![]);
    state.a = a as u8;
    state.add((!b + 1) as u8, false);
    println!("{:#?}", state);
    assert_eq!(state.a, c as u8);
  }

  #[test]
  fn test_sbb() {
    let a: i8 = 20;
    let b: i8 = 88;
    let c = a - b;
    let mut state = State::new(vec![]);
    state.a = a as u8;
    state.cc.cy = true;
    state.add((!b + 1) as u8, true);
    println!("{:#?}", state);
    assert_eq!(state.a, c as u8);
  }
}