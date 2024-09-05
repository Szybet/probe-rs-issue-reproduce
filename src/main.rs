#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

extern crate alloc;

use core::mem::MaybeUninit;

use defmt::info;
use defmt_rtt as _;
pub use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    prelude::*,
    gpio::Io,
    timer::{timg::TimerGroup, ErasedTimer, OneShotTimer},
};
use static_cell::StaticCell;
use esp_hal::Config;

#[global_allocator]
pub(crate) static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn heap_init() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[main]
async fn main(_spawner: Spawner) {
    info!("Init");

    info!("Heap init");
    heap_init();

    info!("Getting the hardware");

    let conf = Config::default();
    let (peripherals, clocks) = esp_hal::init(conf);

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let timer0: ErasedTimer = timg0.timer0.into();
    let timers = [OneShotTimer::new(timer0)];

    static ONE_SHOT_TIMER: StaticCell<[OneShotTimer<ErasedTimer>; 1]> = StaticCell::new();
    esp_hal_embassy::init(&clocks, ONE_SHOT_TIMER.init(timers));

    info!("Initializing it");
    let _io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    info!("Finished initializing third stage bootloader");

    let mut c = 0;
    loop {
        c += 1;
        info!("Nothing");
        embassy_time::Timer::after_secs(1).await;
        if c > 15 {
            esp_hal::reset::software_reset_cpu();
        }
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!`")
}
