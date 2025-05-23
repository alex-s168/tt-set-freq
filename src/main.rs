use luwen_if::{chip::{ArcMsgOptions, Chip, Telemetry}, ArcMsg, CallbackStorage, PowerState, TypedArcMsg};
use luwen_ref::{error::LuwenError, ExtendedPciDevice, PciDevice};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error(transparent)]
    LuwenError(#[from] LuwenError),

    #[error(transparent)]
    PciError(#[from] ttkmd_if::PciError),

    #[error(transparent)]
    LuwenPlatformError(#[from] luwen_if::error::PlatformError),

    #[error("argument has invalid format")]
    ArgNotValidFormat,
}

enum TargetFreq {
    Min,
    Max,
    Current,
    Abs(u32),
}

impl<'a> TryFrom<&'a str> for TargetFreq {
    type Error = AppError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "lo" | "low"  | "min" | "minimum" | "idle" => Ok(TargetFreq::Min),
            "hi" | "high" | "max" | "maximum" | "full" => Ok(TargetFreq::Max),
            "current" => Ok(TargetFreq::Current),
            _ => value.parse()
                .map(|x| TargetFreq::Abs(x))
                .map_err(|_| AppError::ArgNotValidFormat)
        }
    }
}

fn main() -> Result<(), AppError> {
    let mut args = std::env::args().skip(1);
    let target_freq: TargetFreq = args.next().expect("need argument: power state either lo/hi or freq in mhz")
        .as_str()
        .try_into()?;
    let mut target_dev = args.next()
        .map(|x| x.parse().map_err(|_| AppError::ArgNotValidFormat))
        .transpose()?;
    let dump_telem = args.any(|x| x.as_str() == "dump");

    if target_dev.is_none() {
        let found = PciDevice::scan();
        if found.len() > 1 {
            eprintln!("More than one TT device found. Please provide device ID as argument");
            std::process::exit(1);
        }
        target_dev = found.first().map(|x| *x);
    }

    let target_dev = target_dev.expect("no TT device found");

    let dev = ExtendedPciDevice::open(target_dev)?;
    let arch = dev.borrow().device.arch;
    let chip = Chip::open(arch, CallbackStorage::new(luwen_ref::comms_callback, dev))?;

    let telem = chip.inner.get_telemetry()?;

    if dump_telem {
        println!("{:#?}", telem);

        println!("AI_CLK: {}", telem.ai_clk());
        println!("AXI_CLK: {}", telem.axi_clk());
        println!("ARC_CLK: {}", telem.arc_clk());
        println!("ASIC voltage: {}", telem.voltage());
        println!("ASIC temp: {}", telem.asic_temperature());
        println!("voltage regulator temp: {}", telem.vreg_temperature());
        println!("inlinet temp: {}", telem.inlet_temperature());
        println!("outline temp 1: {}", telem.outlet_temperature1());
        println!("outline temp 2: {}", telem.outlet_temperature2());
        println!("power: {}", telem.power());
        println!("current: {}", telem.current());
    }

    println!("\nfound {arch} at PCI device {target_dev}");
    let current_clock = telem.ai_clk();
    println!("current clock speed: {current_clock} MHz");

    let msg = match target_freq {
        TargetFreq::Min => Some(ArcMsg::Typed(TypedArcMsg::SetPowerState(PowerState::LongIdle))),
        TargetFreq::Max => Some(ArcMsg::Typed(TypedArcMsg::SetPowerState(PowerState::Busy))),
        TargetFreq::Abs(arg) => Some(ArcMsg::Raw {
            msg: 0x33,
            arg0: (arg & 0xFFFF) as u16,
            arg1: ((arg >> 16) & 0xFFFF) as u16
        }),
        TargetFreq::Current => None
    };

    if let Some(msg) = msg {
        chip.inner.arc_msg(ArcMsgOptions {
            msg,
            ..Default::default()
        })?;
    }

    let current_clock = chip.inner.get_telemetry()?.ai_clk();
    println!("new actual clock speed: {current_clock} MHz");

    Ok(())
}
