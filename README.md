# TMS570 BSP

Bare Metal Board Support Package for Texas Instruments Cortex-R4F/R5F TMS570
MCUs.

TMS570 Transportation MCUs are ARM Cortex-R4F based floating point MCUs that meet IEC61508/SIL3 safety standards. Targeted transportation safety applications include automotive chassis and stability control, electric power steering, hybrid and electric vehicles, aerospace, railway communications, and off-road vehicle engine control.

The TMS570 family integrates dual Cortex-R4F and Cortex-R5F processors in lock-step and is designed to meet automotive and transportation safety standards. These devices provide system-wide protection through seamless support for error detection from the processor, through the bus interconnect, and into the memories.


### Getting started

1. Xargo v0.3.10
2. armeb-none-eabi 7.2.x
3. rust nightly as default toolchain
4. JTAG programmer: Lautherbach Trace32 Powerview for ARM or OpenOCD

### Build

`cd ti-hercules-bsp/boards/ti_tms570; make`
