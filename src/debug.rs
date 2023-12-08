extern crate alloc;

use defmt::println;
use embassy_time::Instant;

#[allow(dead_code)]
pub fn time<BLOCK: FnOnce<()>>(label: &'static str, block: BLOCK) -> BLOCK::Output {
    let ts = Instant::now();
    let ret = block();
    println!("{} in {}us", label, (Instant::now() - ts).as_micros());
    ret
}
