#![no_std]
#![no_main]

use embedded_hal::{digital::v2::OutputPin};
use longan_nano::hal::{eclic::
    {EclicExt, Level, LevelPriorityBits, Priority, TriggerType}, 
    gpio::{Output, PushPull, gpioa::PA1, gpioa::PA2}, pac, prelude::*,timer::{Event, Timer}};
use riscv_rt::entry;

use panic_halt as _;

static mut LEDCOLOR: usize = 0;

static mut GREEN : Option<PA1<Output<PushPull>>> = None;
static mut BLUE : Option<PA2<Output<PushPull>>> = None;
static mut TIMER : Option<Timer<longan_nano::hal::pac::TIMER1>> = None;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
        .RCU
        .configure()
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let green = gpioa.pa1.into_push_pull_output();
    let blue = gpioa.pa2.into_push_pull_output();

    longan_nano::hal::pac::ECLIC::reset();
    longan_nano::hal::pac::ECLIC::set_level_priority_bits(LevelPriorityBits::L3P1);
    longan_nano::hal::pac::ECLIC::set_threshold_level(Level::L0);
    longan_nano::hal::pac::ECLIC::setup(pac::Interrupt::TIMER1, TriggerType::Level, Level::L1, Priority::P1);

    let mut timer = Timer::timer1(dp.TIMER1, 1.hz(), &mut rcu);
    timer.listen(Event::Update);

    unsafe
    {
       // RED = Some(red);
        BLUE = Some(blue);
        GREEN = Some(green);
        //RED.as_mut().unwrap().set_high().unwrap();
        GREEN.as_mut().unwrap().set_low().unwrap();
        BLUE.as_mut().unwrap().set_high().unwrap();
        TIMER = Some(timer);
    }
    unsafe{
        longan_nano::hal::pac::ECLIC::unmask(pac::Interrupt::TIMER1);
        riscv::interrupt::enable();
    }
    loop {}
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(unused_assignments)]
#[no_mangle]
fn TIMER1 ()
{
    unsafe
    {
        TIMER.as_mut().unwrap().clear_update_interrupt_flag();       
        
        if  LEDCOLOR == 0
        {
            GREEN.as_mut().unwrap().set_high().unwrap();
            BLUE.as_mut().unwrap().set_low().unwrap();
            LEDCOLOR = 1;
        }
        else if LEDCOLOR == 1
        {
            GREEN.as_mut().unwrap().set_low().unwrap();
            BLUE.as_mut().unwrap().set_high().unwrap();
            LEDCOLOR = 0;
        }
    
}
}