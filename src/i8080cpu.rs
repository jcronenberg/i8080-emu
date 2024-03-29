pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
    pub pad: u8,
}

impl ConditionCodes {
    pub fn new() -> ConditionCodes {
        ConditionCodes {
            z: true,
            s: true,
            p: 1,
            cy: 1,
            ac: 1,
            pad: 3,
        }
    }
}

pub struct State8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: usize,
    pub pc: u16,
    pub memory: [u8; 0x4000],
    pub cc: ConditionCodes,
    pub int_enable: bool,
}

impl State8080 {
    pub fn new() -> State8080 {
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: [0; 0x4000],
            cc: ConditionCodes::new(),
            int_enable: true,
        }
    }
}

fn unimplemented_instruction(state: &State8080) {
    println!("\nInstruction: 0x{:02x}/{}", state.memory[state.pc as usize - 1],
             state.memory[state.pc as usize - 1]);
    println!("PC: {:04x}", state.pc - 1);
    println!("Not implemented yet! Exiting...");
    std::process::exit(1);
}

fn shift_nn(shift1: u8, shift2: u8) -> u16 {
    let mut tmp: u16;
    tmp = (shift1 as u16) << 8;
    tmp |= shift2 as u16;
    return tmp;
}

pub fn emulate_8080_op(state: &mut State8080) {
    if state.pc >= 0x2000 {
        std::process::exit(0);
    }
    let mut opcode: [u8; 3] = [0; 3];
    for i in 0..3 {
        opcode[i] = state.memory[state.pc as usize + i];
        //println!("{}: {}", i, opcode[i]); //debug
    }

    state.pc += 1;

    match opcode[0] {
        0x00 => {}, //NOP
        0x01 => { //LXI B,word
            state.c = opcode[1];
            state.b = opcode[2];
            state.pc += 2;
        },
        0x02 => {unimplemented_instruction(&state)},
        0x03 => {unimplemented_instruction(&state)},
        0x04 => {unimplemented_instruction(&state)},
        0x05 => { //DCR B
            if state.b as i16 - 1 < 0 {
                state.b = 0xff;
            } else {
                state.b -= 1;
            }
            state.cc.z = state.b == 0;
            state.cc.s = 0x80 == (state.b & 0x80);
            //TODO parity flag
            println!("DCR B b: {:02x}, cc.z: {}, cc.s: {}", state.b, state.cc.z, state.cc.s);
        },
        0x06 => { //MVI B,N
            state.b = opcode[1];
            state.pc += 1;
            println!("MVI B b: {:02x}", state.b);
        },
        0x07 => {unimplemented_instruction(&state)},
        0x08 => {unimplemented_instruction(&state)},
        0x09 => {unimplemented_instruction(&state)},
        0x0a => {unimplemented_instruction(&state)},
        0x0b => {unimplemented_instruction(&state)},
        0x0c => {unimplemented_instruction(&state)},
        0x0d => {unimplemented_instruction(&state)},
        0x0e => {unimplemented_instruction(&state)},
        0x0f => {unimplemented_instruction(&state)},

        0x10 => {unimplemented_instruction(&state)},
        0x11 => { //LXI D,NN
            state.d = opcode[2];
            state.e = opcode[1];
            println!("LXI D d: {:02x}, e: {:02x}", state.d, state.e); //debug
            state.pc += 2;
        },
        0x12 => {unimplemented_instruction(&state)},
        0x13 => { //INX D
            let mut de = shift_nn(state.d, state.e);
            de = de.wrapping_add(1);
            state.d = (de >> 8) as u8;
            state.e = (de & 0xff) as u8;
            println!("INX H de: {:04x} d: {:02x}, e: {:02x}", de, state.d, state.e); //debug
        },
        0x14 => {unimplemented_instruction(&state)},
        0x15 => {unimplemented_instruction(&state)},
        0x16 => {unimplemented_instruction(&state)},
        0x17 => {unimplemented_instruction(&state)},
        0x18 => {unimplemented_instruction(&state)},
        0x19 => {unimplemented_instruction(&state)},
        0x1a => { //LDAX D
            state.a = state.memory[shift_nn(state.d, state.e) as usize];
            println!("LDAX D a: {:02x}", state.a); //debug
        },
        0x1b => {unimplemented_instruction(&state)},
        0x1c => {unimplemented_instruction(&state)},
        0x1d => {unimplemented_instruction(&state)},
        0x1e => {unimplemented_instruction(&state)},
        0x1f => {unimplemented_instruction(&state)},

        0x20 => {unimplemented_instruction(&state)},
        0x21 => { //LXI H,NN
            state.h = opcode[2];
            state.l = opcode[1];
            println!("LXI H h: {:02x}, l: {:02x}", state.h, state.l); //debug
            state.pc += 2;
        },
        0x22 => {unimplemented_instruction(&state)},
        0x23 => { //INX H
            let mut hl = shift_nn(state.h, state.l);
            hl = hl.wrapping_add(1);
            state.h = (hl >> 8) as u8;
            state.l = (hl & 0xff) as u8;
            println!("INX H hl: {:04x} h: {:02x}, l: {:02x}", hl, state.h, state.l); //debug
        },
        0x24 => { //INR H
            //TODO flags
            state.h += 1;
        },
        0x25 => {unimplemented_instruction(&state)},
        0x26 => {unimplemented_instruction(&state)},
        0x27 => {unimplemented_instruction(&state)},
        0x28 => {unimplemented_instruction(&state)},
        0x29 => {unimplemented_instruction(&state)},
        0x2a => {unimplemented_instruction(&state)},
        0x2b => {unimplemented_instruction(&state)},
        0x2c => {unimplemented_instruction(&state)},
        0x2d => {unimplemented_instruction(&state)},
        0x2e => {unimplemented_instruction(&state)},
        0x2f => {unimplemented_instruction(&state)},

        0x30 => {unimplemented_instruction(&state)},
        0x31 => { //LXI SP,NN
            state.sp = shift_nn(opcode[2], opcode[1]) as usize;
            println!("sp: {:04x}", state.sp); //debug
            state.pc += 2;
        }, 
        0x32 => {unimplemented_instruction(&state)},
        0x33 => {unimplemented_instruction(&state)},
        0x34 => {unimplemented_instruction(&state)},
        0x35 => {unimplemented_instruction(&state)},
        0x36 => {unimplemented_instruction(&state)},
        0x37 => {unimplemented_instruction(&state)},
        0x38 => {unimplemented_instruction(&state)},
        0x39 => {unimplemented_instruction(&state)},
        0x3a => {unimplemented_instruction(&state)},
        0x3b => {unimplemented_instruction(&state)},
        0x3c => {unimplemented_instruction(&state)},
        0x3d => {unimplemented_instruction(&state)},
        0x3e => {unimplemented_instruction(&state)},
        0x3f => {unimplemented_instruction(&state)},

        0x40 => {unimplemented_instruction(&state)},
        0x41 => { //MOV B,C
            state.b = state.c
        },
        0x42 => { //MOV B,D
            state.b = state.d
        },
        0x43 => { //MOV B,E
            state.b = state.e
        },
        0x44 => {unimplemented_instruction(&state)},
        0x45 => {unimplemented_instruction(&state)},
        0x46 => {unimplemented_instruction(&state)},
        0x47 => {unimplemented_instruction(&state)},
        0x48 => {unimplemented_instruction(&state)},
        0x49 => {unimplemented_instruction(&state)},
        0x4a => {unimplemented_instruction(&state)},
        0x4b => {unimplemented_instruction(&state)},
        0x4c => {unimplemented_instruction(&state)},
        0x4d => {unimplemented_instruction(&state)},
        0x4e => {unimplemented_instruction(&state)},
        0x4f => {unimplemented_instruction(&state)},

        0x50 => {unimplemented_instruction(&state)},
        0x51 => {unimplemented_instruction(&state)},
        0x52 => {unimplemented_instruction(&state)},
        0x53 => {unimplemented_instruction(&state)},
        0x54 => {unimplemented_instruction(&state)},
        0x55 => {unimplemented_instruction(&state)},
        0x56 => {unimplemented_instruction(&state)},
        0x57 => {unimplemented_instruction(&state)},
        0x58 => {unimplemented_instruction(&state)},
        0x59 => {unimplemented_instruction(&state)},
        0x5a => {unimplemented_instruction(&state)},
        0x5b => {unimplemented_instruction(&state)},
        0x5c => {unimplemented_instruction(&state)},
        0x5d => {unimplemented_instruction(&state)},
        0x5e => {unimplemented_instruction(&state)},
        0x5f => {unimplemented_instruction(&state)},

        0x60 => {unimplemented_instruction(&state)},
        0x61 => {unimplemented_instruction(&state)},
        0x62 => {unimplemented_instruction(&state)},
        0x63 => {unimplemented_instruction(&state)},
        0x64 => {unimplemented_instruction(&state)},
        0x65 => {unimplemented_instruction(&state)},
        0x66 => {unimplemented_instruction(&state)},
        0x67 => {unimplemented_instruction(&state)},
        0x68 => {unimplemented_instruction(&state)},
        0x69 => {unimplemented_instruction(&state)},
        0x6a => {unimplemented_instruction(&state)},
        0x6b => {unimplemented_instruction(&state)},
        0x6c => {unimplemented_instruction(&state)},
        0x6d => {unimplemented_instruction(&state)},
        0x6e => {unimplemented_instruction(&state)},
        0x6f => {unimplemented_instruction(&state)},

        0x70 => {unimplemented_instruction(&state)},
        0x71 => {unimplemented_instruction(&state)},
        0x72 => {unimplemented_instruction(&state)},
        0x73 => {unimplemented_instruction(&state)},
        0x74 => {unimplemented_instruction(&state)},
        0x75 => {unimplemented_instruction(&state)},
        0x76 => {unimplemented_instruction(&state)},
        0x77 => { //MOV M,A
            state.memory[shift_nn(state.h, state.l) as usize] = state.a;
            println!("MOV M,A hl: {:04x}, memory: {:02x}", shift_nn(state.h, state.l),
                state.memory[shift_nn(state.h, state.l) as usize]); //debug
        },
        0x78 => {unimplemented_instruction(&state)},
        0x79 => {unimplemented_instruction(&state)},
        0x7a => {unimplemented_instruction(&state)},
        0x7b => {unimplemented_instruction(&state)},
        0x7c => {unimplemented_instruction(&state)},
        0x7d => {unimplemented_instruction(&state)},
        0x7e => {unimplemented_instruction(&state)},
        0x7f => {unimplemented_instruction(&state)},

        0x80 => {unimplemented_instruction(&state)},
        0x81 => {unimplemented_instruction(&state)},
        0x82 => {unimplemented_instruction(&state)},
        0x83 => {unimplemented_instruction(&state)},
        0x84 => {unimplemented_instruction(&state)},
        0x85 => {unimplemented_instruction(&state)},
        0x86 => {unimplemented_instruction(&state)},
        0x87 => {unimplemented_instruction(&state)},
        0x88 => {unimplemented_instruction(&state)},
        0x89 => {unimplemented_instruction(&state)},
        0x8a => {unimplemented_instruction(&state)},
        0x8b => {unimplemented_instruction(&state)},
        0x8c => {unimplemented_instruction(&state)},
        0x8d => {unimplemented_instruction(&state)},
        0x8e => {unimplemented_instruction(&state)},
        0x8f => {unimplemented_instruction(&state)},

        0x90 => {unimplemented_instruction(&state)},
        0x91 => {unimplemented_instruction(&state)},
        0x92 => {unimplemented_instruction(&state)},
        0x93 => {unimplemented_instruction(&state)},
        0x94 => {unimplemented_instruction(&state)},
        0x95 => {unimplemented_instruction(&state)},
        0x96 => {unimplemented_instruction(&state)},
        0x97 => {unimplemented_instruction(&state)},
        0x98 => {unimplemented_instruction(&state)},
        0x99 => {unimplemented_instruction(&state)},
        0x9a => {unimplemented_instruction(&state)},
        0x9b => {unimplemented_instruction(&state)},
        0x9c => {unimplemented_instruction(&state)},
        0x9d => {unimplemented_instruction(&state)},
        0x9e => {unimplemented_instruction(&state)},
        0x9f => {unimplemented_instruction(&state)},

        0xa0 => {unimplemented_instruction(&state)},
        0xa1 => {unimplemented_instruction(&state)},
        0xa2 => {unimplemented_instruction(&state)},
        0xa3 => {unimplemented_instruction(&state)},
        0xa4 => {unimplemented_instruction(&state)},
        0xa5 => {unimplemented_instruction(&state)},
        0xa6 => {unimplemented_instruction(&state)},
        0xa7 => {unimplemented_instruction(&state)},
        0xa8 => {unimplemented_instruction(&state)},
        0xa9 => {unimplemented_instruction(&state)},
        0xaa => {unimplemented_instruction(&state)},
        0xab => {unimplemented_instruction(&state)},
        0xac => {unimplemented_instruction(&state)},
        0xad => {unimplemented_instruction(&state)},
        0xae => {unimplemented_instruction(&state)},
        0xaf => {unimplemented_instruction(&state)},

        0xb0 => {unimplemented_instruction(&state)},
        0xb1 => {unimplemented_instruction(&state)},
        0xb2 => {unimplemented_instruction(&state)},
        0xb3 => {unimplemented_instruction(&state)},
        0xb4 => {unimplemented_instruction(&state)},
        0xb5 => {unimplemented_instruction(&state)},
        0xb6 => {unimplemented_instruction(&state)},
        0xb7 => {unimplemented_instruction(&state)},
        0xb8 => {unimplemented_instruction(&state)},
        0xb9 => {unimplemented_instruction(&state)},
        0xba => {unimplemented_instruction(&state)},
        0xbb => {unimplemented_instruction(&state)},
        0xbc => {unimplemented_instruction(&state)},
        0xbd => {unimplemented_instruction(&state)},
        0xbe => {unimplemented_instruction(&state)},
        0xbf => {unimplemented_instruction(&state)},

        0xc0 => {unimplemented_instruction(&state)},
        0xc1 => {unimplemented_instruction(&state)},
        0xc2 => { //JNZ NN
            if state.cc.z {
                state.pc = shift_nn(opcode[2], opcode[1]);
                println!("jnz pc: {:04x}", state.pc); //debug
            } else {
                println!("jnz skipped!");
            }
        },
        0xc3 => { //JMP adr
            state.pc = shift_nn(opcode[2], opcode[1]);
            println!("jmp pc: {:04x}", state.pc); //debug
        },
        0xc4 => {unimplemented_instruction(&state)},
        0xc5 => {unimplemented_instruction(&state)},
        0xc6 => {unimplemented_instruction(&state)},
        0xc7 => {unimplemented_instruction(&state)},
        0xc8 => {unimplemented_instruction(&state)},
        0xc9 => {unimplemented_instruction(&state)},
        0xca => {unimplemented_instruction(&state)},
        0xcb => {unimplemented_instruction(&state)},
        0xcc => {unimplemented_instruction(&state)},
        0xcd => { //CALL NN
            let tmp: u16 = state.pc + 2;
            state.memory[state.sp - 1] = ((tmp >> 8) & 0xff) as u8;
            state.memory[state.sp - 2] = (tmp & 0xff) as u8;
            state.sp -= 2;
            state.pc = shift_nn(opcode[2], opcode[1]);
            println!("call pc: {:04x}", state.pc); //debug
        },
        0xce => {unimplemented_instruction(&state)},
        0xcf => {unimplemented_instruction(&state)},

        0xd0 => {unimplemented_instruction(&state)},
        0xd1 => {unimplemented_instruction(&state)},
        0xd2 => {unimplemented_instruction(&state)},
        0xd3 => {unimplemented_instruction(&state)},
        0xd4 => {unimplemented_instruction(&state)},
        0xd5 => {unimplemented_instruction(&state)},
        0xd6 => {unimplemented_instruction(&state)},
        0xd7 => {unimplemented_instruction(&state)},
        0xd8 => {unimplemented_instruction(&state)},
        0xd9 => {unimplemented_instruction(&state)},
        0xda => {unimplemented_instruction(&state)},
        0xdb => {unimplemented_instruction(&state)},
        0xdc => {unimplemented_instruction(&state)},
        0xdd => {unimplemented_instruction(&state)},
        0xde => {unimplemented_instruction(&state)},
        0xdf => {unimplemented_instruction(&state)},

        0xe0 => {unimplemented_instruction(&state)},
        0xe1 => {unimplemented_instruction(&state)},
        0xe2 => {unimplemented_instruction(&state)},
        0xe3 => {unimplemented_instruction(&state)},
        0xe4 => {unimplemented_instruction(&state)},
        0xe5 => {unimplemented_instruction(&state)},
        0xe6 => {unimplemented_instruction(&state)},
        0xe7 => {unimplemented_instruction(&state)},
        0xe8 => {unimplemented_instruction(&state)},
        0xe9 => {unimplemented_instruction(&state)},
        0xea => {unimplemented_instruction(&state)},
        0xeb => {unimplemented_instruction(&state)},
        0xec => {unimplemented_instruction(&state)},
        0xed => {unimplemented_instruction(&state)},
        0xee => {unimplemented_instruction(&state)},
        0xef => {unimplemented_instruction(&state)},

        0xf0 => {unimplemented_instruction(&state)},
        0xf1 => {unimplemented_instruction(&state)},
        0xf2 => {unimplemented_instruction(&state)},
        0xf3 => {unimplemented_instruction(&state)},
        0xf4 => {unimplemented_instruction(&state)},
        0xf5 => {unimplemented_instruction(&state)},
        0xf6 => {unimplemented_instruction(&state)},
        0xf7 => {unimplemented_instruction(&state)},
        0xf8 => {unimplemented_instruction(&state)},
        0xf9 => {unimplemented_instruction(&state)},
        0xfa => {unimplemented_instruction(&state)},
        0xfb => {unimplemented_instruction(&state)},
        0xfc => {unimplemented_instruction(&state)},
        0xfd => {unimplemented_instruction(&state)},
        0xfe => {unimplemented_instruction(&state)},
        0xff => {unimplemented_instruction(&state)},
    }
}
