struct cpu_t {
	pc: u16,
	sp: u16,
	a: u8,
	x: u8,
	y: u8,
	p: u8,
}
struct memory_t {
	ram: [u8; 0x800],
	rom: [u8; 0x8000],
}
