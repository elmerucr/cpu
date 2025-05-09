use mos6502::cpu;
use mos6502::instruction::Nmos6502;
use mos6502::memory::Bus;
use mos6502::memory::Memory;

fn main() {
    let zero_page_data = [56, 49];

    let program = [
        0xa5, 0x00,
        0x38,
        0xe5, 0x01,
        0xf0, 0x07,
        0x30, 0x08,
        0x85, 0x00, 0x4c, 0x12, 0x00, 0xa5,
        0x00, 0xff, 0xa6, 0x00, 0xa4, 0x01, 0x86, 0x01, 0x84, 0x00, 0x4c, 0x10, 0x00,
    ];

    let mut cpu = cpu::CPU::new(Memory::new(), Nmos6502);

    cpu.memory.set_bytes(0x00, &zero_page_data);
    cpu.memory.set_bytes(0x10, &program);
    cpu.registers.program_counter = 0x10;
    
    cpu.run();

    assert_eq!(7, cpu.registers.accumulator);

    println!("Hello, world!");
}
