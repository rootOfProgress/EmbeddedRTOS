# NextLevelRTOS

[![Cortex M Types](https://img.shields.io/badge/target-thumbv7em--none--eabihf-green)](https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/)

[![Status](https://img.shields.io/badge/Status-W.I.P-red)]()


## About
This is a minimal multithreaded realtime operating system for the ARM CortexM4 processor without unnecessary libraries. The final goal is to compile your own
OS with only the features you need - no boilerplate code, no libraries which inflate your binary without knowing exactly what they actually do. 
Currently this system is developed with a STM32F303 Discovery board as developing platform, but in future it could be possible to be platform independet by feeding the kernel with provided platform / register informations, as long as the cpu is a cortex m4.
